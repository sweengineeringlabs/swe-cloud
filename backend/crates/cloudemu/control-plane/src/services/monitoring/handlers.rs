use crate::Emulator;
use crate::error::EmulatorError;
use data_plane::storage::{MetricMetadata, LogEventMetadata};
use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::info;

pub async fn handle_request(
    State(emulator): State<Arc<Emulator>>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Response {
    let target = headers
        .get("x-amz-target")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    
    info!("CloudWatch/Logs: {}", target);
    let action = target.split('.').next_back().unwrap_or(target);

    let result = match action {
        // Monitoring (Metrics)
        "PutMetricData" => put_metric_data(&emulator, body).await,
        "ListMetrics" => list_metrics(&emulator, body).await,
        
        // Logs
        "CreateLogGroup" => create_log_group(&emulator, body).await,
        "DeleteLogGroup" => delete_log_group(&emulator, body).await,
        "CreateLogStream" => create_log_stream(&emulator, body).await,
        "PutLogEvents" => put_log_events(&emulator, body).await,
        "GetLogEvents" => get_log_events(&emulator, body).await,
        
        _ => Err(EmulatorError::InvalidRequest(format!("Unknown or unsupported target: {}", target))),
    };

    match result {
        Ok(json_val) => Json::<Value>(json_val).into_response(),
        Err(e) => {
            let code = e.code();
            let msg = e.message();
            let status = e.status_code();
            
            let json_err = json!({
                "__type": code,
                "message": msg
            });
            
            (status, Json::<Value>(json_err)).into_response()
        }
    }
}

// --- Monitoring ---

async fn put_metric_data(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let namespace = body["Namespace"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Namespace".into()))?;
    let metric_data = body["MetricData"].as_array().ok_or_else(|| EmulatorError::InvalidArgument("Missing MetricData".into()))?;
    
    let mut metrics = Vec::new();
    let now = chrono::Utc::now().to_rfc3339();
    
    for m in metric_data {
        metrics.push(MetricMetadata {
            namespace: namespace.to_string(),
            metric_name: m["MetricName"].as_str().unwrap_or("").to_string(),
            dimensions: Some(m["Dimensions"].to_string()),
            value: m["Value"].as_f64().unwrap_or(0.0),
            unit: m["Unit"].as_str().map(|s| s.to_string()),
            timestamp: m["Timestamp"].as_str().unwrap_or(&now).to_string(),
        });
    }

    emulator.storage.put_metric_data(namespace, metrics)?;
    
    Ok(json!({}))
}

async fn list_metrics(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let namespace = body["Namespace"].as_str();
    let metric_name = body["MetricName"].as_str();
    
    let metrics = emulator.storage.list_metrics(namespace, metric_name)?;
    let metric_list: Vec<Value> = metrics.into_iter().map(|m| {
        json!({
            "Namespace": m.namespace,
            "MetricName": m.metric_name,
            "Dimensions": serde_json::from_str::<Value>(&m.dimensions.unwrap_or("[]".into())).unwrap_or(json!([])),
            "Unit": m.unit
        })
    }).collect();

    Ok(json!({
        "Metrics": metric_list
    }))
}

// --- Logs ---

async fn create_log_group(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let name = body["logGroupName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing logGroupName".into()))?;
    emulator.storage.create_log_group(name, &emulator.config.account_id, &emulator.config.region)?;
    Ok(json!({}))
}

async fn delete_log_group(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let name = body["logGroupName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing logGroupName".into()))?;
    emulator.storage.delete_log_group(name)?;
    Ok(json!({}))
}

async fn create_log_stream(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let group_name = body["logGroupName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing logGroupName".into()))?;
    let stream_name = body["logStreamName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing logStreamName".into()))?;
    
    emulator.storage.create_log_stream(group_name, stream_name, &emulator.config.account_id, &emulator.config.region)?;
    Ok(json!({}))
}

async fn put_log_events(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let group_name = body["logGroupName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing logGroupName".into()))?;
    let stream_name = body["logStreamName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing logStreamName".into()))?;
    let events_val = body["logEvents"].as_array().ok_or_else(|| EmulatorError::InvalidArgument("Missing logEvents".into()))?;
    
    let mut events = Vec::new();
    for e in events_val {
        events.push(LogEventMetadata {
            timestamp: e["timestamp"].to_string(), // Keep as string for now
            message: e["message"].as_str().unwrap_or("").to_string(),
        });
    }

    emulator.storage.put_log_events(group_name, stream_name, events)?;
    
    Ok(json!({
        "nextSequenceToken": "stub-token"
    }))
}

async fn get_log_events(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let group_name = body["logGroupName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing logGroupName".into()))?;
    let stream_name = body["logStreamName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing logStreamName".into()))?;
    
    let events = emulator.storage.get_log_events(group_name, stream_name)?;
    let event_list: Vec<Value> = events.into_iter().map(|e| {
        json!({
            "timestamp": e.timestamp.parse::<i64>().unwrap_or(0),
            "message": e.message
        })
    }).collect();

    Ok(json!({
        "events": event_list,
        "nextForwardToken": "stub-token",
        "nextBackwardToken": "stub-token"
    }))
}
