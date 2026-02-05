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
    
    info!("SecretsManager: {}", target);

    let result = match target {
        "secretsmanager.CreateSecret" => create_secret(&emulator, body).await,
        "secretsmanager.GetSecretValue" => get_secret_value(&emulator, body).await,
        "secretsmanager.PutSecretValue" => put_secret_value(&emulator, body).await,
        _ => Err(EmulatorError::InvalidRequest(format!("Unknown or unsupported target: {}", target))),
    };

    match result {
        Ok(json_val) => Json::<Value>(json_val).into_response(),
        Err(e) => {
            let code = e.code();
            let msg = e.message();
            let status = e.status_code();
            
            // AWS JSON 1.1 Error Format
            let json_err = json!({
                "__type": code,
                "message": msg
            });
            
            (status, Json::<Value>(json_err)).into_response()
        }
    }
}

async fn create_secret(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let name = body["Name"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Name".into()))?;
    let desc = body["Description"].as_str();
    let tags = body["Tags"].to_string(); // Simple serialization for storage

    let meta = emulator.storage.create_secret(
        name, 
        desc, 
        Some(&tags), 
        &emulator.config.account_id, 
        &emulator.config.region
    )?;

    Ok(json!({
        "ARN": meta.arn,
        "Name": meta.name,
        "VersionId": null 
    }))
}

async fn get_secret_value(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let secret_id = body["SecretId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing SecretId".into()))?;
    let version_id = body["VersionId"].as_str();
    let version_stage = body["VersionStage"].as_str();

    let val = emulator.storage.get_secret_value(secret_id, version_id, version_stage)?;

    // Parse created date
    let created_date_ts = val.created_date.parse::<chrono::DateTime<chrono::Utc>>()
        .map(|d| d.timestamp() as f64)
        .unwrap_or(0.0);

    Ok(json!({
        "ARN": val.arn,
        "Name": val.name,
        "VersionId": val.version_id,
        "SecretString": val.secret_string,
        "SecretBinary": val.secret_binary,
        "VersionStages": val.version_stages,
        "CreatedDate": created_date_ts
    }))
}

async fn put_secret_value(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
     let secret_id = body["SecretId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing SecretId".into()))?;
     let secret_string = body["SecretString"].as_str();
     let secret_binary = body["SecretBinary"].as_str().map(|s| s.as_bytes().to_vec()); // Naive handling
     
     if secret_string.is_none() && secret_binary.is_none() {
         return Err(EmulatorError::InvalidArgument("Must provide SecretString or SecretBinary".into()));
     }
     
     let (arn, version_id) = emulator.storage.put_secret_value(
         secret_id, 
         secret_string, 
         secret_binary.as_deref()
    )?;
     
     Ok(json!({
         "ARN": arn,
         "Name": secret_id,
         "VersionId": version_id,
         "VersionStages": ["AWSCURRENT"]
     }))
}
