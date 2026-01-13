use super::engine::{StorageEngine, MetricMetadata, LogGroupMetadata, LogStreamMetadata, LogEventMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== CloudWatch Operations ====================
    
    pub fn put_metric_data(&self, namespace: &str, metrics: Vec<MetricMetadata>) -> Result<()> {
        let db = self.db.lock();
        for m in metrics {
            db.execute(
                "INSERT INTO cw_metrics (namespace, metric_name, dimensions, value, unit, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![namespace, m.metric_name, m.dimensions, m.value, m.unit, m.timestamp],
            )?;
        }
        Ok(())
    }

    pub fn list_metrics(&self, namespace: Option<&str>, metric_name: Option<&str>) -> Result<Vec<MetricMetadata>> {
        let db = self.db.lock();
        let mut query = "SELECT namespace, metric_name, dimensions, value, unit, timestamp FROM cw_metrics WHERE 1=1".to_string();
        let mut args: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(ns) = namespace {
            query.push_str(" AND namespace = ?");
            args.push(Box::new(ns.to_string()));
        }
        if let Some(name) = metric_name {
            query.push_str(" AND metric_name = ?");
            args.push(Box::new(name.to_string()));
        }

        let mut stmt = db.prepare(&query)?;
        
        let params_refs: Vec<&dyn rusqlite::ToSql> = args.iter().map(|b| b.as_ref()).collect();

        let metrics = stmt.query_map(rusqlite::params_from_iter(params_refs), |row| Ok(MetricMetadata {
            namespace: row.get(0)?,
            metric_name: row.get(1)?,
            dimensions: row.get(2)?,
            value: row.get(3)?,
            unit: row.get(4)?,
            timestamp: row.get(5)?,
        }))?
        .filter_map(|r| r.ok())
        .collect();

        Ok(metrics)
    }

    pub fn create_log_group(&self, name: &str, account_id: &str, region: &str) -> Result<LogGroupMetadata> {
        let db = self.db.lock();
        let arn = format!("arn:aws:logs:{}:{}:log-group:{}", region, account_id, name);
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            "INSERT INTO cw_log_groups (name, arn, created_at) VALUES (?1, ?2, ?3)",
            params![name, arn, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Log group {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;
        
        Ok(LogGroupMetadata {
            name: name.to_string(),
            arn,
            retention_days: None,
            created_at: now,
        })
    }

    pub fn delete_log_group(&self, name: &str) -> Result<()> {
        let db = self.db.lock();
        let rows = db.execute("DELETE FROM cw_log_groups WHERE name = ?1", params![name])?;
        if rows == 0 {
            return Err(EmulatorError::NotFound("LogGroup".into(), name.into()));
        }
        Ok(())
    }

    pub fn create_log_stream(&self, group_name: &str, stream_name: &str, account_id: &str, region: &str) -> Result<LogStreamMetadata> {
        let db = self.db.lock();
        let arn = format!("arn:aws:logs:{}:{}:log-group:{}:log-stream:{}", region, account_id, group_name, stream_name);
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            "INSERT INTO cw_log_streams (name, log_group_name, arn, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![stream_name, group_name, arn, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Log stream {} already exists in group {}", stream_name, group_name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;
        
        Ok(LogStreamMetadata {
            name: stream_name.to_string(),
            log_group_name: group_name.to_string(),
            arn,
            created_at: now,
        })
    }

    pub fn put_log_events(&self, group_name: &str, stream_name: &str, events: Vec<LogEventMetadata>) -> Result<()> {
        let db = self.db.lock();
        for e in events {
            db.execute(
                "INSERT INTO cw_log_events (log_group_name, log_stream_name, timestamp, message) VALUES (?1, ?2, ?3, ?4)",
                params![group_name, stream_name, e.timestamp, e.message],
            )?;
        }
        Ok(())
    }

    pub fn get_log_events(&self, group_name: &str, stream_name: &str) -> Result<Vec<LogEventMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            "SELECT timestamp, message FROM cw_log_events WHERE log_group_name = ?1 AND log_stream_name = ?2 ORDER BY timestamp"
        )?;
        let events = stmt.query_map(params![group_name, stream_name], |row| Ok(LogEventMetadata {
            timestamp: row.get(0)?,
            message: row.get(1)?,
        }))?
        .filter_map(|r| r.ok())
        .collect();
        Ok(events)
    }
}
