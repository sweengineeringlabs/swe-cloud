use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualNetwork {
    pub name: String,
    pub resource_group: String,
    pub location: String,
    pub address_space: String,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subnet {
    pub name: String,
    pub vnet_name: String,
    pub resource_group: String,
    pub address_prefix: String,
}

impl StorageEngine {
    pub fn init_networking_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS azure_vnets (
                name TEXT NOT NULL,
                resource_group TEXT NOT NULL,
                location TEXT NOT NULL,
                address_space TEXT NOT NULL,
                tags TEXT,
                PRIMARY KEY(resource_group, name)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS azure_subnets (
                name TEXT NOT NULL,
                vnet_name TEXT NOT NULL,
                resource_group TEXT NOT NULL,
                address_prefix TEXT NOT NULL,
                PRIMARY KEY(resource_group, vnet_name, name)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_vnet(&self, name: &str, rg: &str, location: &str, cidr: &str) -> Result<VirtualNetwork> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "INSERT INTO azure_vnets (name, resource_group, location, address_space, tags)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![name, rg, location, cidr, "{}"],
        )?;

        Ok(VirtualNetwork {
            name: name.to_string(),
            resource_group: rg.to_string(),
            location: location.to_string(),
            address_space: cidr.to_string(),
            tags: "{}".to_string(),
        })
    }

    pub fn create_subnet(&self, name: &str, vnet: &str, rg: &str, cidr: &str) -> Result<Subnet> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "INSERT INTO azure_subnets (name, vnet_name, resource_group, address_prefix)
             VALUES (?1, ?2, ?3, ?4)",
            params![name, vnet, rg, cidr],
        )?;

        Ok(Subnet {
            name: name.to_string(),
            vnet_name: vnet.to_string(),
            resource_group: rg.to_string(),
            address_prefix: cidr.to_string(),
        })
    }
}
