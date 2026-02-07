use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpTimeSeries {
    pub metric_type: String,
    pub resource_type: String,
    pub timestamp: String,
    pub value: f64,
}

impl StorageEngine {
    pub fn init_monitoring_tables(&self) -> Result<()> {
        let conn = self.db.lock();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS gcp_metrics (
                metric_type TEXT NOT NULL,
                resource_type TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                value REAL NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_time_series(&self, metric_type: &str, resource_type: &str, value: f64) -> Result<()> {
        let conn = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO gcp_metrics (metric_type, resource_type, timestamp, value)
             VALUES (?1, ?2, ?3, ?4)",
            params![metric_type, resource_type, now, value],
        )?;

        Ok(())
    }

    pub fn list_time_series(&self, _project: &str) -> Result<Vec<GcpTimeSeries>> {
        let conn = self.db.lock();
        let mut stmt = conn.prepare(
            "SELECT metric_type, resource_type, timestamp, value FROM gcp_metrics"
        )?;

        let metrics = stmt.query_map([], |row| {
            Ok(GcpTimeSeries {
                metric_type: row.get(0)?,
                resource_type: row.get(1)?,
                timestamp: row.get(2)?,
                value: row.get(3)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

        Ok(metrics)
    }
}
