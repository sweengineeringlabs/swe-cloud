use crate::Emulator;
use axum::{extract::State, http::{HeaderMap, StatusCode}, response::{IntoResponse, Response}, Json};
use serde_json::Value;
use std::sync::Arc;

pub async fn handle_request(
    State(_emulator): State<Arc<Emulator>>,
    headers: HeaderMap,
    Json(_body): Json<Value>,
) -> Response {
    (StatusCode::OK, "SQS Stub").into_response()
}
