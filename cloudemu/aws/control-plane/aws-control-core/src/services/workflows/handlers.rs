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
    
    info!("StepFunctions: {}", target);
    let action = target.split('.').next_back().unwrap_or(target);

    let result = match action {
        "CreateStateMachine" => create_state_machine(&emulator, body).await,
        "ListStateMachines" => list_state_machines(&emulator, body).await,
        "DeleteStateMachine" => delete_state_machine(&emulator, body).await,
        "StartExecution" => start_execution(&emulator, body).await,
        "DescribeExecution" => describe_execution(&emulator, body).await,
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

async fn create_state_machine(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let name = body["name"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing name".into()))?;
    let definition = body["definition"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing definition".into()))?;
    let role_arn = body["roleArn"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing roleArn".into()))?;
    let machine_type = body["type"].as_str().unwrap_or("STANDARD");

    let machine = emulator.storage.create_state_machine(
        name,
        definition,
        role_arn,
        machine_type,
        &emulator.config.account_id,
        &emulator.config.region
    )?;

    Ok(json!({
        "stateMachineArn": machine.arn,
        "creationDate": 1234567890.0
    }))
}

async fn list_state_machines(emulator: &Emulator, _body: Value) -> Result<Value, EmulatorError> {
    let machines = emulator.storage.list_state_machines()?;
    let machine_list: Vec<Value> = machines.into_iter().map(|m| {
        json!({
            "stateMachineArn": m.arn,
            "name": m.name,
            "type": m.machine_type,
            "creationDate": 1234567890.0
        })
    }).collect();

    Ok(json!({
        "stateMachines": machine_list
    }))
}

async fn delete_state_machine(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let arn = body["stateMachineArn"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing stateMachineArn".into()))?;
    emulator.storage.delete_state_machine(arn)?;
    Ok(json!({}))
}

async fn start_execution(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let machine_arn = body["stateMachineArn"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing stateMachineArn".into()))?;
    let name = body["name"].as_str();
    let input = body["input"].as_str().unwrap_or("{}");

    // Get state machine definition
    let machines = emulator.storage.list_state_machines()?;
    let machine = machines.iter().find(|m| m.arn == machine_arn)
        .ok_or_else(|| EmulatorError::NotFound("StateMachine".into(), machine_arn.into()))?;

    // Create execution record
    let mut exec = emulator.storage.start_execution(
        machine_arn,
        name,
        Some(input),
        &emulator.config.account_id,
        &emulator.config.region
    )?;

    // Execute the state machine
    info!("StepFunctions: Executing state machine: {}", machine.name);
    match super::interpreter::StateMachineExecutor::execute(&machine.definition, input) {
        Ok(output) => {
            // Update execution with success
            emulator.storage.update_execution_status(&exec.arn, "SUCCEEDED", Some(&output))?;
            exec.status = "SUCCEEDED".to_string();
            exec.output = Some(output);
            info!("StepFunctions: Execution succeeded: {}", exec.name);
        },
        Err(e) => {
            // Update execution with failure
            let error_msg = format!("Execution failed: {}", e.message());
            emulator.storage.update_execution_status(&exec.arn, "FAILED", Some(&error_msg))?;
            exec.status = "FAILED".to_string();
            exec.output = Some(error_msg);
            tracing::error!("StepFunctions: Execution failed: {}", e.message());
        }
    }

    Ok(json!({
        "executionArn": exec.arn,
        "startDate": 1234567890.0
    }))
}

async fn describe_execution(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let arn = body["executionArn"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing executionArn".into()))?;
    let exec = emulator.storage.describe_execution(arn)?;

    Ok(json!({
        "executionArn": exec.arn,
        "stateMachineArn": exec.state_machine_arn,
        "name": exec.name,
        "status": exec.status,
        "startDate": 1234567890.0,
        "input": exec.input,
        "output": exec.output
    }))
}
