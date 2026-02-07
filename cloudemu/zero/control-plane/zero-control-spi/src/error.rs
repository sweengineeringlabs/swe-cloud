//! ZeroCloud error types.

/// ZeroCloud Result type.
pub type ZeroResult<T> = Result<T, ZeroError>;

/// ZeroCloud Error types.
#[derive(Debug, thiserror::Error)]
pub enum ZeroError {
    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Not Found
    #[error("Not Found: {0}")]
    NotFound(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Resource already exists
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),

    /// Driver error
    #[error("Driver error: {0}")]
    Driver(String),

    /// Invalid request
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}
