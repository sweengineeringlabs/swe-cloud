use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OciSecret {
    pub id: String,
    pub secret_name: String,
    pub vault_id: String,
    pub compartment_id: String,
    pub content: String, // Base64
    pub state: String,
}

impl StorageEngine {
    pub fn init_vault_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS oci_secrets (
                id TEXT NOT NULL,
                secret_name TEXT NOT NULL,
                vault_id TEXT NOT NULL,
                compartment_id TEXT NOT NULL,
                content TEXT NOT NULL,
                state TEXT NOT NULL,
                PRIMARY KEY(id)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_secret(&self, name: &str, vault: &str, compartment: &str, content: &str) -> Result<OciSecret> {
        let conn = self.get_connection()?;
        let id = format!("ocid1.vaultsecret.oc1..{}", uuid::Uuid::new_v4());
        
        conn.execute(
            "INSERT INTO oci_secrets (id, secret_name, vault_id, compartment_id, content, state)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, name, vault, compartment, content, "ACTIVE"],
        )?;

        Ok(OciSecret {
            id,
            secret_name: name.to_string(),
            vault_id: vault.to_string(),
            compartment_id: compartment.to_string(),
            content: content.to_string(),
            state: "ACTIVE".to_string(),
        })
    }
}
