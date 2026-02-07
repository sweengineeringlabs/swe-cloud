use oracle_data_core::StorageEngine;
use oracle_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;
use oracle_data_core::storage::OracleAutonomousDb;

pub struct DatabaseService {
    storage: Arc<StorageEngine>,
}

impl DatabaseService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // POST /20160918/autonomousDatabases
        if req.method == "POST" && req.path.ends_with("/autonomousDatabases") {
            return self.create_autonomous_database(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_autonomous_database(&self, req: &Request) -> CloudResult<Response> {
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let display_name = body["displayName"].as_str().unwrap_or("db-1");
        let db_name = body["dbName"].as_str().unwrap_or("db1");
        let compartment_id = body["compartmentId"].as_str().unwrap_or("");
        let cpu = body["cpuCoreCount"].as_i64().unwrap_or(1) as i32;
        let storage = body["dataStorageSizeInTBs"].as_i64().unwrap_or(1) as i32;

        let id = format!("ocid1.autonomousdatabase.oc1.iad.{}", uuid::Uuid::new_v4());

        let db = OracleAutonomousDb {
            id: id.clone(),
            compartment_id: compartment_id.to_string(),
            display_name: display_name.to_string(),
            db_name: db_name.to_string(),
            cpu_core_count: cpu,
            data_storage_size_in_tbs: storage,
            lifecycle_state: "AVAILABLE".to_string(),
        };

        self.storage.create_autonomous_database(db).map_err(|e| oracle_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "id": id,
            "displayName": display_name,
            "lifecycleState": "AVAILABLE"
        })))
    }
}
