use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpInstance {
    pub name: String,
    pub project: String,
    pub zone: String,
    pub machine_type: String,
    pub status: String,
}

impl StorageEngine {
    const TABLE_GCP_INSTANCES: &'static str = "gcp_instances";

    pub fn init_compute_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                self_link TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                project TEXT NOT NULL,
                zone TEXT NOT NULL,
                machine_type TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at INTEGER,
                UNIQUE(name, project, zone)
            )", 
            Self::TABLE_GCP_INSTANCES
        ), [])?;

        Ok(())
    }

    pub fn insert_instance(&self, vm: GcpInstance) -> Result<GcpInstance> {
        let conn = self.get_connection()?;
        let self_link = format!("https://www.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}", vm.project, vm.zone, vm.name);

        conn.execute(
            &format!("INSERT INTO {} (
                self_link, name, project, zone, machine_type, status, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", Self::TABLE_GCP_INSTANCES),
            params![
                self_link, vm.name, vm.project, vm.zone, vm.machine_type, "RUNNING", 
                chrono::Utc::now().timestamp()
            ],
        )?;

        Ok(vm)
    }
}
