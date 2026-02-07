use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRing {
    pub name: String,
    pub project_id: String,
    pub location: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoKey {
    pub name: String,
    pub key_ring: String,
    pub purpose: String,
    pub created_at: String,
}

impl StorageEngine {
    pub fn init_kms_tables(&self) -> Result<()> {
        let conn = self.db.lock();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS gcp_key_rings (
                name TEXT NOT NULL,
                project_id TEXT NOT NULL,
                location TEXT NOT NULL,
                created_at TEXT NOT NULL,
                PRIMARY KEY(project_id, location, name)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS gcp_crypto_keys (
                name TEXT NOT NULL,
                key_ring TEXT NOT NULL,
                purpose TEXT NOT NULL,
                created_at TEXT NOT NULL,
                PRIMARY KEY(key_ring, name)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_key_ring(&self, name: &str, project: &str, location: &str) -> Result<KeyRing> {
        let conn = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO gcp_key_rings (name, project_id, location, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![name, project, location, now],
        )?;

        Ok(KeyRing {
            name: name.to_string(),
            project_id: project.to_string(),
            location: location.to_string(),
            created_at: now,
        })
    }

    pub fn create_crypto_key(&self, name: &str, key_ring: &str, purpose: &str) -> Result<CryptoKey> {
        let conn = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO gcp_crypto_keys (name, key_ring, purpose, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![name, key_ring, purpose, now],
        )?;

        Ok(CryptoKey {
            name: name.to_string(),
            key_ring: key_ring.to_string(),
            purpose: purpose.to_string(),
            created_at: now,
        })
    }
}
