use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OciZone {
    pub id: String,
    pub name: String,
    pub zone_type: String,
    pub lifecycle_state: String,
}

impl StorageEngine {
    const TABLE_OCI_ZONES: &'static str = "oci_dns_zones";

    pub fn init_dns_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                zone_type TEXT NOT NULL,
                lifecycle_state TEXT NOT NULL,
                created_at INTEGER
            )", 
            Self::TABLE_OCI_ZONES
        ), [])?;

        Ok(())
    }

    pub fn create_zone(&self, name: &str, zone_type: &str) -> Result<OciZone> {
        let conn = self.get_connection()?;
        let id = format!("ocid1.dns-zone.oc1..{}", uuid::Uuid::new_v4());

        conn.execute(
            &format!("INSERT INTO {} (
                id, name, zone_type, lifecycle_state, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5)", Self::TABLE_OCI_ZONES),
            params![id, name, zone_type, "ACTIVE", chrono::Utc::now().timestamp()],
        )?;

        Ok(OciZone {
            id,
            name: name.to_string(),
            zone_type: zone_type.to_string(),
            lifecycle_state: "ACTIVE".to_string(),
        })
    }
}
