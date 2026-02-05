use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
#[derive(Debug)]
pub struct ApiError(pub aws_data_core::error::EmulatorError);

// Re-export for convenience
pub use aws_data_core::error::EmulatorError;

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ApiError {}

impl From<EmulatorError> for ApiError {
    fn from(inner: EmulatorError) -> Self {
        ApiError(inner)
    }
}

impl From<std::io::Error> for ApiError {
    fn from(e: std::io::Error) -> Self {
        ApiError(EmulatorError::Io(e))
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        ApiError(EmulatorError::Json(e))
    }
}

impl std::ops::Deref for ApiError {
    type Target = EmulatorError;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type Result<T> = std::result::Result<T, ApiError>;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let err = self.0;
        
        let status = match &err {
            EmulatorError::NoSuchBucket(_) | EmulatorError::NoSuchKey(_) | EmulatorError::NoSuchBucketPolicy(_) | EmulatorError::NotFound(..) => {
                StatusCode::NOT_FOUND
            }
            EmulatorError::BucketAlreadyExists(_) | EmulatorError::AlreadyExists(_) => StatusCode::CONFLICT,
            EmulatorError::BucketNotEmpty(_) | EmulatorError::InvalidRequest(_) | EmulatorError::InvalidArgument(_) | 
            EmulatorError::MalformedXml(_) | EmulatorError::MalformedPolicy(_) | EmulatorError::InvalidObjectState(_) => {
                StatusCode::BAD_REQUEST
            }
            EmulatorError::Internal(_) | EmulatorError::Database(_) | EmulatorError::Io(_) | EmulatorError::Json(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            EmulatorError::NotImplemented(_) => StatusCode::NOT_IMPLEMENTED,
        };

        let code = err.code();
        let message = err.message();
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
