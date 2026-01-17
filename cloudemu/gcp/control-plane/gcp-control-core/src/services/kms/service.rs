use gcp_data_core::storage::StorageEngine;
use gcp_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct KmsService {
    storage: Arc<StorageEngine>,
}

impl KmsService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // v1/projects/{}/locations/{}/keyRings
        if req.path.contains("/keyRings") {
            if req.method == "POST" {
                if req.path.ends_with("/cryptoKeys") {
                    return self.create_crypto_key(&req);
                } else {
                    return self.create_key_ring(&req);
                }
            }
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_key_ring(&self, req: &Request) -> CloudResult<Response> {
        // /v1/projects/{}/locations/{}/keyRings?keyRingId={}
        let parts: Vec<&str> = req.path.split('/').collect();
        let p_idx = parts.iter().position(|&x| x == "projects").unwrap();
        let project = parts[p_idx + 1];
        let l_idx = parts.iter().position(|&x| x == "locations").unwrap();
        let location = parts[l_idx + 1];

        // Assuming keyRingId in query param or body. Let's look for id in query (mock)
        // or just expect it in path if it was .../keyRings/{id}
        // The REST API is POST .../keyRings?keyRingId=my-key-ring
        
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        // We'll fake reading query param since we don't have it easily
        let name = "keyring1"; 

        let ring = self.storage.create_key_ring(name, project, location).map_err(|e| gcp_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "name": format!("projects/{}/locations/{}/keyRings/{}", project, location, ring.name),
            "createTime": ring.created_at
        })))
    }

    fn create_crypto_key(&self, req: &Request) -> CloudResult<Response> {
        // .../keyRings/{keyRingId}/cryptoKeys?cryptoKeyId={}
        let parts: Vec<&str> = req.path.split('/').collect();
        let k_idx = parts.iter().position(|&x| x == "keyRings").unwrap();
        let key_ring = parts[k_idx + 1];

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let purpose = body["purpose"].as_str().unwrap_or("ENCRYPT_DECRYPT");
        let name = "key1"; // Mock ID

        let key = self.storage.create_crypto_key(name, key_ring, purpose).map_err(|e| gcp_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
             "name": format!(".../keyRings/{}/cryptoKeys/{}", key_ring, key.name),
             "purpose": key.purpose,
             "createTime": key.created_at
        })))
    }
}
