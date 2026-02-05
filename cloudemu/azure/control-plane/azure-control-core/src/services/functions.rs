use azure_control_spi::{Request, Response, CloudResult, CloudError};

use azure_data_core::storage::StorageEngine;
use std::sync::Arc;

/// Azure Functions Handler
pub struct FunctionsService {
    engine: Arc<StorageEngine>,
}

impl FunctionsService {
    pub fn new(engine: Arc<StorageEngine>) -> Self {
        Self { engine }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // Path: /api/{function} or /admin/functions/{function}
        
        let path = req.path.trim_start_matches('/');
        let parts: Vec<&str> = path.split('/').collect();

        if parts.is_empty() {
             return Ok(Response::ok("Azure Functions Emulator"));
        }

        if parts[0] == "api" && parts.len() > 1 {
            let func_name = parts[1];
            return self.invoke_function(func_name, &req.body).await;
        }

        if parts[0] == "admin" && parts.len() > 2 && parts[1] == "functions" {
            let func_name = parts[2];
            if req.method == "PUT" || req.method == "POST" {
                return self.create_function(func_name).await;
            }
        }

        Err(CloudError::Validation("Unknown Function operation".into()))
    }

    async fn create_function(&self, name: &str) -> CloudResult<Response> {
        use azure_data_core::storage::CreateFunctionParams;
        
        let params = CreateFunctionParams {
            name,
            runtime: "nodejs14.x",
            role: "arn:azure:iam::azure:role/default",
            handler: "index.handler",
            code_hash: "default",
            account_id: "azure",
            region: "local",
        };
        
        let func = self.engine.create_function(params)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        Ok(Response::created(format!(r#"{{"functionName":"{}","arn":"{}"}}"#, func.name, func.arn)))
    }

    async fn invoke_function(&self, name: &str, body: &[u8]) -> CloudResult<Response> {
        // Verify function exists
        let _func = self.engine.get_function(name)
            .map_err(|_| CloudError::NotFound { resource_type: "Function".into(), resource_id: name.into() })?;
        
        let body_str = String::from_utf8_lossy(body);
        let resp = format!(r#"{{"result":"Function '{}' executed","input":{}}}"#, name, body_str);
        Ok(Response::ok(resp))
    }
}
