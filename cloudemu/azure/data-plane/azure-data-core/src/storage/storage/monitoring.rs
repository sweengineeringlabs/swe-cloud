use chrono::Utc;
use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize}; // Ensure specific import if needed, or rely on mod.rs
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureMetric {
    pub name: String,
    pub resource_id: String,
    pub timestamp: String,
    pub value: f64,
}

impl StorageEngine {
    pub fn init_monitoring_tables(&self) -> Result<()> {
        let conn = self.db.lock();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS azure_metrics (
                resource_id TEXT NOT NULL,
                name TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                value REAL NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn put_metric(&self, resource_id: &str, name: &str, value: f64) -> Result<()> {
        let conn = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO azure_metrics (resource_id, name, timestamp, value)
             VALUES (?1, ?2, ?3, ?4)",
            params![resource_id, name, now, value],
        )?;

        Ok(())
    }

    pub fn list_metrics(&self, resource_id: &str) -> Result<Vec<AzureMetric>> {
        let conn = self.db.lock();
        let mut stmt = conn.prepare(
            "SELECT name, resource_id, timestamp, value FROM azure_metrics WHERE resource_id = ?"
        )?;

        let metrics = stmt.query_map(params![resource_id], |row| {
            Ok(AzureMetric {
                name: row.get(0)?,
                resource_id: row.get(1)?,
                timestamp: row.get(2)?,
                value: row.get(3)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

        Ok(metrics)
    }
}
