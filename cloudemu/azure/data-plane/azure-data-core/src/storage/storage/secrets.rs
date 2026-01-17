use uuid::Uuid;
use chrono::Utc;
use super::engine::{StorageEngine, SecretMetadata, SecretValue};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== Secrets Operations ====================

    /// Create a secret
    pub fn create_secret(&self, name: &str, description: Option<&str>, tags: Option<&str>, account_id: &str, region: &str) -> Result<SecretMetadata> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        let arn = format!("arn:aws:secretsmanager:{}:{}:secret:{}", region, account_id, name);

        db.execute(
            "INSERT INTO secrets (arn, name, description, created_at, last_changed_date, tags) VALUES (?1, ?2, ?3, ?4, ?4, ?5)",
            params![arn, name, description, now, tags],
        ).map_err(|e| {
             if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Secret {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(SecretMetadata {
            arn,
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            created_at: now.clone(),
            last_changed_date: Some(now),
            tags: tags.map(|s| s.to_string()),
        })
    }
    
    /// Put secret value
    pub fn put_secret_value(&self, secret_id: &str, secret_string: Option<&str>, secret_binary: Option<&[u8]>) -> Result<(String, String)> {
        let db = self.db.lock();
        
        let arn: String = db.query_row(
            "SELECT arn FROM secrets WHERE name = ?1 OR arn = ?1",
            params![secret_id],
            |row| row.get(0),
        ).map_err(|_| EmulatorError::NotFound("Secret".into(), secret_id.into()))?;
        
        let version_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let stages = "[\"AWSCURRENT\"]";
        
        db.execute(
            "UPDATE secret_versions SET version_stages = '[]' WHERE secret_arn = ?1",
            params![arn],
        )?;

        db.execute(
            "INSERT INTO secret_versions (secret_arn, version_id, version_stages, secret_string, secret_binary, created_date) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![arn, version_id, stages, secret_string, secret_binary, now],
        )?;
        
        Ok((arn, version_id))
    }

    /// Get secret value
    pub fn get_secret_value(&self, secret_id: &str, version_id: Option<&str>, _version_stage: Option<&str>) -> Result<SecretValue> {
        let db = self.db.lock();
        let (arn, name): (String, String) = db.query_row(
            "SELECT arn, name FROM secrets WHERE name = ?1 OR arn = ?1",
            params![secret_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ).map_err(|_| EmulatorError::NotFound("Secret".into(), secret_id.into()))?;

        let map_row = |row: &rusqlite::Row| -> rusqlite::Result<SecretValue> {
             let stages_str: String = row.get(3)?;
             let stages: Vec<String> = serde_json::from_str(&stages_str).unwrap_or_default();
             Ok(SecretValue {
                 arn: arn.clone(),
                 name: name.clone(),
                 version_id: row.get(0)?,
                 secret_string: row.get(1)?,
                 secret_binary: row.get(2)?,
                 version_stages: stages,
                 created_date: row.get(4)?,
             }) 
        };

        if let Some(vid) = version_id {
             db.query_row(
                "SELECT version_id, secret_string, secret_binary, version_stages, created_date FROM secret_versions WHERE secret_arn = ?1 AND version_id = ?2", 
                params![arn, vid], 
                map_row
            )
        } else {
             db.query_row(
                "SELECT version_id, secret_string, secret_binary, version_stages, created_date FROM secret_versions WHERE secret_arn = ?1 AND version_stages LIKE '%AWSCURRENT%'", 
                params![arn], 
                map_row
            )
        }.map_err(|_| EmulatorError::NotFound("SecretVersion".into(), "current".into()))
    }
}
