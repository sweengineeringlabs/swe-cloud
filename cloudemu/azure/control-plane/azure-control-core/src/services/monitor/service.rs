use azure_data_core::storage::StorageEngine;
use azure_control_spi::{Request, Response, CloudResult, CloudError};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct MonitorService {
    storage: Arc<StorageEngine>,
}

impl MonitorService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // /subscriptions/{sub}/resourceGroups/{rg}/providers/{resourceProvider}/{resourceType}/{resourceName}/providers/microsoft.insights/metrics
        // Simplify: check if it contains /providers/microsoft.insights/metrics
        
        if req.path.to_lowercase().contains("/providers/microsoft.insights/metrics") {
             if req.method == "GET" {
                 return self.list_metrics(&req);
             }
        }
        
        Ok(Response::not_found("Not Found"))
    }

    fn list_metrics(&self, _req: &Request) -> CloudResult<Response> {
        // We can extract namespace as "Microsoft.Compute/virtualMachines"
        let _namespace = "Microsoft.Compute/virtualMachines"; 
        
        Ok(Response::ok(json!({
            "cost": 0,
            "timespan": "2023-01-01T00:00:00Z/2023-01-01T01:00:00Z",
            "interval": "PT1M",
            "value": []
        }).to_string()).with_header("Content-Type", "application/json"))
    }
}
