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
    _headers: HeaderMap,
    Json(body): Json<Value>,
) -> Response {
    let action = body["Action"].as_str()
        .or_else(|| body["action"].as_str())
        .unwrap_or("");

    let result = match action {
        "CreateDBInstance" => create_db_instance(&emulator, body).await,
        "DescribeDBInstances" => describe_db_instances(&emulator, body).await,
        _ => Err(EmulatorError::NotImplemented(format!("RDS action: {}", action))),
    };

    match result {
        Ok(val) => (axum::http::StatusCode::OK, Json(val)).into_response(),
        Err(e) => crate::error::ApiError(e).into_response(),
    }
}

async fn create_db_instance(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let id = body["DBInstanceIdentifier"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing DBInstanceIdentifier".into()))?;
    let engine = body["Engine"].as_str().unwrap_or("mysql");
    let class = body["DBInstanceClass"].as_str().unwrap_or("db.t3.micro");
    let username = body["MasterUsername"].as_str().unwrap_or("admin");
    let allocated_storage = body["AllocatedStorage"].as_i64().unwrap_or(20) as i32;

    let instance = emulator.storage.create_db_instance(id, engine, class, username, allocated_storage)?;

    Ok(json!({
        "DBInstance": {
            "DBInstanceIdentifier": instance.identifier,
            "DBInstanceStatus": instance.status,
            "Engine": instance.engine,
            "DBInstanceClass": instance.class,
            "AllocatedStorage": instance.allocated_storage,
            "MasterUsername": instance.username,
            "Endpoint": {
                "Address": instance.endpoint_address,
                "Port": instance.endpoint_port,
                "HostedZoneId": "Z000000000000"
            }
        }
    }))
}

async fn describe_db_instances(emulator: &Emulator, _body: Value) -> Result<Value, EmulatorError> {
    let instances = emulator.storage.list_db_instances()?;
    
    // Convert to JSON
    let instances_json: Vec<Value> = instances.into_iter().map(|i| {
        json!({
            "DBInstanceIdentifier": i.identifier,
            "DBInstanceStatus": i.status,
            "Engine": i.engine,
            "DBInstanceClass": i.class,
            "AllocatedStorage": i.allocated_storage,
            "MasterUsername": i.username,
            "Endpoint": {
                "Address": i.endpoint_address,
                "Port": i.endpoint_port
            }
        })
    }).collect();

    Ok(json!({
        "DBInstances": instances_json
    }))
}
