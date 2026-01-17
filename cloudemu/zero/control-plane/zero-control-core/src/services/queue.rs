use zero_control_spi::{ZeroResult, ZeroError};
use zero_data_core::ZeroEngine;
use std::sync::Arc;
use serde_json::json;
use base64;
use chrono;

pub struct QueueService {
    engine: Arc<ZeroEngine>,
}

impl QueueService {
    pub fn new(engine: Arc<ZeroEngine>) -> Self {
        Self { engine }
    }

    pub async fn create_queue(&self, name: &str) -> ZeroResult<String> {
        let conn = self.engine.db.lock();
        
        // Ensure table exists
        let sql = "CREATE TABLE IF NOT EXISTS queues (
            name TEXT PRIMARY KEY,
            url TEXT NOT NULL
        )";
        conn.execute(sql, []).map_err(|e| ZeroError::Internal(e.to_string()))?;

        // Create messages table if needed
        let sql_msg = "CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            queue_name TEXT NOT NULL,
            body TEXT NOT NULL,
            visible_after INTEGER DEFAULT 0
        )";
        conn.execute(sql_msg, []).map_err(|e| ZeroError::Internal(e.to_string()))?;

        let url = format!("http://localhost:8080/v1/queue/{}/messages", name); // Mock URL

        let insert = "INSERT OR REPLACE INTO queues (name, url) VALUES (?1, ?2)";
        conn.execute(insert, zero_data_core::rusqlite::params![name, url])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
            
        Ok(url)
    }

    pub async fn list_queues(&self) -> ZeroResult<Vec<String>> {
        let conn = self.engine.db.lock();
        
        let table_exists: bool = conn.query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='queues'",
            [],
            |row| row.get(0),
        ).unwrap_or(false);

        if !table_exists { return Ok(vec![]); }

        let mut stmt = conn.prepare("SELECT url FROM queues").map_err(|e| ZeroError::Internal(e.to_string()))?;
        let urls = stmt.query_map([], |row| row.get(0)).map_err(|e| ZeroError::Internal(e.to_string()))?
            .collect::<Result<Vec<String>, _>>().map_err(|e| ZeroError::Internal(e.to_string()))?;
        Ok(urls)
    }

    pub async fn send_message(&self, queue_name: &str, body: &str) -> ZeroResult<String> {
        let conn = self.engine.db.lock();
        let id = uuid::Uuid::new_v4().to_string();
        
        // SQS standard: MessageId
        let insert = "INSERT INTO messages (id, queue_name, body, visible_after) VALUES (?1, ?2, ?3, 0)";
        conn.execute(insert, zero_data_core::rusqlite::params![id, queue_name, body])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
            
        Ok(id)
    }

    pub async fn receive_message(&self, queue_name: &str) -> ZeroResult<Option<serde_json::Value>> {
        let conn = self.engine.db.lock();
        let now = chrono::Utc::now().timestamp();
        
        // Find first message that is visible
        let mut stmt = conn.prepare("SELECT id, body FROM messages WHERE queue_name = ?1 AND visible_after <= ?2 LIMIT 1")
             .map_err(|e| ZeroError::Internal(e.to_string()))?;
        
        let mut rows = stmt.query(zero_data_core::rusqlite::params![queue_name, now])
             .map_err(|e| ZeroError::Internal(e.to_string()))?;

        if let Some(row) = rows.next().map_err(|e| ZeroError::Internal(e.to_string()))? {
            let id: String = row.get(0).unwrap();
            let body: String = row.get(1).unwrap();
            
            // Standard SQS visibility timeout: 30 seconds
            let visibility_timeout = 30;
            let next_visible = now + visibility_timeout;
            
            // Generate a receipt handle (for now just the id, but in AWS it's a signed blob)
            let receipt_handle = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, format!("{}:{}", id, next_visible));

            conn.execute("UPDATE messages SET visible_after = ?1 WHERE id = ?2", zero_data_core::rusqlite::params![next_visible, id])
                .map_err(|e| ZeroError::Internal(e.to_string()))?;

            Ok(Some(json!({ 
                "MessageId": id, 
                "Body": body,
                "ReceiptHandle": receipt_handle,
                "Attributes": {
                    "ApproximateReceiveCount": "1",
                    "SentTimestamp": now.to_string()
                }
            })))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_message(&self, _queue_name: &str, receipt_handle: &str) -> ZeroResult<()> {
        let conn = self.engine.db.lock();
        
        // Decode receipt handle to get ID
        let decoded = String::from_utf8(
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, receipt_handle)
                .map_err(|e| ZeroError::Validation(format!("Invalid receipt handle: {}", e)))?
        ).map_err(|e| ZeroError::Validation(format!("Invalid receipt handle UTF8: {}", e)))?;
        
        let parts: Vec<&str> = decoded.split(':').collect();
        if parts.is_empty() {
             return Err(ZeroError::Validation("Malformed receipt handle".into()));
        }
        let id = parts[0];

        let affected = conn.execute("DELETE FROM messages WHERE id = ?1", zero_data_core::rusqlite::params![id])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
        
        if affected == 0 {
             return Err(ZeroError::NotFound("Message not found or already deleted".into()));
        }
        Ok(())
    }
}
