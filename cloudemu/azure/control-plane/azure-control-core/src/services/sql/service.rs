use azure_data_core::storage::StorageEngine;
use azure_data_core::storage::AzureSqlDatabase;
use azure_control_spi::{Request, Response, CloudResult, CloudError};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct SqlService {
    storage: Arc<StorageEngine>,
}

impl SqlService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // PUT .../servers/{server}/databases/{db}
        if req.method == "PUT" && req.path.contains("databases") {
            return self.create_db(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_db(&self, req: &Request) -> CloudResult<Response> {
        let parts: Vec<&str> = req.path.split('/').collect();
        let db_idx = parts.iter().position(|&x| x == "databases").unwrap() + 1;
        let db_name = parts[db_idx];
        let srv_idx = parts.iter().position(|&x| x == "servers").unwrap() + 1;
        let server = parts[srv_idx];
        let rg_idx = parts.iter().position(|&x| x == "resourceGroups").unwrap() + 1;
        let rg = parts[rg_idx];

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let location = body["location"].as_str().unwrap_or("eastus");
        let sku = body["sku"]["name"].as_str().unwrap_or("Standard");

        let db = AzureSqlDatabase {
            name: db_name.to_string(),
            server_name: server.to_string(),
            resource_group: rg.to_string(),
            location: location.to_string(),
            sku: sku.to_string(),
            status: "Online".to_string(),
        };

        self.storage.create_sql_db(db).map_err(|e| CloudError::Internal(e.to_string()))?;

        Ok(Response::ok(json!({
            "name": db_name,
            "status": "Online"
        }).to_string()).with_header("Content-Type", "application/json"))
    }
}
