use super::StorageEngine;
use crate::error::Result;
use rusqlite::params;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleInstance {
    pub id: String,
    pub compartment_id: String,
    pub display_name: String,
    pub shape: String,
    pub lifecycle_state: String,
}

impl StorageEngine {
    const TABLE_OCI_INSTANCES: &'static str = "oci_instances";

    pub fn init_compute_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                compartment_id TEXT NOT NULL,
                display_name TEXT NOT NULL,
                shape TEXT NOT NULL,
                lifecycle_state TEXT NOT NULL,
                created_at INTEGER
            )", 
            Self::TABLE_OCI_INSTANCES
        ), [])?;

        Ok(())
    }

    pub fn launch_instance(&self, instance: OracleInstance) -> Result<OracleInstance> {
        let conn = self.get_connection()?;
        
        conn.execute(
            &format!("INSERT INTO {} (
                id, compartment_id, display_name, shape, lifecycle_state, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", Self::TABLE_OCI_INSTANCES),
            params![
                instance.id, instance.compartment_id, instance.display_name, instance.shape, "RUNNING",
                chrono::Utc::now().timestamp()
            ],
        )?;

        Ok(instance)
    }
}
