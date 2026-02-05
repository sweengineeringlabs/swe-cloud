use azure_data_core::storage::StorageEngine;
use azure_control_spi::{Request, Response, CloudResult, CloudError};
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
        // PUT .../dnsZones/{zoneName}
        if req.path.contains("/dnsZones/") && req.method == "PUT" {
            return self.create_zone(&req);
        }
        Ok(Response::not_found("Not Found"))
    }

    fn create_zone(&self, req: &Request) -> CloudResult<Response> {
        let parts: Vec<&str> = req.path.split('/').collect();
        let name_idx = parts.iter().position(|&x| x == "dnsZones").unwrap() + 1;
        let name = parts[name_idx];
        let rg_idx = parts.iter().position(|&x| x == "resourceGroups").unwrap() + 1;
        let rg = parts[rg_idx];

        let zone = self.storage.create_dns_zone(name, rg).map_err(|e| CloudError::Internal(e.to_string()))?;

        Ok(Response::ok(json!({
            "id": zone.id,
            "name": zone.name,
            "type": "Microsoft.Network/dnsZones",
            "location": "global",
            "properties": {
                "zoneType": zone.zone_type
            }
        }).to_string()).with_header("Content-Type", "application/json"))
    }
}
