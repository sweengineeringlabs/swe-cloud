use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use gcp_data_core::error::EmulatorError;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Core error: {0}")]
    Core(#[from] EmulatorError),
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Core(EmulatorError::NotFound(msg)) => (StatusCode::NOT_FOUND, msg),
            ApiError::Core(EmulatorError::AlreadyExists(msg)) => (StatusCode::CONFLICT, msg),
            ApiError::Core(EmulatorError::InvalidRequest(msg)) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Core(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "error": {
                "message": message,
                "code": status.as_u16()
            }
        }));

        (status, body).into_response()
    }
}
