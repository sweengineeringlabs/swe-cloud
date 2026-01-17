use azure_data_core::StorageEngine;
use std::sync::Arc;
use azure_control_spi::{CloudResult, Request, Response, CloudError};
use serde_json::{json, Value};

pub struct AcrService {
    storage: Arc<StorageEngine>,
}

impl AcrService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // .../providers/Microsoft.ContainerRegistry/registries/{name}
        if req.path.contains("/providers/Microsoft.ContainerRegistry/registries/") && req.method == "PUT" {
            return self.create_registry(&req).await;
        }
        
        Ok(Response::not_found("ACR Service Not Found"))
    }

    async fn create_registry(&self, req: &Request) -> CloudResult<Response> {
        let parts: Vec<&str> = req.path.split('/').collect();
        let name = parts.last().unwrap_or(&"");
        let rg_parts: Vec<&str> = req.path.split("/resourceGroups/").collect();
        let rg = rg_parts.get(1).and_then(|s| s.split('/').next()).unwrap_or("");
        
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let location = body["location"].as_str().unwrap_or("eastus");
        let sku = body["sku"]["name"].as_str().unwrap_or("Basic");
        let admin_enabled = body["properties"]["adminUserEnabled"].as_bool().unwrap_or(false);

        let acr = self.storage.create_registry(name, rg, location, sku, admin_enabled)
            .map_err(|e| CloudError::Internal(e.to_string()))?;

        Ok(Response::ok(json!({
            "id": req.path,
            "name": acr.name,
            "location": acr.location,
            "sku": { "name": acr.sku },
            "properties": {
                "provisioningState": acr.provisioning_state,
                "loginServer": acr.login_server,
                "adminUserEnabled": acr.admin_user_enabled
            }
        }).to_string()).with_header("Content-Type", "application/json"))
    }
}
