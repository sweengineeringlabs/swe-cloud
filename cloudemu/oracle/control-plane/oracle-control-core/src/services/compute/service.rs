use oracle_data_core::StorageEngine;
use oracle_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;
use oracle_data_core::storage::OracleInstance;

pub struct ComputeService {
    storage: Arc<StorageEngine>,
}

impl ComputeService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // POST /20160918/instances
        if req.method == "POST" && req.path.ends_with("/instances") {
            return self.launch_instance(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn launch_instance(&self, req: &Request) -> CloudResult<Response> {
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let display_name = body["displayName"].as_str().unwrap_or("instance-1");
        let compartment_id = body["compartmentId"].as_str().unwrap_or("");
        let shape = body["shape"].as_str().unwrap_or("VM.Standard2.1");

        let id = format!("ocid1.instance.oc1.iad.{}", uuid::Uuid::new_v4());

        let instance = OracleInstance {
            id: id.clone(),
            compartment_id: compartment_id.to_string(),
            display_name: display_name.to_string(),
            shape: shape.to_string(),
            lifecycle_state: "RUNNING".to_string(),
        };

        self.storage.launch_instance(instance).map_err(|e| oracle_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "id": id,
            "displayName": display_name,
            "lifecycleState": "RUNNING"
        })))
    }
}
