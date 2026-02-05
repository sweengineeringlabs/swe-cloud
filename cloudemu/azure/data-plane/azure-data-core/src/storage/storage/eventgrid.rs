use chrono::Utc;
use super::engine::{StorageEngine, EventGridTopicMetadata, EventGridSubscriptionMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== Topics ====================

    pub fn create_eventgrid_topic(&self, name: &str, location: &str, resource_group: &str) -> Result<EventGridTopicMetadata> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        let endpoint = format!("https://{}.{}.eventgrid.azure.net/api/events", name, location);

        db.execute(
            "INSERT INTO az_eventgrid_topics (name, location, resource_group, endpoint, created_at) VALUES (?, ?, ?, ?, ?)",
            params![name, location, resource_group, endpoint, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Event Grid Topic {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(EventGridTopicMetadata {
            name: name.to_string(),
            location: location.to_string(),
            resource_group: resource_group.to_string(),
            endpoint,
            created_at: now,
        })
    }

    pub fn get_eventgrid_topic(&self, name: &str) -> Result<EventGridTopicMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT name, location, resource_group, endpoint, created_at FROM az_eventgrid_topics WHERE name = ?",
            params![name],
            |row| {
                Ok(EventGridTopicMetadata {
                    name: row.get(0)?,
                    location: row.get(1)?,
                    resource_group: row.get(2)?,
                    endpoint: row.get(3)?,
                    created_at: row.get(4)?,
                })
            },
        ).map_err(|_| EmulatorError::NotFound("EventGridTopic".into(), format!("{}", name)))
    }

    pub fn delete_eventgrid_topic(&self, name: &str) -> Result<()> {
        let db = self.db.lock();
        let count = db.execute("DELETE FROM az_eventgrid_topics WHERE name = ?", params![name])?;
        if count == 0 {
            return Err(EmulatorError::NotFound("EventGridTopic".into(), name.to_string()));
        }
        Ok(())
    }

    // ==================== Subscriptions ====================

    pub fn create_eventgrid_subscription(&self, topic_name: &str, subscription_name: &str, endpoint: &str, protocol: &str) -> Result<EventGridSubscriptionMetadata> {
        // Verify topic exists
        self.get_eventgrid_topic(topic_name)?;

        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();

        db.execute(
            "INSERT INTO az_eventgrid_subscriptions (name, topic_name, endpoint, protocol, created_at) VALUES (?, ?, ?, ?, ?)",
            params![subscription_name, topic_name, endpoint, protocol, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Subscription {} already exists for topic {}", subscription_name, topic_name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(EventGridSubscriptionMetadata {
            name: subscription_name.to_string(),
            topic_name: topic_name.to_string(),
            endpoint: endpoint.to_string(),
            protocol: protocol.to_string(),
            created_at: now,
        })
    }

    pub fn list_eventgrid_subscriptions(&self, topic_name: &str) -> Result<Vec<EventGridSubscriptionMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            "SELECT name, topic_name, endpoint, protocol, created_at FROM az_eventgrid_subscriptions WHERE topic_name = ?"
        )?;

        let subs = stmt.query_map(params![topic_name], |row| {
            Ok(EventGridSubscriptionMetadata {
                name: row.get(0)?,
                topic_name: row.get(1)?,
                endpoint: row.get(2)?,
                protocol: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

        Ok(subs)
    }

    pub fn delete_eventgrid_subscription(&self, topic_name: &str, subscription_name: &str) -> Result<()> {
        let db = self.db.lock();
        let count = db.execute(
            "DELETE FROM az_eventgrid_subscriptions WHERE topic_name = ? AND name = ?",
            params![topic_name, subscription_name]
        )?;
        
        if count == 0 {
            return Err(EmulatorError::NotFound("EventGridSubscription".into(), format!("Subscription {} for topic {}", subscription_name, topic_name)));
        }
        Ok(())
    }
}
