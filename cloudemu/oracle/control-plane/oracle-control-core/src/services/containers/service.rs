use oracle_data_core::StorageEngine;
use oracle_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct ContainerService {
    storage: Arc<StorageEngine>,
}

impl ContainerService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // /20210201/containerInstances
        if req.path.contains("/containerInstances") && req.method == "POST" {
            return self.create_instance(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_instance(&self, req: &Request) -> CloudResult<Response> {
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let name = body["displayName"].as_str().unwrap_or("ci1");
        let compartment = body["compartmentId"].as_str().unwrap_or("ocid1.compartment.oc1..test");
        let ad = body["availabilityDomain"].as_str().unwrap_or("AD-1");

        let instance = self.storage.create_container_instance(name, compartment, ad).map_err(|e| oracle_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "id": instance.id,
            "displayName": instance.display_name,
            "compartmentId": instance.compartment_id,
            "availabilityDomain": instance.availability_domain,
            "lifecycleState": instance.state
        })))
    }
}
