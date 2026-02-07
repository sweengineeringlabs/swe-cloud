use azure_data_core::storage::StorageEngine;
use azure_control_spi::{Request, Response, CloudResult, CloudError};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct LogicAppsService {
    storage: Arc<StorageEngine>,
}

impl LogicAppsService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // PUT /subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.Logic/workflows/{workflowName}
        if req.path.contains("/providers/Microsoft.Logic/workflows/") && req.method == "PUT" {
            return self.create_workflow(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_workflow(&self, req: &Request) -> CloudResult<Response> {
        let parts: Vec<&str> = req.path.split('/').collect();
        // .../resourceGroups/{rg}/.../workflows/{name}
        let rg_idx = parts.iter().position(|&x| x == "resourceGroups").unwrap();
        let resource_group = parts[rg_idx + 1];
        let wf_idx = parts.iter().position(|&x| x == "workflows").unwrap();
        let name = parts[wf_idx + 1];

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let location = body["location"].as_str().unwrap_or("westus");
        let definition = body["properties"]["definition"].to_string();

        let app = self.storage.create_logic_app(name, resource_group, location, &definition).map_err(|e| CloudError::Internal(e.to_string()))?;

        Ok(Response::ok(json!({
            "id": format!("/subscriptions/sub1/resourceGroups/{}/providers/Microsoft.Logic/workflows/{}", resource_group, name),
            "name": app.name,
            "type": "Microsoft.Logic/workflows",
            "location": app.location,
            "properties": {
                "state": app.state,
                "createdTime": app.created_at
            }
        }).to_string()).with_header("Content-Type", "application/json"))
    }
}
