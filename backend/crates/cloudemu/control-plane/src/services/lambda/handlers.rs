use crate::Emulator;
use crate::error::EmulatorError;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn handle_request(
    State(emulator): State<Arc<Emulator>>,
    req: axum::extract::Request,
) -> Response {
    let path_val = req.uri().path().to_string();
    let method = req.method().clone();
    
    // Detect action from path
    if path_val.contains("/invocations") {
        let function_name = path_val.split('/')
            .find(|s| !s.is_empty() && *s != "2015-03-31" && *s != "functions")
            .unwrap_or("");
            
        // For invocation, we need the body
        let body_bytes = match axum::body::to_bytes(req.into_body(), 10 * 1024 * 1024).await {
            Ok(b) => b,
            Err(_) => return (axum::http::StatusCode::BAD_REQUEST, "Invalid body").into_response(),
        };
        
        let body_val: Value = serde_json::from_slice(&body_bytes).unwrap_or(json!({}));

        return match invoke(&emulator, function_name, body_val).await {
            Ok(val) => Json::<Value>(val).into_response(),
            Err(e) => (e.status_code(), Json(json!({"message": e.message()}))).into_response(),
        };
    }

    if method == axum::http::Method::POST && path_val.ends_with("/functions") {
         // Create function
         let body_bytes = match axum::body::to_bytes(req.into_body(), 10 * 1024 * 1024).await {
            Ok(b) => b,
            Err(_) => return (axum::http::StatusCode::BAD_REQUEST, "Invalid body").into_response(),
        };
        let body_val: Value = serde_json::from_slice(&body_bytes).unwrap_or(json!({}));
        
        return match create_function(&emulator, body_val).await {
            Ok(val) => Json::<Value>(val).into_response(),
            Err(e) => (e.status_code(), Json(json!({"message": e.message()}))).into_response(),
        };
    }

    // Default to a generic handler or error
    (axum::http::StatusCode::NOT_FOUND, "Lambda Endpoint Not Found").into_response()
}

// Special handler for POST /2015-03-31/functions/{FunctionName}/invocations
pub async fn invoke(emulator: &Emulator, name: &str, _payload: Value) -> Result<Value, EmulatorError> {
    let _function = emulator.storage.get_function(name)?;
    
    // CloudEmu mock: just returns a success message
    Ok(json!({
        "StatusCode": 200,
        "Payload": "Mock execution successful"
    }))
}

pub async fn create_function(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let name = body["FunctionName"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing FunctionName".into()))?;
    let runtime = body["Runtime"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Runtime".into()))?;
    let role = body["Role"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Role".into()))?;
    let handler = body["Handler"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Handler".into()))?;
    
    
    let func = emulator.storage.create_function(crate::storage::CreateFunctionParams {
        name,
        runtime,
        role,
        handler,
        code_hash: "mock-hash",
        account_id: &emulator.config.account_id,
        region: &emulator.config.region
    })?;
    
    Ok(json!(func))
}
