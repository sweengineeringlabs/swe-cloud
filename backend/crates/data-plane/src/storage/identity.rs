use super::engine::{StorageEngine, UserPoolMetadata, UserMetadata, UserGroupMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== Cognito Operations ====================
    
    pub fn create_user_pool(&self, name: &str, account_id: &str, region: &str) -> Result<UserPoolMetadata> {
        let db = self.db.lock();
        let pool_id = format!("{}_{}", region, uuid::Uuid::new_v4().to_string().replace("-", ""));
        let arn = format!("arn:aws:cognito-idp:{}:{}:userpool/{}", region, account_id, pool_id);
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            "INSERT INTO cognito_user_pools (id, name, arn, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![pool_id, name, arn, now],
        )?;
        
        Ok(UserPoolMetadata {
            id: pool_id,
            name: name.to_string(),
            arn,
            created_at: now,
        })
    }

    pub fn list_user_pools(&self) -> Result<Vec<UserPoolMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT id, name, arn, created_at FROM cognito_user_pools")?;
        let pools = stmt.query_map([], |row| Ok(UserPoolMetadata {
            id: row.get(0)?,
            name: row.get(1)?,
            arn: row.get(2)?,
            created_at: row.get(3)?,
        }))?
        .filter_map(|r| r.ok())
        .collect();
        Ok(pools)
    }

    pub fn admin_create_user(&self, pool_id: &str, username: &str, attributes: Vec<(String, String)>) -> Result<UserMetadata> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        
        let email = attributes.iter().find(|(n, _)| n == "email").map(|(_, v)| v.to_string());
        
        db.execute(
            "INSERT INTO cognito_users (user_pool_id, username, email, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![pool_id, username, email, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("User {} already exists in pool {}", username, pool_id))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;
        
        for (name, value) in attributes {
            db.execute(
                "INSERT INTO cognito_user_attributes (user_pool_id, username, name, value) VALUES (?1, ?2, ?3, ?4)",
                params![pool_id, username, name, value],
            )?;
        }
        
        Ok(UserMetadata {
            user_pool_id: pool_id.to_string(),
            username: username.to_string(),
            email,
            status: "CONFIRMED".to_string(),
            enabled: true,
            created_at: now,
        })
    }

    pub fn admin_get_user(&self, pool_id: &str, username: &str) -> Result<(UserMetadata, Vec<(String, String)>)> {
        let db = self.db.lock();
        let user = db.query_row(
            "SELECT user_pool_id, username, email, status, enabled, created_at FROM cognito_users WHERE user_pool_id = ?1 AND username = ?2",
            params![pool_id, username],
            |row| Ok(UserMetadata {
                user_pool_id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                status: row.get(3)?,
                enabled: row.get(4)?,
                created_at: row.get(5)?,
            })
        ).map_err(|_| EmulatorError::NotFound("User".into(), username.into()))?;
        
        let mut stmt = db.prepare("SELECT name, value FROM cognito_user_attributes WHERE user_pool_id = ?1 AND username = ?2")?;
        let attrs = stmt.query_map(params![pool_id, username], |row| Ok((row.get(0)?, row.get(1)?)))?
            .filter_map(|r| r.ok())
            .collect();
            
        Ok((user, attrs))
    }

    pub fn create_group(&self, pool_id: &str, group_name: &str, description: Option<&str>, precedence: Option<i32>) -> Result<UserGroupMetadata> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            "INSERT INTO cognito_groups (user_pool_id, group_name, description, precedence, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![pool_id, group_name, description, precedence, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Group {} already exists in pool {}", group_name, pool_id))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;
        
        Ok(UserGroupMetadata {
            user_pool_id: pool_id.to_string(),
            group_name: group_name.to_string(),
            description: description.map(|s| s.to_string()),
            precedence,
            created_at: now,
        })
    }

    pub fn list_groups(&self, pool_id: &str) -> Result<Vec<UserGroupMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT user_pool_id, group_name, description, precedence, created_at FROM cognito_groups WHERE user_pool_id = ?1")?;
        let groups = stmt.query_map(params![pool_id], |row| Ok(UserGroupMetadata {
            user_pool_id: row.get(0)?,
            group_name: row.get(1)?,
            description: row.get(2)?,
            precedence: row.get(3)?,
            created_at: row.get(4)?,
        }))?
        .filter_map(|r| r.ok())
        .collect();
        Ok(groups)
    }
}
