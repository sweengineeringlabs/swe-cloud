use crate::Emulator;
use crate::error::EmulatorError;
use aws_data_core::storage::EventTargetMetadata;
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
    
    info!("EventBridge: {}", target);
    let action = target.split('.').next_back().unwrap_or(target);

    let result = match action {
        "CreateEventBus" => create_event_bus(&emulator, body).await,
        "DeleteEventBus" => delete_event_bus(&emulator, body).await,
        "ListEventBuses" => list_event_buses(&emulator, body).await,
        "PutRule" => put_rule(&emulator, body).await,
        "ListRules" => list_rules(&emulator, body).await,
        "PutTargets" => put_targets(&emulator, body).await,
        "ListTargetsByRule" => list_targets_by_rule(&emulator, body).await,
        "PutEvents" => put_events(&emulator, body).await,
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

async fn create_event_bus(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let name = body["Name"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Name".into()))?;
    let bus = emulator.storage.create_event_bus(name, &emulator.config.account_id, &emulator.config.region)?;

    Ok(json!({
        "EventBusArn": bus.arn
    }))
}

async fn delete_event_bus(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let name = body["Name"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Name".into()))?;
    emulator.storage.delete_event_bus(name)?;
    Ok(json!({}))
}

async fn list_event_buses(emulator: &Emulator, _body: Value) -> Result<Value, EmulatorError> {
    let buses = emulator.storage.list_event_buses()?;
    let bus_list: Vec<Value> = buses.into_iter().map(|b| {
        json!({
            "Name": b.name,
            "Arn": b.arn,
            "Policy": b.policy
        })
    }).collect();

    Ok(json!({
        "EventBuses": bus_list
    }))
}

async fn put_rule(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let name = body["Name"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Name".into()))?;
    let bus_name = body["EventBusName"].as_str().unwrap_or("default");
    let pattern = body["EventPattern"].as_str();
    let state = body["State"].as_str().unwrap_or("ENABLED");
    let description = body["Description"].as_str();
    let schedule = body["ScheduleExpression"].as_str();

    let arn = emulator.storage.put_rule(
        name,
        bus_name,
        pattern,
        state,
        description,
        schedule,
        &emulator.config.account_id,
        &emulator.config.region
    )?;

    Ok(json!({
        "RuleArn": arn
    }))
}

async fn list_rules(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let bus_name = body["EventBusName"].as_str().unwrap_or("default");
    let rules = emulator.storage.list_rules(bus_name)?;
    let rule_list: Vec<Value> = rules.into_iter().map(|r| {
        json!({
            "Name": r.name,
            "Arn": r.arn,
            "EventPattern": r.event_pattern,
            "State": r.state,
            "Description": r.description,
            "ScheduleExpression": r.schedule_expression,
            "EventBusName": r.event_bus_name
        })
    }).collect();

    Ok(json!({
        "Rules": rule_list
    }))
}

async fn put_targets(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let bus_name = body["EventBusName"].as_str().unwrap_or("default");
    let rule_name = body["Rule"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Rule".into()))?;
    let targets_val = body["Targets"].as_array().ok_or_else(|| EmulatorError::InvalidArgument("Missing Targets".into()))?;
    
    let mut targets = Vec::new();
    for t in targets_val {
        targets.push(EventTargetMetadata {
            id: t["Id"].as_str().unwrap_or("").to_string(),
            rule_name: rule_name.to_string(),
            event_bus_name: bus_name.to_string(),
            arn: t["Arn"].as_str().unwrap_or("").to_string(),
            input: t["Input"].as_str().map(|s| s.to_string()),
            input_path: t["InputPath"].as_str().map(|s| s.to_string()),
        });
    }

    emulator.storage.put_targets(bus_name, rule_name, targets)?;
    
    Ok(json!({
        "FailedEntries": [],
        "FailedEntryCount": 0
    }))
}

async fn list_targets_by_rule(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let bus_name = body["EventBusName"].as_str().unwrap_or("default");
    let rule_name = body["Rule"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Rule".into()))?;
    
    let targets = emulator.storage.list_targets(bus_name, rule_name)?;
    let target_list: Vec<Value> = targets.into_iter().map(|t| {
        json!({
            "Id": t.id,
            "Arn": t.arn,
            "Input": t.input,
            "InputPath": t.input_path
        })
    }).collect();

    Ok(json!({
        "Targets": target_list
    }))
}

async fn put_events(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let entries = body["Entries"].as_array().ok_or_else(|| EmulatorError::InvalidArgument("Missing Entries".into()))?;
    let mut results = Vec::new();

    for entry in entries {
        let bus_name = entry["EventBusName"].as_str().unwrap_or("default");
        let source = entry["Source"].as_str().unwrap_or("");
        let detail_type = entry["DetailType"].as_str().unwrap_or("");
        let detail = entry["Detail"].as_str().unwrap_or("{}");
        let resources = entry["Resources"].to_string();

        let event_id = emulator.storage.record_event(
            bus_name,
            source,
            detail_type,
            detail,
            Some(&resources)
        )?;

        // Match event against rules and trigger targets
        let rules = emulator.storage.list_rules(bus_name)?;
        
        for rule in rules {
            if rule.state != "ENABLED" {
                continue;
            }
            
            // Check if event matches rule pattern
            if let Some(pattern_str) = &rule.event_pattern {
                if matches_pattern(entry, pattern_str) {
                    info!("EventBridge: Event {} matched rule {}", event_id, rule.name);
                    
                    // Get targets for this rule
                    let targets = emulator.storage.list_targets(&rule.name, bus_name)?;
                    
                    // Trigger each target
                    for target in targets {
                        trigger_target(emulator, &target, entry, &event_id).await;
                    }
                }
            }
        }

        results.push(json!({
            "EventId": event_id
        }));
    }

    Ok(json!({
        "Entries": results,
        "FailedEntryCount": 0
    }))
}

/// Check if an event matches an EventBridge pattern
fn matches_pattern(event: &Value, pattern_str: &str) -> bool {
    let pattern: Value = match serde_json::from_str(pattern_str) {
        Ok(p) => p,
        Err(_) => return false,
    };
    
    // Simple pattern matching - check source and detail-type
    if let Some(sources) = pattern["source"].as_array() {
        let event_source = event["Source"].as_str().unwrap_or("");
        let source_match = sources.iter().any(|s| s.as_str() == Some(event_source));
        if !source_match {
            return false;
        }
    }
    
    if let Some(detail_types) = pattern["detail-type"].as_array() {
        let event_detail_type = event["DetailType"].as_str().unwrap_or("");
        let type_match = detail_types.iter().any(|t| t.as_str() == Some(event_detail_type));
        if !type_match {
            return false;
        }
    }
    
    // For full implementation, we'd need deeper pattern matching on detail object
    // For now, this covers the most common use cases
    true
}

/// Trigger a target with an event
async fn trigger_target(emulator: &Emulator, target: &EventTargetMetadata, event: &Value, event_id: &str) {
    let arn = &target.arn;
    
    // Parse ARN to determine target type
    if arn.contains(":sqs:") {
        // Send to SQS queue
        if let Some(queue_name) = arn.split(':').last() {
            let message = json!({
                "version": "0",
                "id": event_id,
                "detail-type": event["DetailType"],
                "source": event["Source"],
                "time": chrono::Utc::now().to_rfc3339(),
                "region": emulator.config.region,
                "resources": event.get("Resources").unwrap_or(&json!([])),
                "detail": serde_json::from_str::<Value>(event["Detail"].as_str().unwrap_or("{}")).unwrap_or(json!({}))
            });
            
            if let Err(e) = emulator.storage.send_message(queue_name, &message.to_string()) {
                tracing::warn!("EventBridge: Failed to send to SQS {}: {}", queue_name, e);
            } else {
                info!("EventBridge: Sent event to SQS queue {}", queue_name);
            }
        }
    } else if arn.contains(":sns:") {
        // Publish to SNS topic  
        info!("EventBridge: Would publish to SNS topic {} (SNS publish via EventBridge integration)", arn);
    } else if arn.contains(":lambda:") {
        // Invoke Lambda
        info!("EventBridge: Would invoke Lambda {} (Lambda execution not yet implemented)", arn);
    } else {
        tracing::warn!("EventBridge: Unknown target type: {}", arn);
    }
}
