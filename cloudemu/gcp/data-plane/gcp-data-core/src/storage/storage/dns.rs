use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedZone {
    pub name: String,
    pub dns_name: String,
    pub description: String,
    pub id: String,
    pub visibility: String,
}

impl StorageEngine {
    const TABLE_DNS_ZONES: &'static str = "gcp_dns_managed_zones";

    pub fn init_dns_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                name TEXT PRIMARY KEY,
                dns_name TEXT NOT NULL,
                description TEXT,
                id TEXT NOT NULL,
                visibility TEXT NOT NULL,
                created_at INTEGER
            )", 
            Self::TABLE_DNS_ZONES
        ), [])?;

        Ok(())
    }

    pub fn create_managed_zone(&self, name: &str, dns_name: &str, desc: &str) -> Result<ManagedZone> {
        let conn = self.get_connection()?;
        let id = uuid::Uuid::new_v4().to_string(); // Numeric in real GCP, but string here is fine

        conn.execute(
            &format!("INSERT INTO {} (
                name, dns_name, description, id, visibility, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", Self::TABLE_DNS_ZONES),
            params![name, dns_name, desc, id, "public", chrono::Utc::now().timestamp()],
        )?;

        Ok(ManagedZone {
            name: name.to_string(),
            dns_name: dns_name.to_string(),
            description: desc.to_string(),
            id,
            visibility: "public".to_string(),
        })
    }
}
