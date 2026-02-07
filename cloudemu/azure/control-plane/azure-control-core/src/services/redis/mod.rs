use azure_data_core::StorageEngine;
use std::sync::Arc;
use azure_control_spi::{CloudResult, Request, Response, CloudError};
use serde_json::{json, Value};

pub struct RedisService {
    storage: Arc<StorageEngine>,
}

impl RedisService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // .../providers/Microsoft.Cache/redis/{name}
        if req.path.contains("/providers/Microsoft.Cache/redis/") && req.method == "PUT" {
            return self.create_redis(&req).await;
        }
        
        Ok(Response::not_found("Redis Service Not Found"))
    }

    async fn create_redis(&self, req: &Request) -> CloudResult<Response> {
        let parts: Vec<&str> = req.path.split('/').collect();
        let name = parts.last().unwrap_or(&"");
        let rg_parts: Vec<&str> = req.path.split("/resourceGroups/").collect();
        let rg = rg_parts.get(1).and_then(|s| s.split('/').next()).unwrap_or("");
        
        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let location = body["location"].as_str().unwrap_or("eastus");
        let sku_name = body["properties"]["sku"]["name"].as_str().unwrap_or("Basic");
        let sku_family = body["properties"]["sku"]["family"].as_str().unwrap_or("C");
        let capacity = body["properties"]["sku"]["capacity"].as_i64().unwrap_or(1) as i32;

        let redis = self.storage.create_redis_cache(name, rg, location, sku_name, sku_family, capacity)
            .map_err(|e| CloudError::Internal(e.to_string()))?;

        Ok(Response::ok(json!({
            "id": req.path,
            "name": redis.name,
            "location": redis.location,
            "properties": {
                "provisioningState": redis.provisioning_state,
                "hostName": redis.host_name,
                "port": redis.port,
                "sslPort": redis.ssl_port
            }
        }).to_string()).with_header("Content-Type", "application/json"))
    }
}
