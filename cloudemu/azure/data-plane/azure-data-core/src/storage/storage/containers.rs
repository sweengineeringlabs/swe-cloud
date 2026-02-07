use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerGroup {
    pub name: String,
    pub resource_group: String,
    pub location: String,
    pub os_type: String,
    pub state: String,
}

impl StorageEngine {
    pub fn init_containers_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS azure_container_groups (
                name TEXT NOT NULL,
                resource_group TEXT NOT NULL,
                location TEXT NOT NULL,
                os_type TEXT NOT NULL,
                state TEXT NOT NULL,
                PRIMARY KEY(resource_group, name)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_container_group(&self, name: &str, rg: &str, location: &str, os: &str) -> Result<ContainerGroup> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "INSERT INTO azure_container_groups (name, resource_group, location, os_type, state)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![name, rg, location, os, "Running"],
        )?;

        Ok(ContainerGroup {
            name: name.to_string(),
            resource_group: rg.to_string(),
            location: location.to_string(),
            os_type: os.to_string(),
            state: "Running".to_string(),
        })
    }
}
