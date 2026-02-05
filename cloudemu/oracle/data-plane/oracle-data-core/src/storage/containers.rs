use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInstance {
    pub id: String,
    pub display_name: String,
    pub compartment_id: String,
    pub availability_domain: String,
    pub state: String,
}

impl StorageEngine {
    pub fn init_containers_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS oci_container_instances (
                id TEXT NOT NULL,
                display_name TEXT NOT NULL,
                compartment_id TEXT NOT NULL,
                availability_domain TEXT NOT NULL,
                state TEXT NOT NULL,
                PRIMARY KEY(id)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_container_instance(&self, name: &str, compartment: &str, ad: &str) -> Result<ContainerInstance> {
        let conn = self.get_connection()?;
        let id = format!("ocid1.computecontainerinstance.oc1..{}", uuid::Uuid::new_v4());
        
        conn.execute(
            "INSERT INTO oci_container_instances (id, display_name, compartment_id, availability_domain, state)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, name, compartment, ad, "ACTIVE"],
        )?;

        Ok(ContainerInstance {
            id,
            display_name: name.to_string(),
            compartment_id: compartment.to_string(),
            availability_domain: ad.to_string(),
            state: "ACTIVE".to_string(),
        })
    }
}
