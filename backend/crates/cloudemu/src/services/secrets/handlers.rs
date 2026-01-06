use crate::Emulator;
use axum::{extract::State, http::{HeaderMap, StatusCode}, response::{IntoResponse, Response}, Json};
use serde_json::Value;
use std::sync::Arc;

pub async fn handle_request(
    State(_emulator): State<Arc<Emulator>>,
    headers: HeaderMap,
    Json(_body): Json<Value>,
) -> Response {
    let target = headers.get("x-amz-target").and_then(|h| h.to_str().ok()).unwrap_or("");
    // Dispatch based on target
    (StatusCode::OK, format!("Secrets Stub: {}", target)).into_response()
}
