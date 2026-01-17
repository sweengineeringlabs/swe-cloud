use oracle_data_core::StorageEngine;
use oracle_control_spi::{Request, Response, CloudResult};
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
        // POST /20180115/zones
        if req.path.ends_with("/zones") && req.method == "POST" {
            return self.create_zone(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_zone(&self, req: &Request) -> CloudResult<Response> {
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let name = body["name"].as_str().unwrap_or("example.com");
        let zone_type = body["zoneType"].as_str().unwrap_or("PRIMARY");

        let zone = self.storage.create_zone(name, zone_type).map_err(|e| oracle_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "id": zone.id,
            "name": zone.name,
            "zoneType": zone.zone_type,
            "lifecycleState": zone.lifecycle_state
        })))
    }
}
