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

    pub fn list_subscriptions_by_topic(&self, topic_arn: &str) -> Result<Vec<super::engine::SubscriptionMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            "SELECT arn, topic_arn, protocol, endpoint, created_at FROM sns_subscriptions WHERE topic_arn = ?"
        )?;
        let rows = stmt.query_map(params![topic_arn], |row| {
            Ok(super::engine::SubscriptionMetadata {
                arn: row.get(0)?,
                topic_arn: row.get(1)?,
                protocol: row.get(2)?,
                endpoint: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sns_topics_and_subscriptions() {
        let engine = StorageEngine::in_memory().unwrap();
        
        // Create Topic
        let topic = engine.create_topic("my-topic", "123456789012", "us-east-1").unwrap();
        assert_eq!(topic.name, "my-topic");
        assert!(topic.arn.ends_with(":my-topic"));
        
        // List Topics
        let topics = engine.list_topics().unwrap();
        assert_eq!(topics.len(), 1);
        assert_eq!(topics[0].name, "my-topic");

        // Subscribe
        let sub_arn = engine.subscribe(&topic.arn, "sqs", "arn:aws:sqs:us-east-1:123:queue").unwrap();
        
        // List Subscriptions
        let subs = engine.list_subscriptions_by_topic(&topic.arn).unwrap();
        assert_eq!(subs.len(), 1);
        assert_eq!(subs[0].arn, sub_arn);
        assert_eq!(subs[0].protocol, "sqs");
        assert_eq!(subs[0].endpoint, "arn:aws:sqs:us-east-1:123:queue");
    }
}

