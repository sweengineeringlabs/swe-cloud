use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OciFunction {
    pub application_id: String,
    pub id: String,
    pub display_name: String,
    pub image: String,
    pub memory_in_mbs: i64,
}

impl StorageEngine {
    pub fn init_functions_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS oci_functions (
                id TEXT NOT NULL,
                application_id TEXT NOT NULL,
                display_name TEXT NOT NULL,
                image TEXT NOT NULL,
                memory_in_mbs INTEGER NOT NULL,
                PRIMARY KEY(id)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_function(&self, app_id: &str, name: &str, image: &str, memory: i64) -> Result<OciFunction> {
        let conn = self.get_connection()?;
        let id = format!("ocid1.fn.oc1..{}", uuid::Uuid::new_v4());
        
        conn.execute(
            "INSERT INTO oci_functions (id, application_id, display_name, image, memory_in_mbs)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, app_id, name, image, memory],
        )?;

        Ok(OciFunction {
            id,
            application_id: app_id.to_string(),
            display_name: name.to_string(),
            image: image.to_string(),
            memory_in_mbs: memory,
        })
    }
}
