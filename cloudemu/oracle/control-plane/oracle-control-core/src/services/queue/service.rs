use oracle_data_core::StorageEngine;
use oracle_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct QueueService {
    storage: Arc<StorageEngine>,
}

impl QueueService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // /20210201/queues
        if req.path.contains("/queues") && req.method == "POST" {
            return self.create_queue(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_queue(&self, req: &Request) -> CloudResult<Response> {
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let name = body["displayName"].as_str().unwrap_or("queue1");
        let compartment = body["compartmentId"].as_str().unwrap_or("ocid1.compartment.oc1..test");

        let queue = self.storage.create_queue(name, compartment).map_err(|e| oracle_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "id": queue.id,
            "displayName": queue.name,
            "compartmentId": queue.compartment_id,
            "messagesEndpoint": queue.messages_endpoint,
            "lifecycleState": "ACTIVE"
        })))
    }
}
