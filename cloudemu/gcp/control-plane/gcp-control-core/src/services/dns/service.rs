use gcp_data_core::storage::StorageEngine;
use gcp_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct DnsService {
    storage: Arc<StorageEngine>,
}

impl DnsService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // POST /dns/v1/projects/{project}/managedZones
        if req.path.contains("/managedZones") && req.method == "POST" {
            return self.create_zone(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_zone(&self, req: &Request) -> CloudResult<Response> {
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let name = body["name"].as_str().unwrap_or("zone1");
        let dns_name = body["dnsName"].as_str().unwrap_or("example.com.");
        let desc = body["description"].as_str().unwrap_or("");

        let zone = self.storage.create_managed_zone(name, dns_name, desc).map_err(|e| gcp_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "kind": "dns#managedZone",
            "name": zone.name,
            "dnsName": zone.dns_name,
            "description": zone.description,
            "id": zone.id,
            "visibility": zone.visibility
        })))
    }
}
