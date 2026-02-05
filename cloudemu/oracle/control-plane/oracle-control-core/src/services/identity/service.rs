use oracle_data_core::StorageEngine;
use oracle_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct IdentityService {
    storage: Arc<StorageEngine>,
}

impl IdentityService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // POST /20160918/users
        if req.path.ends_with("/users") && req.method == "POST" {
            return self.create_user(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_user(&self, req: &Request) -> CloudResult<Response> {
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let name = body["name"].as_str().unwrap_or("user1");
        let desc = body["description"].as_str().unwrap_or("");

        let user = self.storage.create_user(name, desc).map_err(|e| oracle_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "id": user.id,
            "name": user.name,
            "description": user.description,
            "lifecycleState": user.lifecycle_state
        })))
    }
}
