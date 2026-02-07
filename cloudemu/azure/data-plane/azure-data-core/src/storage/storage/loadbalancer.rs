use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancer {
    pub name: String,
    pub resource_group: String,
    pub location: String,
    pub sku: String, // Basic | Standard
    pub provisioning_state: String,
}

impl StorageEngine {
    pub fn init_loadbalancer_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS azure_load_balancers (
                name TEXT NOT NULL,
                resource_group TEXT NOT NULL,
                location TEXT NOT NULL,
                sku TEXT NOT NULL,
                provisioning_state TEXT NOT NULL,
                PRIMARY KEY(resource_group, name)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_load_balancer(&self, name: &str, rg: &str, location: &str, sku: &str) -> Result<LoadBalancer> {
        let conn = self.get_connection()?;
        let state = "Succeeded";
        
        conn.execute(
            "INSERT INTO azure_load_balancers (name, resource_group, location, sku, provisioning_state)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![name, rg, location, sku, state],
        )?;

        Ok(LoadBalancer {
            name: name.to_string(),
            resource_group: rg.to_string(),
            location: location.to_string(),
            sku: sku.to_string(),
            provisioning_state: state.to_string(),
        })
    }
}
