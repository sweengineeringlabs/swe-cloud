use azure_data_core::storage::StorageEngine;
use azure_control_spi::{Request, Response, CloudResult, CloudError};
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
        // MS Graph API usually, but sticking to Azure ARM style for now
        if req.path.contains("roleAssignments") && req.method == "PUT" {
            return self.create_role_assignment(&req);
        }
        if req.path.contains("servicePrincipals") && req.method == "POST" {
            return self.create_sp(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_sp(&self, req: &Request) -> CloudResult<Response> {
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let display_name = body["displayName"].as_str().unwrap_or("sp-1");
        
        let sp = self.storage.create_service_principal(display_name).map_err(|e| CloudError::Internal(e.to_string()))?;

        Ok(Response::ok(json!({
            "id": sp.id,
            "appId": sp.app_id,
            "displayName": sp.display_name,
            "objectType": sp.object_type
        }).to_string()).with_header("Content-Type", "application/json"))
    }

    fn create_role_assignment(&self, req: &Request) -> CloudResult<Response> {
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let principal_id = body["properties"]["principalId"].as_str().unwrap_or("");
        let role_id = body["properties"]["roleDefinitionId"].as_str().unwrap_or("");
        
        let parts: Vec<&str> = req.path.split("/providers/Microsoft.Authorization").collect();
        let scope = parts[0]; 

        let ra = self.storage.create_role_assignment(scope, principal_id, role_id).map_err(|e| CloudError::Internal(e.to_string()))?;

        Ok(Response::ok(json!({
            "id": ra.id,
            "name": ra.name,
            "type": "Microsoft.Authorization/roleAssignments",
            "properties": {
                "scope": ra.scope,
                "principalId": ra.principal_id,
                "roleDefinitionId": ra.role_definition_id
            }
        }).to_string()).with_header("Content-Type", "application/json"))
    }
}
