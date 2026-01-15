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

        // Firestore paths: /projects/{project}/databases/{database}/documents/{collection}/{doc}
        if parts.len() >= 5 && parts[0] == "projects" && parts[2] == "databases" && parts[4] == "documents" {
            if parts.len() == 5 {
                // List collections
                return self.list_collections().await;
            }
            
            let collection = parts[5];
            
            if parts.len() == 6 {
                match req.method.as_str() {
                    "POST" => return self.create_document(collection, &req.body).await,
                    "GET" => return self.list_documents(collection).await,
                    _ => {}
                }
            } else if parts.len() == 7 {
                let doc_id = parts[6];
                match req.method.as_str() {
                    "GET" => return self.get_document(collection, doc_id).await,
                    "PATCH" | "PUT" => return self.update_document(collection, doc_id, &req.body).await,
                    "DELETE" => return self.delete_document(collection, doc_id).await,
                    _ => {}
                }
            }
        }

        Err(CloudError::Validation(format!("Unsupported Firestore operation: {} {}", req.method, req.path)))
    }

    async fn list_collections(&self) -> CloudResult<Response> {
        Ok(Response::ok(r#"{"collections":[]}"#).with_header("Content-Type", "application/json"))
    }

    async fn create_document(&self, collection: &str, body: &[u8]) -> CloudResult<Response> {
        let doc_body = String::from_utf8(body.to_vec()).unwrap_or_default();
        let doc_id = format!("doc_{}", uuid::Uuid::new_v4());
        
        // Use DynamoDB table to store documents
        let table_name = format!("firestore_{}", collection);
        
        // Create table if it doesn't exist
        let _ = self.engine.create_table(&table_name, "{}", "{}", "gcp", "local");
        
        self.engine.put_item(&table_name, &doc_id, None, &doc_body)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        Ok(Response::created(format!(r#"{{"name":"{}","id":"{}"}}"#, collection, doc_id)))
    }

    async fn list_documents(&self, collection: &str) -> CloudResult<Response> {
        let table_name = format!("firestore_{}", collection);
        let items = self.engine.scan_items(&table_name)
            .unwrap_or_default();
            
        Ok(Response::ok(format!(r#"{{"documents":{:?}}}"#, items)))
    }

    async fn get_document(&self, collection: &str, doc_id: &str) -> CloudResult<Response> {
        let table_name = format!("firestore_{}", collection);
        let doc = self.engine.get_item(&table_name, doc_id, None)
            .map_err(|_| CloudError::NotFound { resource_type: "Document".into(), resource_id: doc_id.into() })?;
            
        Ok(Response::ok(doc.unwrap_or_default()))
    }

    async fn update_document(&self, collection: &str, doc_id: &str, body: &[u8]) -> CloudResult<Response> {
        let doc_body = String::from_utf8(body.to_vec()).unwrap_or_default();
        let table_name = format!("firestore_{}", collection);
        
        self.engine.put_item(&table_name, doc_id, None, &doc_body)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        Ok(Response::ok(format!(r#"{{"id":"{}"}}"#, doc_id)))
    }

    async fn delete_document(&self, collection: &str, doc_id: &str) -> CloudResult<Response> {
        // Verify exists
        let table_name = format!("firestore_{}", collection);
        let _ = self.engine.get_item(&table_name, doc_id, None)
            .map_err(|_| CloudError::NotFound { resource_type: "Document".into(), resource_id: doc_id.into() })?;
            
        Ok(Response::no_content())
    }
}
