use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OciQueue {
    pub id: String,
    pub name: String,
    pub compartment_id: String,
    pub messages_endpoint: String,
}

impl StorageEngine {
    pub fn init_queue_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS oci_queues (
                id TEXT NOT NULL,
                name TEXT NOT NULL,
                compartment_id TEXT NOT NULL,
                messages_endpoint TEXT NOT NULL,
                PRIMARY KEY(id)
            )",
            [],
        )?;
        // Messages table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS oci_queue_messages (
                id TEXT NOT NULL,
                queue_id TEXT NOT NULL,
                content TEXT NOT NULL,
                visible_after INTEGER NOT NULL,
                expire_after INTEGER NOT NULL,
                PRIMARY KEY(id)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_queue(&self, name: &str, compartment: &str) -> Result<OciQueue> {
        let conn = self.get_connection()?;
        let id = format!("ocid1.queue.oc1..{}", uuid::Uuid::new_v4());
        let endpoint = format!("https://cell-1.queue.messaging.us-phoenix-1.oci.oraclecloud.com/20210201/{}", id);
        
        conn.execute(
            "INSERT INTO oci_queues (id, name, compartment_id, messages_endpoint)
             VALUES (?1, ?2, ?3, ?4)",
            params![id, name, compartment, endpoint],
        )?;

        Ok(OciQueue {
            id,
            name: name.to_string(),
            compartment_id: compartment.to_string(),
            messages_endpoint: endpoint,
        })
    }
}
