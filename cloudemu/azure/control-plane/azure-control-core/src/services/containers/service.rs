use azure_data_core::storage::StorageEngine;
use azure_control_spi::{Request, Response, CloudResult, CloudError};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct ContainerService {
    storage: Arc<StorageEngine>,
}

impl ContainerService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // /subscriptions/{sub}/resourceGroups/{rg}/providers/Microsoft.ContainerInstance/containerGroups/{name}
        if req.path.contains("/providers/Microsoft.ContainerInstance/containerGroups/") && req.method == "PUT" {
            return self.create_container_group(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_container_group(&self, req: &Request) -> CloudResult<Response> {
        let parts: Vec<&str> = req.path.split('/').collect();
        let rg_idx = parts.iter().position(|&x| x == "resourceGroups").unwrap();
        let resource_group = parts[rg_idx + 1];
        let n_idx = parts.iter().position(|&x| x == "containerGroups").unwrap();
        let name = parts[n_idx + 1];

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let location = body["location"].as_str().unwrap_or("westus");
        let os_type = body["properties"]["osType"].as_str().unwrap_or("Linux");

        let group = self.storage.create_container_group(name, resource_group, location, os_type).map_err(|e| CloudError::Internal(e.to_string()))?;

        Ok(Response::ok(json!({
            "name": group.name,
            "id": format!("/subscriptions/sub1/resourceGroups/{}/providers/Microsoft.ContainerInstance/containerGroups/{}", resource_group, name),
            "location": group.location,
            "properties": {
                "osType": group.os_type,
                "provisioningState": "Succeeded",
                "instanceView": {
                    "state": group.state
                }
            }
        }).to_string()).with_header("Content-Type", "application/json"))
    }
}
