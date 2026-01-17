use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleAutonomousDb {
    pub id: String,
    pub compartment_id: String,
    pub display_name: String,
    pub db_name: String,
    pub cpu_core_count: i32,
    pub data_storage_size_in_tbs: i32,
    pub lifecycle_state: String,
}

impl StorageEngine {
    const TABLE_OCI_ADB: &'static str = "oci_autonomous_databases";

    pub fn init_db_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                compartment_id TEXT NOT NULL,
                display_name TEXT NOT NULL,
                db_name TEXT NOT NULL,
                cpu_core_count INTEGER,
                data_storage_size_in_tbs INTEGER,
                lifecycle_state TEXT NOT NULL,
                created_at INTEGER
            )", 
            Self::TABLE_OCI_ADB
        ), [])?;

        Ok(())
    }

    pub fn create_autonomous_database(&self, db: OracleAutonomousDb) -> Result<OracleAutonomousDb> {
        let conn = self.get_connection()?;
        
        conn.execute(
            &format!("INSERT INTO {} (
                id, compartment_id, display_name, db_name, cpu_core_count, data_storage_size_in_tbs, lifecycle_state, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", Self::TABLE_OCI_ADB),
            params![
                db.id, db.compartment_id, db.display_name, db.db_name, db.cpu_core_count, db.data_storage_size_in_tbs, "AVAILABLE",
                chrono::Utc::now().timestamp()
            ],
        )?;

        Ok(db)
    }
}
