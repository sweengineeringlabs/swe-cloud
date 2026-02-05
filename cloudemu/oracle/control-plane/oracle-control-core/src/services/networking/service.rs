use oracle_data_core::StorageEngine;
use oracle_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct NetworkingService {
    storage: Arc<StorageEngine>,
}

impl NetworkingService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // /20160918/vcns
        if req.path.ends_with("/vcns") && req.method == "POST" {
            return self.create_vcn(&req);
        }
        // /20160918/subnets
        if req.path.ends_with("/subnets") && req.method == "POST" {
            return self.create_subnet(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_vcn(&self, req: &Request) -> CloudResult<Response> {
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let name = body["displayName"].as_str().unwrap_or("vcn1");
        let compartment = body["compartmentId"].as_str().unwrap_or("ocid1.compartment.oc1..test");
        let cidr = body["cidrBlock"].as_str().unwrap_or("10.0.0.0/16");

        let vcn = self.storage.create_vcn(name, compartment, cidr).map_err(|e| oracle_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "id": vcn.id,
            "displayName": vcn.display_name,
            "cidrBlock": vcn.cidr_block,
            "compartmentId": vcn.compartment_id,
            "lifecycleState": vcn.state
        })))
    }

    fn create_subnet(&self, req: &Request) -> CloudResult<Response> {
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let name = body["displayName"].as_str().unwrap_or("subnet1");
        let vcn_id = body["vcnId"].as_str().unwrap_or("");
        let compartment = body["compartmentId"].as_str().unwrap_or("ocid1.compartment.oc1..test");
        let cidr = body["cidrBlock"].as_str().unwrap_or("10.0.0.0/24");

        let subnet = self.storage.create_subnet(name, vcn_id, compartment, cidr).map_err(|e| oracle_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "id": subnet.id,
            "displayName": subnet.display_name,
            "vcnId": subnet.vcn_id,
            "cidrBlock": subnet.cidr_block,
            "compartmentId": subnet.compartment_id,
            "lifecycleState": "AVAILABLE"
        })))
    }
}
