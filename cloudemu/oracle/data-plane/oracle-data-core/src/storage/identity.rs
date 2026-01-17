use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OciUser {
    pub id: String,
    pub name: String,
    pub description: String,
    pub lifecycle_state: String,
}

impl StorageEngine {
    const TABLE_OCI_USERS: &'static str = "oci_iam_users";

    pub fn init_identity_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                lifecycle_state TEXT NOT NULL,
                created_at INTEGER
            )", 
            Self::TABLE_OCI_USERS
        ), [])?;

        Ok(())
    }

    pub fn create_user(&self, name: &str, desc: &str) -> Result<OciUser> {
        let conn = self.get_connection()?;
        let id = format!("ocid1.user.oc1..{}", uuid::Uuid::new_v4());

        conn.execute(
            &format!("INSERT INTO {} (
                id, name, description, lifecycle_state, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5)", Self::TABLE_OCI_USERS),
            params![id, name, desc, "ACTIVE", chrono::Utc::now().timestamp()],
        )?;

        Ok(OciUser {
            id,
            name: name.to_string(),
            description: desc.to_string(),
            lifecycle_state: "ACTIVE".to_string(),
        })
    }
}
