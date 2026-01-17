use gcp_data_core::storage::StorageEngine;
use gcp_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;
use gcp_data_core::storage::GcpInstance;

pub struct ComputeService {
    storage: Arc<StorageEngine>,
}

impl ComputeService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // POST /compute/v1/projects/{project}/zones/{zone}/instances
        if req.method == "POST" && req.path.contains("/instances") {
            return self.insert_instance(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn insert_instance(&self, req: &Request) -> CloudResult<Response> {
        let parts: Vec<&str> = req.path.split('/').collect();
        let project_idx = parts.iter().position(|&x| x == "projects").unwrap() + 1;
        let project = parts[project_idx];
        let zone_idx = parts.iter().position(|&x| x == "zones").unwrap() + 1;
        let zone = parts[zone_idx];

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let name = body["name"].as_str().ok_or(gcp_control_spi::Error::BadRequest("Missing name".into()))?;
        let machine_type = body["machineType"].as_str().unwrap_or("n1-standard-1");

        let vm = GcpInstance {
            name: name.to_string(),
            project: project.to_string(),
            zone: zone.to_string(),
            machine_type: machine_type.to_string(),
            status: "RUNNING".to_string(),
        };

        self.storage.insert_instance(vm).map_err(|e| gcp_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "kind": "compute#operation",
            "status": "DONE",
            "targetLink": format!("https://www.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}", project, zone, name)
        })))
    }
}
