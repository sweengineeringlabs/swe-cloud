//! ZeroCloud Control Plane Orchestrator

use zero_control_spi::{ZeroRequest, ZeroResponse, ZeroResult, ZeroService, ZeroError};
use zero_data_core::ZeroEngine;
use async_trait::async_trait;
use std::sync::Arc;
use serde_json::json;

pub struct ZeroProvider {
    engine: Arc<ZeroEngine>,
}

impl ZeroProvider {
    pub fn new(engine: Arc<ZeroEngine>) -> Self {
        Self { engine }
    }
}

#[async_trait]
impl ZeroService for ZeroProvider {
    async fn handle_request(&self, req: ZeroRequest) -> ZeroResult<ZeroResponse> {
        // Path routing: /v1/nodes
        if req.path.contains("/nodes") && req.method == "GET" {
            let nodes = self.engine.list_nodes().map_err(|e| ZeroError::Internal(e.to_string()))?;
            return Ok(ZeroResponse::json(json!({ "nodes": nodes })));
        }

        // Create Workload: POST /v1/workloads
        if req.path.contains("/workloads") && req.method == "POST" {
            let body: serde_json::Value = serde_json::from_slice(&req.body)
                .map_err(|e| ZeroError::Validation(e.to_string()))?;
            
            let id = body["id"].as_str().ok_or_else(|| ZeroError::Validation("Missing id".into()))?;
            let image = body["image"].as_str().ok_or_else(|| ZeroError::Validation("Missing image".into()))?;

            let status = self.engine.compute.create_workload(id, image, 1.0, 512).await?;
            return Ok(ZeroResponse::json(json!(status)));
        }

        // Create Volume: POST /v1/volumes
        if req.path.contains("/volumes") && req.method == "POST" {
            let body: serde_json::Value = serde_json::from_slice(&req.body)
                .map_err(|e| ZeroError::Validation(e.to_string()))?;
            
            let id = body["id"].as_str().ok_or_else(|| ZeroError::Validation("Missing id".into()))?;
            let size = body["size_gb"].as_i64().unwrap_or(10) as i32;

            let status = self.engine.storage.create_volume(id, size).await?;
            return Ok(ZeroResponse::json(json!(status)));
        }

        // Delete Workload: DELETE /v1/workloads
        if req.path.contains("/workloads") && req.method == "DELETE" {
            let body: serde_json::Value = serde_json::from_slice(&req.body)
                .map_err(|e| ZeroError::Validation(e.to_string()))?;
            
            let id = body["id"].as_str().ok_or_else(|| ZeroError::Validation("Missing id".into()))?;

            self.engine.compute.delete_workload(id).await?;
            return Ok(ZeroResponse::json(json!({ "status": "Deleted", "id": id })));
        }

        // Create Network: POST /v1/networks
        if req.path.contains("/networks") && req.method == "POST" {
            let body: serde_json::Value = serde_json::from_slice(&req.body)
                .map_err(|e| ZeroError::Validation(e.to_string()))?;
            
            let id = body["id"].as_str().ok_or_else(|| ZeroError::Validation("Missing id".into()))?;
            let cidr = body["cidr"].as_str().unwrap_or("10.0.0.0/24");

            let status = self.engine.network.create_network(id, cidr).await?;
            return Ok(ZeroResponse::json(json!(status)));
        }

        Ok(ZeroResponse::json(json!({ "message": "ZeroCloud API v1" })))
    }
}
