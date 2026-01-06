use crate::Emulator;
use crate::error::EmulatorError;
use crate::storage::EventTargetMetadata;
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
    let action = target.split('.').last().unwrap_or(target);

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
        Ok(json_val) => Json(json_val).into_response(),
        Err(e) => {
            let code = e.code();
            let msg = e.message();
            let status = e.status_code();
            
            let json_err = json!({
                "__type": code,
                "message": msg
            });
            
            (status, Json(json_err)).into_response()
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

        results.push(json!({
            "EventId": event_id
        }));
    }

    Ok(json!({
        "Entries": results,
        "FailedEntryCount": 0
    }))
}
