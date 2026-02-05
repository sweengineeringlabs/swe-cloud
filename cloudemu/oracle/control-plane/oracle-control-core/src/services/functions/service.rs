use oracle_data_core::StorageEngine;
use oracle_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct FunctionsService {
    storage: Arc<StorageEngine>,
}

impl FunctionsService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // /20181201/functions
        if req.path.contains("/functions") && req.method == "POST" {
            return self.create_function(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_function(&self, req: &Request) -> CloudResult<Response> {
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let app_id = body["applicationId"].as_str().unwrap_or("app1");
        let display_name = body["displayName"].as_str().unwrap_or("func1");
        let image = body["image"].as_str().unwrap_or("image:latest");
        let memory = body["memoryInMBs"].as_i64().unwrap_or(128);

        let func = self.storage.create_function(app_id, display_name, image, memory).map_err(|e| oracle_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "id": func.id,
            "applicationId": func.application_id,
            "displayName": func.display_name,
            "image": func.image,
            "memoryInMBs": func.memory_in_mbs
        })))
    }
}
