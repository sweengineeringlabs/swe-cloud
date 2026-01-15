use super::engine::{StorageEngine, PubSubTopicMetadata, PubSubSubscriptionMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== Topics ====================

    pub fn create_pubsub_topic(&self, name: &str, project_id: &str) -> Result<PubSubTopicMetadata> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();

        db.execute(
            "INSERT INTO pubsub_topics (name, project_id, created_at) VALUES (?, ?, ?)",
            params![name, project_id, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Topic {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(PubSubTopicMetadata {
            name: name.to_string(),
            project_id: project_id.to_string(),
            created_at: now,
        })
    }

    pub fn get_pubsub_topic(&self, name: &str) -> Result<PubSubTopicMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT name, project_id, created_at FROM pubsub_topics WHERE name = ?",
            params![name],
            |row| {
                Ok(PubSubTopicMetadata {
                    name: row.get(0)?,
                    project_id: row.get(1)?,
                    created_at: row.get(2)?,
                })
            },
        ).map_err(|_| EmulatorError::NotFound(format!("Topic {} not found", name)))
    }

    pub fn delete_pubsub_topic(&self, name: &str) -> Result<()> {
        let db = self.db.lock();
        let count = db.execute("DELETE FROM pubsub_topics WHERE name = ?", params![name])?;
        if count == 0 {
            return Err(EmulatorError::NotFound(format!("Topic {} not found", name)));
        }
        Ok(())
    }

    // ==================== Subscriptions ====================

    pub fn create_pubsub_subscription(&self, name: &str, topic_name: &str, project_id: &str, push_endpoint: Option<&str>) -> Result<PubSubSubscriptionMetadata> {
        // Verify topic exists
        self.get_pubsub_topic(topic_name)?;

        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();

        db.execute(
            "INSERT INTO pubsub_subscriptions (name, topic_name, project_id, push_endpoint, created_at) VALUES (?, ?, ?, ?, ?)",
            params![name, topic_name, project_id, push_endpoint, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Subscription {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(PubSubSubscriptionMetadata {
            name: name.to_string(),
            topic_name: topic_name.to_string(),
            project_id: project_id.to_string(),
            push_endpoint: push_endpoint.map(|s| s.to_string()),
            ack_deadline_seconds: 10,
            created_at: now,
        })
    }

    pub fn get_pubsub_subscription(&self, name: &str) -> Result<PubSubSubscriptionMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT name, topic_name, project_id, push_endpoint, ack_deadline_seconds, created_at FROM pubsub_subscriptions WHERE name = ?",
            params![name],
            |row| {
                Ok(PubSubSubscriptionMetadata {
                    name: row.get(0)?,
                    topic_name: row.get(1)?,
                    project_id: row.get(2)?,
                    push_endpoint: row.get(3)?,
                    ack_deadline_seconds: row.get(4)?,
                    created_at: row.get(5)?,
                })
            },
        ).map_err(|_| EmulatorError::NotFound(format!("Subscription {} not found", name)))
    }

    pub fn delete_pubsub_subscription(&self, name: &str) -> Result<()> {
        let db = self.db.lock();
        let count = db.execute("DELETE FROM pubsub_subscriptions WHERE name = ?", params![name])?;
        if count == 0 {
            return Err(EmulatorError::NotFound(format!("Subscription {} not found", name)));
        }
        Ok(())
    }
}
