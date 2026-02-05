use gcp_data_core::storage::StorageEngine;
use gcp_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct IamService {
    storage: Arc<StorageEngine>,
}

impl IamService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // POST /v1/projects/{project}/serviceAccounts
        if req.path.contains("/serviceAccounts") && req.method == "POST" {
            return self.create_service_account(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_service_account(&self, req: &Request) -> CloudResult<Response> {
        let parts: Vec<&str> = req.path.split('/').collect();
        let project_idx = parts.iter().position(|&x| x == "projects").unwrap() + 1;
        let project = parts[project_idx];

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let account_id = body["accountId"].as_str().unwrap_or("sa1");
        let display_name = body["serviceAccount"]["displayName"].as_str().unwrap_or(account_id);

        let sa = self.storage.create_service_account(project, account_id, display_name).map_err(|e| gcp_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "name": sa.name,
            "projectId": sa.project_id,
            "uniqueId": sa.unique_id,
            "email": sa.email,
            "displayName": sa.display_name
        })))
    }
}
