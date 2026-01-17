use oracle_data_core::StorageEngine;
use oracle_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct VaultService {
    storage: Arc<StorageEngine>,
}

impl VaultService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // /20180608/secrets
        if req.path.contains("/secrets") && req.method == "POST" {
            return self.create_secret(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_secret(&self, req: &Request) -> CloudResult<Response> {
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let name = body["secretName"].as_str().unwrap_or("secret1");
        let vault_id = body["vaultId"].as_str().unwrap_or("ocid1.vault.oc1..test");
        let compartment = body["compartmentId"].as_str().unwrap_or("ocid1.compartment.oc1..test");
        let content = body["secretContent"]["content"].as_str().unwrap_or(""); // Base64

        let secret = self.storage.create_secret(name, vault_id, compartment, content).map_err(|e| oracle_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "id": secret.id,
            "secretName": secret.secret_name,
            "vaultId": secret.vault_id,
            "lifecycleState": secret.state
        })))
    }
}
