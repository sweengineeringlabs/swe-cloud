use gcp_control_spi::{Request, Response, CloudResult, CloudError};
use gcp_data_core::storage::StorageEngine;
use std::sync::Arc;

/// GCP Cloud Functions Handler
pub struct CloudFunctionsService {
    engine: Arc<StorageEngine>,
}

impl CloudFunctionsService {
    pub fn new(engine: Arc<StorageEngine>) -> Self {
        Self { engine }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        let path = req.path.trim_start_matches('/');
        let parts: Vec<&str> = path.split('/').collect();

        if parts.is_empty() {
            return Ok(Response::ok("Cloud Functions Emulator"));
        }

        // Paths: /v1/projects/{project}/locations/{location}/functions/{function}
        if parts.len() >= 6 && parts[0] == "v1" && parts[1] == "projects" {
            if parts[3] == "locations" && parts[5] == "functions" {
                if parts.len() == 6 {
                    // List functions
                    return self.list_functions().await;
                } else {
                    let function_name = parts[6];
                    if parts.len() == 7 {
                        match req.method.as_str() {
                            "POST" => return self.create_function(function_name).await,
                            "GET" => return self.get_function(function_name).await,
                            "DELETE" => return self.delete_function(function_name).await,
                            _ => {}
                        }
                    } else if parts.len() == 8 && parts[7] == "call" {
                        return self.invoke_function(function_name, &req.body).await;
                    }
                }
            }
        }

        Err(CloudError::Validation(format!("Unsupported Cloud Functions operation: {} {}", req.method, req.path)))
    }

    async fn create_function(&self, name: &str) -> CloudResult<Response> {
        use gcp_data_core::storage::CreateFunctionParams;
        
        let params = CreateFunctionParams {
            name,
            runtime: "nodejs14",
            role: "arn:gcp:iam::gcp:role/default",
            handler: "index.handler",
            code_hash: "default",
            account_id: "gcp",
            region: "local",
        };
        
        let func = self.engine.create_function(params)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        Ok(Response::created(format!(r#"{{"name":"{}"}}"#, func.name)))
    }

    async fn get_function(&self, name: &str) -> CloudResult<Response> {
        let func = self.engine.get_function(name)
            .map_err(|_| CloudError::NotFound { resource_type: "Function".into(), resource_id: name.into() })?;
            
        Ok(Response::ok(format!(r#"{{"name":"{}","runtime":"{}"}}"#, func.name, func.runtime)))
    }

    async fn delete_function(&self, _name: &str) -> CloudResult<Response> {
        Ok(Response::no_content())
    }

    async fn list_functions(&self) -> CloudResult<Response> {
        Ok(Response::ok(r#"{"functions":[]}"#))
    }

    async fn invoke_function(&self, name: &str, body: &[u8]) -> CloudResult<Response> {
        // Verify function exists
        let _func = self.engine.get_function(name)
            .map_err(|_| CloudError::NotFound { resource_type: "Function".into(), resource_id: name.into() })?;
        
        let input = String::from_utf8_lossy(body);
        Ok(Response::ok(format!(r#"{{"result":"Function executed","input":{}}}"#, input)))
    }
}
