//! Core error types for multi-cloud emulation.

/// Result type alias for CloudEmu operations.
pub type CloudResult<T> = Result<T, CloudError>;

/// Unified error type for all cloud operations across providers.
#[derive(Debug, thiserror::Error)]
pub enum CloudError {
    /// Resource not found
    #[error("{resource_type} not found: {resource_id}")]
    NotFound {
        /// Type of resource (bucket, table, queue, etc.)
        resource_type: String,
        /// Identifier of the resource
        resource_id: String,
    },

    /// Resource already exists
    #[error("{resource_type} already exists: {resource_id}")]
    AlreadyExists {
        /// Type of resource
        resource_type: String,
        /// Identifier of the resource
        resource_id: String,
    },

    /// Invalid input or configuration
    #[error("Validation error: {0}")]
    Validation(String),

    /// Unsupported service type
    #[error("Service {0:?} not supported by this provider")]
    UnsupportedService(crate::ServiceType),

    /// Unsupported operation
    #[error("Operation '{0}' not supported")]
    UnsupportedOperation(String),

    /// Provider-specific error
    #[error("[{provider}] {code}: {message}")]
    Provider {
        /// Provider name (aws, azure, gcp)
        provider: String,
        /// Error code from the provider
        code: String,
        /// Error message
        message: String,
    },

    /// Storage error
    #[error("Storage error: {0}")]
    Storage(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<serde_json::Error> for CloudError {
    fn from(err: serde_json::Error) -> Self {
        CloudError::Serialization(err.to_string())
    }
}

/// Simple error type for service handlers.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Bad request
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Not found
    #[error("Not found: {0}")]
    NotFound(String),
}

impl From<Error> for CloudError {
    fn from(err: Error) -> Self {
        match err {
            Error::BadRequest(msg) => CloudError::Validation(msg),
            Error::Internal(msg) => CloudError::Internal(msg),
            Error::NotFound(msg) => CloudError::NotFound {
                resource_type: "Resource".to_string(),
                resource_id: msg,
            },
        }
    }
}
