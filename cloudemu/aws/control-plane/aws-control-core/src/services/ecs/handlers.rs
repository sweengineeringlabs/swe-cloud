use crate::Emulator;
use crate::error::EmulatorError;
use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn handle_request(
    State(emulator): State<Arc<Emulator>>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Response {
    let target = headers.get("x-amz-target")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    let action = target.split('.').last().unwrap_or("");

    let result = match action {
        "CreateCluster" => create_cluster(&emulator, body).await,
        "ListClusters" => list_clusters(&emulator, body).await,
        "RegisterTaskDefinition" => register_task_definition(&emulator, body).await,
        _ => Err(EmulatorError::NotImplemented(format!("ECS action: {}", action))),
    };

    match result {
        Ok(val) => (axum::http::StatusCode::OK, Json(val)).into_response(),
        Err(e) => crate::error::ApiError(e).into_response(),
    }
}

async fn create_cluster(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let name = body["clusterName"].as_str().unwrap_or("default");
    let cluster = emulator.storage.create_cluster(name)?;
    
    Ok(json!({
        "cluster": {
            "clusterName": cluster.name,
            "clusterArn": cluster.arn,
            "status": cluster.status,
            "registeredContainerInstancesCount": 0,
            "runningTasksCount": 0,
            "pendingTasksCount": 0,
            "activeServicesCount": 0
        }
    }))
}

async fn list_clusters(emulator: &Emulator, _body: Value) -> Result<Value, EmulatorError> {
    let arns = emulator.storage.list_clusters()?;
    Ok(json!({
        "clusterArns": arns
    }))
}

async fn register_task_definition(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let family = body["family"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing family".into()))?;
    
    // Parse container definitions from JSON array
    let containers_json = body["containerDefinitions"].as_array().ok_or_else(|| EmulatorError::InvalidArgument("Missing containerDefinitions".into()))?;
    
    // Convert JSON containers to internal struct
    use aws_data_core::ContainerDefinition; // Assuming exposed
    let mut containers = Vec::new();
    
    for c in containers_json {
        use aws_data_core::PortMapping;
        let mut ports = Vec::new();
        if let Some(p_maps) = c["portMappings"].as_array() {
            for p in p_maps {
                ports.push(PortMapping {
                    container_port: p["containerPort"].as_i64().unwrap_or(0) as i32,
                    host_port: p["hostPort"].as_i64().unwrap_or(0) as i32,
                    protocol: p["protocol"].as_str().unwrap_or("tcp").to_string(),
                });
            }
        }
    
        containers.push(ContainerDefinition {
            name: c["name"].as_str().unwrap_or("default").to_string(),
            image: c["image"].as_str().unwrap_or("alpine").to_string(),
            cpu: c["cpu"].as_i64().unwrap_or(256) as i32,
            memory: c["memory"].as_i64().unwrap_or(512) as i32,
            port_mappings: ports,
        });
    }

    let task_def = emulator.storage.register_task_definition(family, containers)?;
    
    // Convert back to JSON for response (simplified)
    Ok(json!({
        "taskDefinition": {
            "family": task_def.family,
            "taskDefinitionArn": task_def.arn,
            "revision": task_def.revision,
            "status": "ACTIVE"
        }
    }))
}
