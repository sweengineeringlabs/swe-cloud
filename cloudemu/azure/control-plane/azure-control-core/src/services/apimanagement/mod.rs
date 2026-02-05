use azure_data_core::StorageEngine;
use std::sync::Arc;
use azure_control_spi::{CloudResult, Request, Response, CloudError};
use serde_json::{json, Value};

pub struct ApiManagementService {
    storage: Arc<StorageEngine>,
}

impl ApiManagementService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        if req.path.contains("/providers/Microsoft.ApiManagement/service/") {
            if req.path.contains("/apis") {
                 let parts: Vec<&str> = req.path.split("/apis").collect();
                 let base_path = parts[0];
                 let service_name = base_path.split('/').last().unwrap_or("");
                 let rg_parts: Vec<&str> = base_path.split("/resourceGroups/").collect();
                 let rg = rg_parts.get(1).and_then(|s| s.split('/').next()).unwrap_or("");

                 if req.method == "GET" {
                     return self.list_apis(service_name, rg).await;
                 } else if req.method == "PUT" {
                     return self.create_api(service_name, rg, &req).await;
                 }
            } else if req.method == "PUT" {
                return self.create_service(&req).await;
            }
        }
        
        Ok(Response::not_found("Api Management Service Not Found"))
    }

    async fn create_service(&self, req: &Request) -> CloudResult<Response> {
        let parts: Vec<&str> = req.path.split('/').collect();
        let name = parts.last().unwrap_or(&"");
        let rg_parts: Vec<&str> = req.path.split("/resourceGroups/").collect();
        let rg = rg_parts.get(1).and_then(|s| s.split('/').next()).unwrap_or("");

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let location = body["location"].as_str().unwrap_or("eastus");
        let pub_name = body["properties"]["publisherName"].as_str().unwrap_or("Admin");
        let pub_email = body["properties"]["publisherEmail"].as_str().unwrap_or("admin@example.com");
        let sku = body["sku"]["name"].as_str().unwrap_or("Developer");

        let service = self.storage.create_api_service(name, rg, location, pub_name, pub_email, sku)
            .map_err(|e| CloudError::Internal(e.to_string()))?;

        Ok(Response::ok(json!({
            "id": req.path,
            "name": service.name,
            "location": service.location,
            "properties": {
                "provisioningState": service.provisioning_state,
                "gatewayUrl": service.gateway_url
            }
        }).to_string()).with_header("Content-Type", "application/json"))
    }

    async fn create_api(&self, service_name: &str, rg: &str, req: &Request) -> CloudResult<Response> {
        let parts: Vec<&str> = req.path.split("/apis/").collect();
        let api_name = parts.get(1).unwrap_or(&"");
        
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let path = body["properties"]["path"].as_str().unwrap_or("");

        let api = self.storage.create_api(api_name, service_name, rg, path)
            .map_err(|e| CloudError::Internal(e.to_string()))?;

        Ok(Response::ok(json!({
            "id": req.path,
            "name": api.name,
            "properties": {
                "path": api.path,
                "protocols": ["https"]
            }
        }).to_string()).with_header("Content-Type", "application/json"))
    }

    async fn list_apis(&self, _service_name: &str, _rg: &str) -> CloudResult<Response> {
        // Simplified: return empty list for now or dummy
        Ok(Response::ok(json!({ "value": [] }).to_string()).with_header("Content-Type", "application/json"))
    }
}
