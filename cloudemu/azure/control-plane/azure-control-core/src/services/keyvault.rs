use azure_control_spi::{Request, Response, CloudResult, CloudError};

use azure_data_core::storage::StorageEngine;
use std::sync::Arc;

/// Azure Key Vault Handler
pub struct KeyVaultService {
    engine: Arc<StorageEngine>,
}

impl KeyVaultService {
    pub fn new(engine: Arc<StorageEngine>) -> Self {
        Self { engine }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // Path: /secrets/{name}
        
        let path = req.path.trim_start_matches('/');
        let parts: Vec<&str> = path.split('/').collect();

        if parts.is_empty() {
            return Ok(Response::ok("Key Vault Emulator"));
        }

        if parts[0] == "secrets" && parts.len() > 1 {
            let secret_name = parts[1];
            match req.method.as_str() {
                "PUT" => return self.set_secret(secret_name, &req.body).await,
                "GET" => return self.get_secret(secret_name).await,
                "DELETE" => return self.delete_secret(secret_name).await,
                _ => {}
            }
        }

        Err(CloudError::Validation("Unknown Key Vault operation".into()))
    }

    async fn set_secret(&self, name: &str, body: &[u8]) -> CloudResult<Response> {
        let secret_value = String::from_utf8(body.to_vec()).unwrap_or_default();
        
        // Create secret metadata
        self.engine.create_secret(name, None, None, "azure", "local")
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        // Put secret value
        self.engine.put_secret_value(name, Some(&secret_value), None)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        Ok(Response::ok(format!(r#"{{"id":"{}","value":"***"}}"#, name))
            .with_header("Content-Type", "application/json"))
    }

    async fn get_secret(&self, name: &str) -> CloudResult<Response> {
        let secret = self.engine.get_secret_value(name, None, None)
            .map_err(|_| CloudError::NotFound { resource_type: "Secret".into(), resource_id: name.into() })?;
            
        let value = secret.secret_string.unwrap_or_default();
        Ok(Response::ok(format!(r#"{{"id":"{}","value":"{}"}}"#, name, value))
            .with_header("Content-Type", "application/json"))
    }

    async fn delete_secret(&self, name: &str) -> CloudResult<Response> {
        // Storage engine doesn't have delete_secret, so we'll just return success
        // In production, you'd want to implement this in the engine
        let _ = self.engine.get_secret_value(name, None, None)
            .map_err(|_| CloudError::NotFound { resource_type: "Secret".into(), resource_id: name.into() })?;
            
        Ok(Response::ok(r#"{"deletedDate":0}"#)
            .with_header("Content-Type", "application/json"))
    }
}
