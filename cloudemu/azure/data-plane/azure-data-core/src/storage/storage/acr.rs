use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub name: String,
    pub resource_group: String,
    pub location: String,
    pub sku: String,
    pub login_server: String,
    pub admin_user_enabled: bool,
    pub provisioning_state: String,
}

impl StorageEngine {
    pub fn init_acr_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS azure_registries (
                name TEXT NOT NULL,
                resource_group TEXT NOT NULL,
                location TEXT NOT NULL,
                sku TEXT NOT NULL,
                login_server TEXT NOT NULL,
                admin_user_enabled BOOLEAN NOT NULL,
                provisioning_state TEXT NOT NULL,
                PRIMARY KEY(resource_group, name)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_registry(&self, name: &str, rg: &str, location: &str, sku: &str, admin_enabled: bool) -> Result<Registry> {
        let conn = self.get_connection()?;
        let login_server = format!("{}.azurecr.io", name);
        let state = "Succeeded";
        
        conn.execute(
            "INSERT INTO azure_registries (name, resource_group, location, sku, login_server, admin_user_enabled, provisioning_state)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![name, rg, location, sku, login_server, admin_enabled, state],
        )?;

        Ok(Registry {
            name: name.to_string(),
            resource_group: rg.to_string(),
            location: location.to_string(),
            sku: sku.to_string(),
            login_server,
            admin_user_enabled: admin_enabled,
            provisioning_state: state.to_string(),
        })
    }
}
