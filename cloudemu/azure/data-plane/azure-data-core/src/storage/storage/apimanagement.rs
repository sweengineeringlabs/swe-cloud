use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiService {
    pub name: String,
    pub resource_group: String,
    pub location: String,
    pub publisher_name: String,
    pub publisher_email: String,
    pub sku_name: String,
    pub gateway_url: String,
    pub provisioning_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Api {
    pub name: String,
    pub service_name: String,
    pub resource_group: String,
    pub display_name: Option<String>,
    pub path: String,
    pub protocols: String,
}

impl StorageEngine {
    pub fn init_apimanagement_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS azure_api_services (
                name TEXT NOT NULL,
                resource_group TEXT NOT NULL,
                location TEXT NOT NULL,
                publisher_name TEXT NOT NULL,
                publisher_email TEXT NOT NULL,
                sku_name TEXT NOT NULL,
                gateway_url TEXT NOT NULL,
                provisioning_state TEXT NOT NULL,
                PRIMARY KEY(resource_group, name)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS azure_apis (
                name TEXT NOT NULL,
                service_name TEXT NOT NULL,
                resource_group TEXT NOT NULL,
                display_name TEXT,
                path TEXT NOT NULL,
                protocols TEXT NOT NULL,
                PRIMARY KEY(resource_group, service_name, name)
            )",
            [],
        )?;

        Ok(())
    }

    pub fn create_api_service(&self, name: &str, rg: &str, location: &str, pub_name: &str, pub_email: &str, sku: &str) -> Result<ApiService> {
        let conn = self.get_connection()?;
        let gateway_url = format!("https://{}.azure-api.net", name);
        let state = "Succeeded";
        
        conn.execute(
            "INSERT INTO azure_api_services (name, resource_group, location, publisher_name, publisher_email, sku_name, gateway_url, provisioning_state)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![name, rg, location, pub_name, pub_email, sku, gateway_url, state],
        )?;

        Ok(ApiService {
            name: name.to_string(),
            resource_group: rg.to_string(),
            location: location.to_string(),
            publisher_name: pub_name.to_string(),
            publisher_email: pub_email.to_string(),
            sku_name: sku.to_string(),
            gateway_url,
            provisioning_state: state.to_string(),
        })
    }

    pub fn create_api(&self, name: &str, service_name: &str, rg: &str, path: &str) -> Result<Api> {
         let conn = self.get_connection()?;
         
         conn.execute(
            "INSERT INTO azure_apis (name, service_name, resource_group, display_name, path, protocols)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![name, service_name, rg, name, path, "[\"https\"]"],
        )?;
        
        Ok(Api {
            name: name.to_string(),
            service_name: service_name.to_string(),
            resource_group: rg.to_string(),
            display_name: Some(name.to_string()),
            path: path.to_string(),
            protocols: "[\"https\"]".to_string(),
        })
    }
}
