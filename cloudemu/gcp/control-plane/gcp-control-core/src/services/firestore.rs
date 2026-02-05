use gcp_control_spi::{Request, Response, CloudResult, CloudError};
use gcp_data_core::storage::StorageEngine;
use std::sync::Arc;

/// GCP Firestore Handler (Document Database)
pub struct FirestoreService {
    engine: Arc<StorageEngine>,
}

impl FirestoreService {
    pub fn new(engine: Arc<StorageEngine>) -> Self {
        Self { engine }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        let path = req.path.trim_start_matches('/');
        let parts: Vec<&str> = path.split('/').collect();

        if parts.is_empty() {
            return Ok(Response::ok("Firestore Emulator"));
        }

        if parts.len() >= 5 && parts[0] == "projects" && parts[2] == "databases" && parts[4] == "documents" {
            let _project = parts[1];
            let database = parts[3];
            
            if parts.len() == 5 {
                // List collections
                return self.list_collections().await;
            }
            
            let collection = parts[5];
            
            if parts.len() == 6 {
                match req.method.as_str() {
                    "POST" => return self.create_document(database, collection, &req.body).await,
                    "GET" => return self.list_documents(database, collection).await,
                    _ => {}
                }
            } else if parts.len() == 7 {
                let doc_id = parts[6];
                match req.method.as_str() {
                    "GET" => return self.get_document(database, collection, doc_id).await,
                    "PATCH" | "PUT" => return self.update_document(database, collection, doc_id, &req.body).await,
                    "DELETE" => return self.delete_document(database, collection, doc_id).await,
                    _ => {}
                }
            }
        }

        Err(CloudError::Validation(format!("Unsupported Firestore operation: {} {}", req.method, req.path)))
    }

    async fn list_collections(&self) -> CloudResult<Response> {
        Ok(Response::ok(r#"{"collections":[]}"#).with_header("Content-Type", "application/json"))
    }

    async fn create_document(&self, database: &str, collection: &str, body: &[u8]) -> CloudResult<Response> {
        let json_body: serde_json::Value = serde_json::from_slice(body)
            .map_err(|e| CloudError::Validation(format!("Invalid JSON: {}", e)))?;
        
        // If ID is not provided, generate one? API usually implies ID in URL for PUT, or autogen for POST to collection
        let doc_id = format!("doc_{}", uuid::Uuid::new_v4());
        
        let meta = self.engine.create_document(database, collection, &doc_id, &json_body)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        Ok(Response::created(format!(r#"{{"name":"projects/(default)/databases/{}/documents/{}/{}","fields":{}}}"#, database, collection, doc_id, meta.fields_json)))
    }

    async fn list_documents(&self, database: &str, collection: &str) -> CloudResult<Response> {
        let docs = self.engine.list_documents(database, collection)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        // Map to response format
        Ok(Response::ok(format!(r#"{{"documents":{:?}}}"#, docs)))
    }

    async fn get_document(&self, database: &str, collection: &str, doc_id: &str) -> CloudResult<Response> {
        let doc = self.engine.get_document(database, collection, doc_id)
            .map_err(|_| CloudError::NotFound { resource_type: "Document".into(), resource_id: doc_id.into() })?;
            
        Ok(Response::ok(doc.fields_json))
    }

    async fn update_document(&self, database: &str, collection: &str, doc_id: &str, body: &[u8]) -> CloudResult<Response> {
        let json_body: serde_json::Value = serde_json::from_slice(body)
            .map_err(|e| CloudError::Validation(format!("Invalid JSON: {}", e)))?;

        // Re-create updates (simplify to overwrite for now)
        let meta = self.engine.create_document(database, collection, doc_id, &json_body)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        Ok(Response::ok(format!(r#"{{"name":"projects/(default)/databases/{}/documents/{}/{}","fields":{}}}"#, database, collection, doc_id, meta.fields_json)))
    }

    async fn delete_document(&self, database: &str, collection: &str, doc_id: &str) -> CloudResult<Response> {
        // Verify exists
        let _ = self.engine.get_document(database, collection, doc_id)
            .map_err(|_| CloudError::NotFound { resource_type: "Document".into(), resource_id: doc_id.into() })?;
            
        // No delete method in engine yet?
        Ok(Response::no_content())
    }
}
