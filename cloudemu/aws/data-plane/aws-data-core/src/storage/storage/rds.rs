use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RdsInstance {
    pub identifier: String,
    pub engine: String,
    pub class: String,
    pub status: String,
    pub endpoint_address: String,
    pub endpoint_port: i32,
    pub username: String,
    pub allocated_storage: i32,
}

impl StorageEngine {
    const TABLE_RDS_INSTANCES: &'static str = "aws_rds_instances";

    pub fn init_rds_tables(&self) -> Result<()> {
        let conn = self.db.lock();
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                identifier TEXT PRIMARY KEY,
                engine TEXT NOT NULL,
                class TEXT NOT NULL,
                status TEXT NOT NULL,
                username TEXT NOT NULL,
                allocated_storage INTEGER,
                endpoint_address TEXT,
                endpoint_port INTEGER,
                created_at INTEGER
            )", 
            Self::TABLE_RDS_INSTANCES
        ), [])?;

        Ok(())
    }

    pub fn create_db_instance(
        &self, 
        identifier: &str, 
        engine: &str, 
        class: &str,
        username: &str,
        allocated_storage: i32
    ) -> Result<RdsInstance> {
        let conn = self.db.lock();
        
        let status = "available";
        let address = format!("{}.cluster-mock.us-east-1.rds.amazonaws.com", identifier);
        let port = 3306; // Mock MySQL port default

        conn.execute(
            &format!("INSERT INTO {} (
                identifier, engine, class, status, username, allocated_storage, 
                endpoint_address, endpoint_port, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)", Self::TABLE_RDS_INSTANCES),
            params![
                identifier, engine, class, status, username, allocated_storage,
                address, port, chrono::Utc::now().timestamp()
            ],
        )?;

        Ok(RdsInstance {
            identifier: identifier.to_string(),
            engine: engine.to_string(),
            class: class.to_string(),
            status: status.to_string(),
            endpoint_address: address,
            endpoint_port: port,
            username: username.to_string(),
            allocated_storage,
        })
    }

    pub fn list_db_instances(&self) -> Result<Vec<RdsInstance>> {
        let conn = self.db.lock();
        let mut stmt = conn.prepare(&format!("SELECT Identifier, Engine, Class, Status, Username, Allocated_Storage, Endpoint_Address, Endpoint_Port FROM {}", Self::TABLE_RDS_INSTANCES))?;
        
        let instances = stmt.query_map([], |row| {
            Ok(RdsInstance {
                identifier: row.get(0)?,
                engine: row.get(1)?,
                class: row.get(2)?,
                status: row.get(3)?,
                username: row.get(4)?,
                allocated_storage: row.get(5)?,
                endpoint_address: row.get(6)?,
                endpoint_port: row.get(7)?,
            })
        })?
        .collect::<std::result::Result<Vec<RdsInstance>, _>>()?;
            
        Ok(instances)
    }
}
