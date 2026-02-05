use uuid::Uuid;
use chrono::Utc;
use super::engine::{StorageEngine, KmsKeyMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== KMS Operations ====================

    pub fn create_key(&self, description: Option<&str>, key_usage: &str, tags: Option<&str>, account_id: &str, region: &str) -> Result<KmsKeyMetadata> {
        let db = self.db.lock();
        let key_id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:kms:{}:{}:key/{}", region, account_id, key_id);
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            "INSERT INTO kms_keys (id, arn, description, key_usage, created_at, tags) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![key_id, arn, description, key_usage, now, tags],
        )?;
        
        Ok(KmsKeyMetadata {
            id: key_id,
            arn,
            description: description.map(|s| s.to_string()),
            key_usage: key_usage.to_string(),
            key_state: "Enabled".to_string(),
            created_at: now,
            tags: tags.map(|s| s.to_string()),
        })
    }
    
    pub fn get_key(&self, key_id: &str) -> Result<KmsKeyMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT id, arn, description, key_usage, key_state, created_at, tags FROM kms_keys WHERE id = ?1 OR arn = ?1",
            params![key_id],
            |row| Ok(KmsKeyMetadata {
                id: row.get(0)?,
                arn: row.get(1)?,
                description: row.get(2)?,
                key_usage: row.get(3)?,
                key_state: row.get(4)?,
                created_at: row.get(5)?,
                tags: row.get(6)?,
            })
        ).map_err(|_| EmulatorError::NotFound("Key".into(), key_id.into()))
    }
    
    pub fn enable_key(&self, key_id: &str) -> Result<()> {
         self.set_key_state(key_id, "Enabled")
    }
    
    pub fn disable_key(&self, key_id: &str) -> Result<()> {
         self.set_key_state(key_id, "Disabled")
    }
    
    fn set_key_state(&self, key_id: &str, state: &str) -> Result<()> {
        let db = self.db.lock();
        let rows = db.execute(
             "UPDATE kms_keys SET key_state = ?1 WHERE id = ?2 OR arn = ?2",
             params![state, key_id]
        )?;
        if rows == 0 {
             return Err(EmulatorError::NotFound("Key".into(), key_id.into()));
        }
        Ok(())
    }

    pub fn list_keys(&self) -> Result<Vec<KmsKeyMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT id, arn, description, key_usage, key_state, created_at, tags FROM kms_keys")
            .map_err(|e| EmulatorError::Database(e.to_string()))?;
            
        let keys = stmt.query_map([], |row| Ok(KmsKeyMetadata {
            id: row.get(0)?,
            arn: row.get(1)?,
            description: row.get(2)?,
            key_usage: row.get(3)?,
            key_state: row.get(4)?,
            created_at: row.get(5)?,
            tags: row.get(6)?,
        }))
        .map_err(|e| EmulatorError::Database(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();
        
        Ok(keys)
    }
}
