use crate::Emulator;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;
use std::sync::Arc;
use tracing::warn;

pub async fn dispatch(
    State(emulator): State<Arc<Emulator>>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Response {
    let _ = &emulator;
    let _ = &body;

    let target = headers
        .get("x-amz-target")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let service = target.split('.').next().unwrap_or("");

    match service {
        #[cfg(feature = "dynamodb")]
        "DynamoDB_20120810" => {
            crate::services::dynamodb::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "sqs")]
        "AmazonSQS" | "AWSSQS" => {
            crate::services::sqs::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "secretsmanager")]
        "secretsmanager" => {
            crate::services::secrets::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "eventbridge")]
        "AWSEvents" => {
            crate::services::events::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "kms")]
        "TrentService" => { // KMS uses TrentService internally often or just KMS?
            // AWS KMS target is usually "TrentService.<Op>"
            crate::services::kms::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "cloudwatch")]
        "Monitoring" | "Logs_20140530" => {
             crate::services::monitoring::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "cognito")]
        "AWSCognitoIdentityProviderService" => {
             crate::services::identity::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "stepfunctions")]
        "AWSStepFunctions" => {
             crate::services::workflows::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "ec2")]
        "AmazonEC2" => {
             crate::services::ec2::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        #[cfg(feature = "sns")]
        "AmazonSNS" | "" if body["Action"].as_str().is_some() => {
             crate::services::sns::handlers::handle_request(
                State(emulator),
                headers,
                Json(body),
            ).await
        }
        _ => {
            warn!("Unknown service target: {}", target);
            (StatusCode::NOT_FOUND, format!("Unknown service target: {}", target)).into_response()
        }
    }
}
