use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OciMetric {
    pub namespace: String,
    pub resource_group: String,
    pub timestamp: String,
    pub value: f64,
}

impl StorageEngine {
    pub fn init_monitoring_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS oci_metrics (
                namespace TEXT NOT NULL,
                resource_group TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                value REAL NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn post_metric(&self, namespace: &str, rg: &str, value: f64) -> Result<()> {
        let conn = self.get_connection()?;
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO oci_metrics (namespace, resource_group, timestamp, value)
             VALUES (?1, ?2, ?3, ?4)",
            params![namespace, rg, now, value],
        )?;

        Ok(())
    }

    pub fn list_metrics(&self, _compartment: &str) -> Result<Vec<OciMetric>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT namespace, resource_group, timestamp, value FROM oci_metrics"
        )?;

        let metrics = stmt.query_map([], |row| {
            Ok(OciMetric {
                namespace: row.get(0)?,
                resource_group: row.get(1)?,
                timestamp: row.get(2)?,
                value: row.get(3)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

        Ok(metrics)
    }
}
