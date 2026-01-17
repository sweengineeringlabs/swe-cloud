use zero_control_spi::{ZeroResult, ZeroError};
use zero_data_core::ZeroEngine;
use std::sync::Arc;
use serde_json::json;

pub struct LbService {
    engine: Arc<ZeroEngine>,
}

impl LbService {
    pub fn new(engine: Arc<ZeroEngine>) -> Self {
        Self { engine }
    }

    pub async fn create_load_balancer(&self, name: &str, port: i32, target: &str) -> ZeroResult<String> {
        let conn = self.engine.db.lock();
        
        // Ensure table exists
        let sql = "CREATE TABLE IF NOT EXISTS load_balancers (
            name TEXT PRIMARY KEY,
            port INTEGER,
            target TEXT,
            dns_name TEXT
        )";
        conn.execute(sql, []).map_err(|e| ZeroError::Internal(e.to_string()))?;

        let dns_name = format!("{}.lb.zero.local", name);
        
        let insert = "INSERT OR REPLACE INTO load_balancers (name, port, target, dns_name) VALUES (?1, ?2, ?3, ?4)";
        conn.execute(insert, zero_data_core::rusqlite::params![name, port, target, dns_name])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
            
        Ok(dns_name)
    }

    pub async fn list_load_balancers(&self) -> ZeroResult<Vec<serde_json::Value>> {
        let conn = self.engine.db.lock();
        
        let table_exists: bool = conn.query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='load_balancers'",
            [],
            |row| row.get(0),
        ).unwrap_or(false);

        if !table_exists { return Ok(vec![]); }

        let mut stmt = conn.prepare("SELECT name, port, target, dns_name FROM load_balancers").map_err(|e| ZeroError::Internal(e.to_string()))?;
        let lbs = stmt.query_map([], |row| {
             Ok(json!({ 
                 "Name": row.get::<_, String>(0)?,
                 "Port": row.get::<_, i32>(1)?, 
                 "Target": row.get::<_, String>(2)?,
                 "DNSName": row.get::<_, String>(3)? 
             }))
        }).map_err(|e| ZeroError::Internal(e.to_string()))?
            .collect::<Result<Vec<serde_json::Value>, _>>().map_err(|e| ZeroError::Internal(e.to_string()))?;
        Ok(lbs)
    }
}
