use azure_control_spi::{Request, Response, CloudResult, CloudError};
use azure_data_core::storage::StorageEngine;
use std::sync::Arc;

/// Azure Cosmos DB Service Handler (SQL API emulation)
pub struct CosmosService {
    engine: Arc<StorageEngine>,
}

impl CosmosService {
    pub fn new(engine: Arc<StorageEngine>) -> Self {
        Self { engine }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        let path = req.path.trim_start_matches('/');
        let parts: Vec<&str> = path.split('/').collect();

        if parts.is_empty() {
             return Ok(Response::ok("Cosmos DB Emulator Running"));
        }

        if parts[0] == "dbs" {
             if parts.len() == 1 {
                 if req.method == "GET" {
                     return self.list_databases().await;
                 } else if req.method == "POST" {
                     return self.create_database(&req.body).await;
                 }
             } else {
                 let db_req = parts[1];
                 if parts.len() == 2 {
                      match req.method.as_str() {
                          "GET" => return self.get_database(db_req).await,
                          "DELETE" => return self.delete_database(db_req).await,
                          _ => {}
                      }
                 }
                 
                 if parts.len() > 2 && parts[2] == "colls" {
                     if parts.len() == 3 {
                         match req.method.as_str() {
                             "GET" => return self.list_collections(db_req).await,
                             "POST" => return self.create_collection(db_req, &req.body).await,
                             _ => {}
                         }
                     } else {
                         let coll_req = parts[3];
                         if parts.len() == 4 {
                              match req.method.as_str() {
                                  "GET" => return self.get_collection(db_req, coll_req).await,
                                  "DELETE" => return self.delete_collection(db_req, coll_req).await,
                                  _ => {}
                              }
                         }
                         
                         if parts.len() > 4 && parts[4] == "docs" {
                             match req.method.as_str() {
                                 "GET" => return self.list_documents(db_req, coll_req).await,
                                 "POST" => return self.create_document(db_req, coll_req, &req.body).await,
                                 _ => {}
                             }
                         }
                     }
                 }
             }
        }

        Err(CloudError::Validation(format!("Unsupported Cosmos operation: {} {}", req.method, req.path)))
    }

    async fn list_databases(&self) -> CloudResult<Response> {
        let json = r#"{"_rid":"","Databases":[],"_count":0}"#;
        Ok(Response::ok(json).with_header("Content-Type", "application/json"))
    }

    async fn create_database(&self, _body: &[u8]) -> CloudResult<Response> {
        // Cosmos DBs are logical namespaces. We might not need to physical storage
        // unless we want to track them.
        Ok(Response::created(r#"{"id":"db1"}"#).with_header("Content-Type", "application/json"))
    }

    async fn get_database(&self, _db: &str) -> CloudResult<Response> {
        Ok(Response::ok(r#"{"id":"db1"}"#).with_header("Content-Type", "application/json"))
    }

    async fn delete_database(&self, _db: &str) -> CloudResult<Response> {
        Ok(Response::no_content())
    }

    async fn list_collections(&self, _db: &str) -> CloudResult<Response> {
         let json = r#"{"_rid":"","DocumentCollections":[],"_count":0}"#;
         Ok(Response::ok(json).with_header("Content-Type", "application/json"))
    }

    async fn create_collection(&self, db: &str, _body: &[u8]) -> CloudResult<Response> {
        // Create underlying DynamoDB table for the collection
        // Table Name: db_coll
        let coll_id = "coll1"; // TODO: Parse from body
        let table_name = format!("{}_{}", db, coll_id);
        
        self.engine.create_table(&table_name, "{}", "{}", "azure", "local")
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        Ok(Response::created(r#"{"id":"coll1"}"#).with_header("Content-Type", "application/json"))
    }

    async fn get_collection(&self, db: &str, coll: &str) -> CloudResult<Response> {
        let table_name = format!("{}_{}", db, coll);
        let _table = self.engine.get_table(&table_name)
            .map_err(|e| CloudError::NotFound { resource_type: "Collection".into(), resource_id: coll.into() })?;
            
        Ok(Response::ok(r#"{"id":"coll1"}"#).with_header("Content-Type", "application/json"))
    }

    async fn delete_collection(&self, _db: &str, _coll: &str) -> CloudResult<Response> {
        Ok(Response::no_content())
    }

    async fn list_documents(&self, db: &str, coll: &str) -> CloudResult<Response> {
        let table_name = format!("{}_{}", db, coll);
        // Scan items (DynamoDB scan)
        let items = self.engine.scan_items(&table_name)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        // Convert items to Cosmos response format
        // Simplified: just dump them
        let json = format!(r#"{{"_rid":"","Documents":{:?},"_count":{}}}"#, items, items.len());
        Ok(Response::ok(json).with_header("Content-Type", "application/json"))
    }
    
    async fn create_document(&self, db: &str, coll: &str, body: &[u8]) -> CloudResult<Response> {
        let table_name = format!("{}_{}", db, coll);
        let doc_body = String::from_utf8(body.to_vec()).unwrap_or_default();
        
        // Use random ID for partition key if not present
        let id = "doc1"; // TODO extract ID
        
        self.engine.put_item(&table_name, id, None, &doc_body)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
            
        Ok(Response::created(r#"{"id":"doc1"}"#).with_header("Content-Type", "application/json"))
    }
}
