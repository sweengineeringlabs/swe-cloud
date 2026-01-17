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

    pub async fn create_load_balancer(&self, name: &str, lb_type: &str) -> ZeroResult<serde_json::Value> {
        let conn = self.engine.db.lock();
        
        // Ensure tables exist
        conn.execute("CREATE TABLE IF NOT EXISTS load_balancers (
            name TEXT PRIMARY KEY,
            type TEXT,
            dns_name TEXT,
            status TEXT
        )", []).map_err(|e| ZeroError::Internal(e.to_string()))?;

        conn.execute("CREATE TABLE IF NOT EXISTS listeners (
            id TEXT PRIMARY KEY,
            lb_name TEXT,
            port INTEGER,
            protocol TEXT,
            target_group_arn TEXT
        )", []).map_err(|e| ZeroError::Internal(e.to_string()))?;

        conn.execute("CREATE TABLE IF NOT EXISTS target_groups (
            arn TEXT PRIMARY KEY,
            name TEXT,
            port INTEGER,
            protocol TEXT,
            health_check_path TEXT
        )", []).map_err(|e| ZeroError::Internal(e.to_string()))?;

        conn.execute("CREATE TABLE IF NOT EXISTS targets (
            group_arn TEXT,
            target_id TEXT,
            port INTEGER,
            status TEXT,
            PRIMARY KEY(group_arn, target_id)
        )", []).map_err(|e| ZeroError::Internal(e.to_string()))?;

        let dns_name = format!("{}.lb.zero.local", name);
        let status = "active";
        
        let insert = "INSERT OR REPLACE INTO load_balancers (name, type, dns_name, status) VALUES (?1, ?2, ?3, ?4)";
        conn.execute(insert, zero_data_core::rusqlite::params![name, lb_type, dns_name, status])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
            
        Ok(json!({ 
            "LoadBalancerName": name, 
            "DNSName": dns_name, 
            "Status": { "Code": status },
            "Type": lb_type 
        }))
    }

    pub async fn create_target_group(&self, name: &str, port: i32, protocol: &str) -> ZeroResult<String> {
        let conn = self.engine.db.lock();
        let arn = format!("arn:zero:elasticloadbalancing:000000:targetgroup/{}/{}", name, uuid::Uuid::new_v4());
        
        let sql = "INSERT INTO target_groups (arn, name, port, protocol, health_check_path) VALUES (?1, ?2, ?3, ?4, ?5)";
        conn.execute(sql, zero_data_core::rusqlite::params![arn, name, port, protocol, "/health"])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
        
        Ok(arn)
    }

    pub async fn register_targets(&self, group_arn: &str, target_id: &str, port: i32) -> ZeroResult<()> {
        let conn = self.engine.db.lock();
        let sql = "INSERT OR REPLACE INTO targets (group_arn, target_id, port, status) VALUES (?1, ?2, ?3, ?4)";
        conn.execute(sql, zero_data_core::rusqlite::params![group_arn, target_id, port, "healthy"])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
        Ok(())
    }

    pub async fn create_listener(&self, lb_name: &str, port: i32, protocol: &str, target_group_arn: &str) -> ZeroResult<String> {
        let conn = self.engine.db.lock();
        let id = format!("arn:zero:elasticloadbalancing:000000:listener/{}/{}", lb_name, uuid::Uuid::new_v4());
        
        let sql = "INSERT INTO listeners (id, lb_name, port, protocol, target_group_arn) VALUES (?1, ?2, ?3, ?4, ?5)";
        conn.execute(sql, zero_data_core::rusqlite::params![id, lb_name, port, protocol, target_group_arn])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
        
        Ok(id)
    }

    pub async fn list_load_balancers(&self) -> ZeroResult<Vec<serde_json::Value>> {
        let conn = self.engine.db.lock();
        
        let table_exists: bool = conn.query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='load_balancers'",
            [],
            |row| row.get(0),
        ).unwrap_or(false);

        if !table_exists { return Ok(vec![]); }

        let mut stmt = conn.prepare("SELECT name, type, dns_name, status FROM load_balancers").map_err(|e| ZeroError::Internal(e.to_string()))?;
        let lbs = stmt.query_map([], |row| {
             Ok(json!({ 
                 "LoadBalancerName": row.get::<_, String>(0)?,
                 "Type": row.get::<_, String>(1)?, 
                 "DNSName": row.get::<_, String>(2)?,
                 "Status": { "Code": row.get::<_, String>(3)? }
             }))
        }).map_err(|e| ZeroError::Internal(e.to_string()))?
            .collect::<Result<Vec<serde_json::Value>, _>>().map_err(|e| ZeroError::Internal(e.to_string()))?;
        Ok(lbs)
    }
}
