use super::engine::{StorageEngine, QueueMetadata, MessageMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== SQS Operations ====================

    pub fn create_queue(&self, name: &str, account_id: &str, region: &str) -> Result<QueueMetadata> {
        let db = self.db.lock();
        let arn = format!("arn:aws:sqs:{}:{}:{}", region, account_id, name);
        let url = format!("http://localhost:4566/{}/{}", account_id, name);
        let now = chrono::Utc::now().to_rfc3339();

        db.execute(
            "INSERT INTO sqs_queues (name, url, arn, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![name, url, arn, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Queue {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(QueueMetadata {
            name: name.to_string(),
            url,
            arn,
            created_at: now,
            visibility_timeout: 30,
            message_retention_period: 345600,
            delay_seconds: 0,
            receive_message_wait_time_seconds: 0,
        })
    }

    pub fn send_message(&self, queue_name: &str, body: &str) -> Result<String> {
        let db = self.db.lock();
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        // Check if queue exists
        let exists: bool = db.query_row(
            "SELECT 1 FROM sqs_queues WHERE name = ?1",
            params![queue_name],
            |_| Ok(true),
        ).unwrap_or(false);

        if !exists {
            return Err(EmulatorError::NotFound("Queue".into(), queue_name.into()));
        }

        db.execute(
            "INSERT INTO sqs_messages (id, queue_name, body, sent_at, visible_at) VALUES (?1, ?2, ?3, ?4, ?4)",
            params![id, queue_name, body, now],
        )?;

        Ok(id)
    }

    pub fn receive_message(&self, queue_name: &str, max_count: i32) -> Result<Vec<MessageMetadata>> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();

        let mut stmt = db.prepare(
            "SELECT id, body, md5_body, sent_at, visible_at, receive_count FROM sqs_messages 
             WHERE queue_name = ?1 AND visible_at <= ?2 
             LIMIT ?3"
        )?;

        let mut messages: Vec<MessageMetadata> = stmt.query_map(params![queue_name, now, max_count], |row| {
            Ok(MessageMetadata {
                id: row.get(0)?,
                queue_name: queue_name.to_string(),
                body: row.get(1)?,
                md5_body: row.get(2)?,
                sent_at: row.get(3)?,
                visible_at: row.get(4)?,
                receipt_handle: None,
                receive_count: row.get(5)?,
            })
        })?.filter_map(|r| r.ok()).collect();

        // Update visibility and receipt handles for received messages
        for msg in &mut messages {
            let handle = uuid::Uuid::new_v4().to_string();
            let new_visible_at = (chrono::Utc::now() + chrono::Duration::seconds(30)).to_rfc3339();
            
            db.execute(
                "UPDATE sqs_messages SET receipt_handle = ?1, visible_at = ?2, receive_count = receive_count + 1 WHERE id = ?3",
                params![handle, new_visible_at, msg.id],
            )?;
            
            msg.receipt_handle = Some(handle);
            msg.visible_at = new_visible_at;
        }

        Ok(messages)
    }

    pub fn delete_message(&self, queue_name: &str, receipt_handle: &str) -> Result<()> {
        let db = self.db.lock();
        let rows = db.execute(
            "DELETE FROM sqs_messages WHERE queue_name = ?1 AND receipt_handle = ?2",
            params![queue_name, receipt_handle],
        )?;

        if rows == 0 {
            return Err(EmulatorError::NotFound("Message".into(), receipt_handle.into()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqs_workflow() {
        let engine = StorageEngine::in_memory().unwrap();
        
        // Create Queue
        let queue = engine.create_queue("my-queue", "123", "us-east-1").unwrap();
        assert_eq!(queue.name, "my-queue");
        assert!(queue.url.contains("my-queue"));
        
        // Send Message
        let msg_id = engine.send_message("my-queue", "hello world").unwrap();
        assert!(!msg_id.is_empty());
        
        // Receive Message (should be visible immediately)
        let messages = engine.receive_message("my-queue", 10).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].body, "hello world");
        assert!(messages[0].receipt_handle.is_some());
        
        // Delete Message
        engine.delete_message("my-queue", messages[0].receipt_handle.as_ref().unwrap()).unwrap();
        
        // Verify Empty
        let messages_after = engine.receive_message("my-queue", 10).unwrap();
        assert_eq!(messages_after.len(), 0);
    }
}

