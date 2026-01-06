//! Error types for the emulator

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Result type alias
pub type Result<T> = std::result::Result<T, EmulatorError>;

/// Emulator error type
#[derive(Debug, thiserror::Error)]
pub enum EmulatorError {
    // S3 Bucket Errors
    #[error("NoSuchBucket")]
    NoSuchBucket(String),
    
    #[error("BucketAlreadyOwnedByYou")]
    BucketAlreadyExists(String),
    
    #[error("BucketNotEmpty")]
    BucketNotEmpty(String),
    
    // S3 Object Errors
    #[error("NoSuchKey")]
    NoSuchKey(String),
    
    #[error("InvalidObjectState")]
    InvalidObjectState(String),
    
    // Policy Errors
    #[error("NoSuchBucketPolicy")]
    NoSuchBucketPolicy(String),
    
    #[error("MalformedPolicy")]
    MalformedPolicy(String),
    
    // General Errors
    #[error("InvalidRequest")]
    InvalidRequest(String),
    
    #[error("InvalidArgument")]
    InvalidArgument(String),
    
    #[error("MalformedXML")]
    MalformedXml(String),
    
    #[error("InternalError")]
    Internal(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("{0} not found: {1}")]
    NotFound(String, String),

    #[error("{0} already exists")]
    AlreadyExists(String),
}

impl EmulatorError {
    /// Get AWS error code
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoSuchBucket(_) => "NoSuchBucket",
            Self::BucketAlreadyExists(_) => "BucketAlreadyOwnedByYou",
            Self::BucketNotEmpty(_) => "BucketNotEmpty",
            Self::NoSuchKey(_) => "NoSuchKey",
            Self::InvalidObjectState(_) => "InvalidObjectState",
            Self::NoSuchBucketPolicy(_) => "NoSuchBucketPolicy",
            Self::MalformedPolicy(_) => "MalformedPolicy", 
            Self::InvalidRequest(_) => "InvalidRequest",
            Self::InvalidArgument(_) => "InvalidArgument",
            Self::MalformedXml(_) => "MalformedXML",
            Self::Internal(_) => "InternalError",
            Self::Database(_) => "InternalError",
            Self::Io(_) => "InternalError",
            Self::Json(_) => "InternalError",
            Self::NotFound(..) => "ResourceNotFound",
            Self::AlreadyExists(_) => "ResourceAlreadyExists",
        }
    }

    /// Get HTTP status code
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NoSuchBucket(_) | Self::NoSuchKey(_) | Self::NoSuchBucketPolicy(_) | Self::NotFound(..) => {
                StatusCode::NOT_FOUND
            }
            Self::BucketAlreadyExists(_) | Self::AlreadyExists(_) => StatusCode::CONFLICT,
            Self::BucketNotEmpty(_) | Self::InvalidRequest(_) | Self::InvalidArgument(_) | 
            Self::MalformedXml(_) | Self::MalformedPolicy(_) | Self::InvalidObjectState(_) => {
                StatusCode::BAD_REQUEST
            }
            Self::Internal(_) | Self::Database(_) | Self::Io(_) | Self::Json(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
    
    /// Get error message for AWS response
    pub fn message(&self) -> String {
        match self {
            Self::NoSuchBucket(name) => format!("The specified bucket does not exist: {}", name),
            Self::BucketAlreadyExists(name) => format!("Your previous request to create the named bucket succeeded and you already own it: {}", name),
            Self::BucketNotEmpty(name) => format!("The bucket you tried to delete is not empty: {}", name),
            Self::NoSuchKey(key) => format!("The specified key does not exist: {}", key),
            Self::InvalidObjectState(msg) => msg.clone(),
            Self::NoSuchBucketPolicy(name) => format!("The bucket policy does not exist: {}", name),
            Self::MalformedPolicy(msg) => format!("Malformed policy: {}", msg),
            Self::InvalidRequest(msg) => msg.clone(),
            Self::InvalidArgument(msg) => msg.clone(),
            Self::MalformedXml(msg) => format!("The XML you provided was not well-formed: {}", msg),
            Self::Internal(msg) => msg.clone(),
            Self::Database(msg) => msg.clone(),
            Self::Io(e) => e.to_string(),
            Self::Json(e) => e.to_string(),
            Self::NotFound(type_, id) => format!("{} not found: {}", type_, id),
            Self::AlreadyExists(msg) => msg.clone(),
        }
    }
}

impl From<rusqlite::Error> for EmulatorError {
    fn from(e: rusqlite::Error) -> Self {
        Self::Database(e.to_string())
    }
}

impl IntoResponse for EmulatorError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let code = self.code();
        let message = self.message();
        let request_id = uuid::Uuid::new_v4().to_string();

        // AWS S3 error response format
        let body = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<Error>
    <Code>{}</Code>
    <Message>{}</Message>
    <RequestId>{}</RequestId>
</Error>"#,
            code, 
            quick_xml::escape::escape(&message),
            request_id
        );

        (
            status, 
            [
                ("content-type", "application/xml"),
                ("x-amz-request-id", &request_id),
            ], 
            body
        ).into_response()
    }
}
