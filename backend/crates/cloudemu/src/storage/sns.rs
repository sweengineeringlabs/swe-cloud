use super::engine::{StorageEngine, TopicMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== SNS Operations ====================

    pub fn create_topic(&self, name: &str, account_id: &str, region: &str) -> Result<TopicMetadata> {
        let arn = format!("arn:aws:sns:{}:{}:{}", region, account_id, name);
        let created_at = chrono::Utc::now().to_rfc3339();
        
        let db = self.db.lock();
        db.execute(
            "INSERT INTO sns_topics (name, arn, created_at) VALUES (?, ?, ?)",
            params![name, arn, created_at],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Topic {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;
        
        Ok(TopicMetadata {
            name: name.to_string(),
            arn,
            display_name: None,
            created_at,
        })
    }

    pub fn subscribe(&self, topic_arn: &str, protocol: &str, endpoint: &str) -> Result<String> {
        let sub_id = uuid::Uuid::new_v4().to_string();
        let sub_arn = format!("{}:{}", topic_arn, sub_id);
        let created_at = chrono::Utc::now().to_rfc3339();
        
        let db = self.db.lock();
        db.execute(
            "INSERT INTO sns_subscriptions (arn, topic_arn, protocol, endpoint, created_at) VALUES (?, ?, ?, ?, ?)",
            params![sub_arn, topic_arn, protocol, endpoint, created_at],
        )?;
        
        Ok(sub_arn)
    }

    pub fn list_topics(&self) -> Result<Vec<TopicMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT name, arn, display_name, created_at FROM sns_topics")?;
        let rows = stmt.query_map([], |row| {
            Ok(TopicMetadata {
                name: row.get(0)?,
                arn: row.get(1)?,
                display_name: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;
        
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }
}
