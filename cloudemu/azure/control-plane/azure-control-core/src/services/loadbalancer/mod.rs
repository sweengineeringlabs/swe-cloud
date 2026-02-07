use azure_data_core::StorageEngine;
use std::sync::Arc;
use azure_control_spi::{CloudResult, Request, Response, CloudError};
use serde_json::{json, Value};

pub struct LoadBalancerService {
    storage: Arc<StorageEngine>,
}

impl LoadBalancerService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // .../providers/Microsoft.Network/loadBalancers/{name}
        if req.path.contains("/providers/Microsoft.Network/loadBalancers/") && req.method == "PUT" {
            return self.create_lb(&req).await;
        }
        
        Ok(Response::not_found("Load Balancer Service Not Found"))
    }

    async fn create_lb(&self, req: &Request) -> CloudResult<Response> {
        let parts: Vec<&str> = req.path.split('/').collect();
        let name = parts.last().unwrap_or(&"");
        let rg_parts: Vec<&str> = req.path.split("/resourceGroups/").collect();
        let rg = rg_parts.get(1).and_then(|s| s.split('/').next()).unwrap_or("");
        
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let location = body["location"].as_str().unwrap_or("eastus");
        let sku = body["sku"]["name"].as_str().unwrap_or("Basic");

        let lb = self.storage.create_load_balancer(name, rg, location, sku)
            .map_err(|e| CloudError::Internal(e.to_string()))?;

        Ok(Response::ok(json!({
            "id": req.path,
            "name": lb.name,
            "location": lb.location,
            "sku": { "name": lb.sku },
            "properties": {
                "provisioningState": lb.provisioning_state
            }
        }).to_string()).with_header("Content-Type", "application/json"))
    }
}
