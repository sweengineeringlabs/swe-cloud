use gcp_control_spi::{Request, Response, CloudResult, CloudError};
use gcp_data_core::storage::StorageEngine;
use std::sync::Arc;

/// GCP Secret Manager Handler
pub struct SecretManagerService {
    engine: Arc<StorageEngine>,
}

impl SecretManagerService {
    pub fn new(engine: Arc<StorageEngine>) -> Self {
        Self { engine }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        let path = req.path.trim_start_matches('/');
        let parts: Vec<&str> = path.split('/').collect();

        if parts.is_empty() {
            return Ok(Response::ok("Secret Manager Emulator"));
        }

        // Paths: /v1/projects/{project}/secrets/{secret}
        // or /v1/projects/{project}/secrets/{secret}/versions/latest
        if parts.len() >= 4 && parts[0] == "v1" && parts[1] == "projects" && parts[3] == "secrets" {
            if parts.len() == 4 {
                // List secrets
                return self.list_secrets().await;
            }
            
            let secret_name = parts[4];
            
            if parts.len() == 5 {
                match req.method.as_str() {
                    "POST" => return self.create_secret(secret_name).await,
                    "DELETE" => return self.delete_secret(secret_name).await,
                    _ => {}
                }
            } else if parts.len() == 7 && parts[5] == "versions" {
                if parts[6] == "latest" && req.method == "GET" {
                    return self.access_secret_version(secret_name).await;
                } else if req.method == "POST" {
                    return self.add_secret_version(secret_name, &req.body).await;
                }
            }
        }

        Err(CloudError::Validation(format!("Unsupported Secret Manager operation: {} {}", req.method, req.path)))
    }

    async fn create_secret(&self, name: &str) -> CloudResult<Response> {
        self.engine.create_secret(name, None, None, "gcp", "local")
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        Ok(Response::created(format!(r#"{{"name":"{}"}}"#, name)))
    }

    async fn add_secret_version(&self, name: &str, body: &[u8]) -> CloudResult<Response> {
        let secret_value = String::from_utf8(body.to_vec()).unwrap_or_default();
        
        self.engine.put_secret_value(name, Some(&secret_value), None)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        Ok(Response::created(format!(r#"{{"name":"{}"}}"#, name)))
    }

    async fn access_secret_version(&self, name: &str) -> CloudResult<Response> {
        let secret = self.engine.get_secret_value(name, None, None)
            .map_err(|_| CloudError::NotFound { resource_type: "Secret".into(), resource_id: name.into() })?;
            
        let value = secret.secret_string.unwrap_or_default();
        Ok(Response::ok(format!(r#"{{"name":"{}","payload":{{"data":"{}"}}}}"#, name, value)))
    }

    async fn delete_secret(&self, name: &str) -> CloudResult<Response> {
        // Verify exists
        let _ = self.engine.get_secret_value(name, None, None)
            .map_err(|_| CloudError::NotFound { resource_type: "Secret".into(), resource_id: name.into() })?;
            
        Ok(Response::no_content())
    }

    async fn list_secrets(&self) -> CloudResult<Response> {
        Ok(Response::ok(r#"{"secrets":[]}"#))
    }
}
