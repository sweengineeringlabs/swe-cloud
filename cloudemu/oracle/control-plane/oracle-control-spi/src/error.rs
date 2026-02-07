//! Oracle control-plane error types.

/// Oracle error type.
#[derive(Debug, Clone)]
pub enum Error {
    /// Internal error
    Internal(String),
    /// Not found error
    NotFound(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Internal(msg) => write!(f, "Internal error: {}", msg),
            Error::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Result type for Oracle cloud operations.
pub type CloudResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
