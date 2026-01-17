use gcp_data_core::storage::StorageEngine;
use gcp_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;
use gcp_data_core::storage::GcpSqlInstance;

pub struct SqlService {
    storage: Arc<StorageEngine>,
}

impl SqlService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // POST /sql/v1beta4/projects/{project}/instances
        if req.method == "POST" && req.path.contains("/instances") {
            return self.insert_instance(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn insert_instance(&self, req: &Request) -> CloudResult<Response> {
        let parts: Vec<&str> = req.path.split('/').collect();
        let project_idx = parts.iter().position(|&x| x == "projects").unwrap() + 1;
        let project = parts[project_idx];

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let name = body["name"].as_str().unwrap_or("sql-1");
        let tier = body["settings"]["tier"].as_str().unwrap_or("db-n1-standard-1");
        let region = body["region"].as_str().unwrap_or("us-central1");

        let db = GcpSqlInstance {
            name: name.to_string(),
            project: project.to_string(),
            region: region.to_string(),
            tier: tier.to_string(),
            state: "RUNNABLE".to_string(),
        };

        self.storage.insert_sql_instance(db).map_err(|e| gcp_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "kind": "sql#operation",
            "status": "DONE"
        })))
    }
}
