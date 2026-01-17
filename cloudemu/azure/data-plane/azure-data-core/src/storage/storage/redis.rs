use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisCache {
    pub name: String,
    pub resource_group: String,
    pub location: String,
    pub sku_name: String,
    pub sku_family: String,
    pub sku_capacity: i32,
    pub host_name: String,
    pub port: i32,
    pub ssl_port: i32,
    pub provisioning_state: String,
}

impl StorageEngine {
    pub fn init_redis_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS azure_redis_caches (
                name TEXT NOT NULL,
                resource_group TEXT NOT NULL,
                location TEXT NOT NULL,
                sku_name TEXT NOT NULL,
                sku_family TEXT NOT NULL,
                sku_capacity INTEGER NOT NULL,
                host_name TEXT NOT NULL,
                port INTEGER NOT NULL,
                ssl_port INTEGER NOT NULL,
                provisioning_state TEXT NOT NULL,
                PRIMARY KEY(resource_group, name)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_redis_cache(&self, name: &str, rg: &str, location: &str, sku_name: &str, sku_family: &str, capacity: i32) -> Result<RedisCache> {
        let conn = self.get_connection()?;
        let host_name = format!("{}.redis.cache.windows.net", name);
        let port = 6379;
        let ssl_port = 6380;
        let state = "Succeeded";
        
        conn.execute(
            "INSERT INTO azure_redis_caches (name, resource_group, location, sku_name, sku_family, sku_capacity, host_name, port, ssl_port, provisioning_state)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![name, rg, location, sku_name, sku_family, capacity, host_name, port, ssl_port, state],
        )?;

        Ok(RedisCache {
            name: name.to_string(),
            resource_group: rg.to_string(),
            location: location.to_string(),
            sku_name: sku_name.to_string(),
            sku_family: sku_family.to_string(),
            sku_capacity: capacity,
            host_name,
            port,
            ssl_port,
            provisioning_state: state.to_string(),
        })
    }
}
