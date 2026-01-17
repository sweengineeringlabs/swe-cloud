use zero_control_spi::{ZeroResult, ZeroError};
use zero_data_core::ZeroEngine;
use std::sync::Arc;
use serde_json::json;

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
        
        let insert = "INSERT INTO messages (id, queue_name, body) VALUES (?1, ?2, ?3)";
        conn.execute(insert, zero_data_core::rusqlite::params![id, queue_name, body])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
            
        Ok(id)
    }

    pub async fn receive_message(&self, queue_name: &str) -> ZeroResult<Option<serde_json::Value>> {
        let conn = self.engine.db.lock();
        
        // Simple FIFO fetch
        let mut stmt = conn.prepare("SELECT id, body FROM messages WHERE queue_name = ?1 LIMIT 1")
             .map_err(|e| ZeroError::Internal(e.to_string()))?;
        
        let mut rows = stmt.query(zero_data_core::rusqlite::params![queue_name])
             .map_err(|e| ZeroError::Internal(e.to_string()))?;

        if let Some(row) = rows.next().map_err(|e| ZeroError::Internal(e.to_string()))? {
            let id: String = row.get(0).unwrap();
            let body: String = row.get(1).unwrap();
            
            // Delete-on-read for simplicity in this MVP (Not real visibility timeout)
            conn.execute("DELETE FROM messages WHERE id = ?1", zero_data_core::rusqlite::params![id])
                .map_err(|e| ZeroError::Internal(e.to_string()))?;

            Ok(Some(json!({ "MessageId": id, "Body": body })))
        } else {
            Ok(None)
        }
    }
}
