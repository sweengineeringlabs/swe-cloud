//! Error types for CloudKit operations.

use std::fmt;

/// Result type alias for CloudKit operations.
pub type CloudResult<T> = Result<T, CloudError>;

/// Unified error type for all cloud operations.
#[derive(Debug)]
pub enum CloudError {
    /// Authentication or authorization failure
    Auth(AuthError),
    /// Network or connectivity error
    Network(NetworkError),
    /// Resource not found
    NotFound {
        /// Type of resource (bucket, object, queue, etc.)
        resource_type: String,
        /// Identifier of the resource
        resource_id: String,
    },
    /// Resource already exists
    AlreadyExists {
        /// Type of resource
        resource_type: String,
        /// Identifier of the resource
        resource_id: String,
    },
    /// Invalid input or configuration
    Validation(String),
    /// Rate limit exceeded
    RateLimited {
        /// When to retry (if known)
        retry_after: Option<std::time::Duration>,
    },
    /// Service unavailable
    ServiceUnavailable {
        /// Name of the service
        service: String,
        /// Optional message
        message: Option<String>,
    },
    /// Operation timeout
    Timeout {
        /// What operation timed out
        operation: String,
        /// Timeout duration
        duration: std::time::Duration,
    },
    /// Provider-specific error
    Provider {
        /// Provider name (aws, azure, gcp, oracle)
        provider: String,
        /// Error code from the provider
        code: String,
        /// Error message
        message: String,
    },
    /// Internal SDK error
    Internal(String),
    /// Configuration error
    Config(String),
    /// Serialization/deserialization error
    Serialization(String),
    /// IO error
    Io(std::io::Error),
    /// Service-specific error
    ServiceError(String),
}

impl std::error::Error for CloudError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CloudError::Auth(e) => Some(e),
            CloudError::Network(e) => Some(e),
            CloudError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for CloudError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CloudError::Auth(e) => write!(f, "Authentication error: {}", e),
            CloudError::Network(e) => write!(f, "Network error: {}", e),
            CloudError::NotFound { resource_type, resource_id } => {
                write!(f, "{} not found: {}", resource_type, resource_id)
            }
            CloudError::AlreadyExists { resource_type, resource_id } => {
                write!(f, "{} already exists: {}", resource_type, resource_id)
            }
            CloudError::Validation(msg) => write!(f, "Validation error: {}", msg),
            CloudError::RateLimited { retry_after } => {
                match retry_after {
                    Some(d) => write!(f, "Rate limited, retry after {:?}", d),
                    None => write!(f, "Rate limited"),
                }
            }
            CloudError::ServiceUnavailable { service, message } => {
                match message {
                    Some(msg) => write!(f, "Service {} unavailable: {}", service, msg),
                    None => write!(f, "Service {} unavailable", service),
                }
            }
            CloudError::Timeout { operation, duration } => {
                write!(f, "Operation '{}' timed out after {:?}", operation, duration)
            }
            CloudError::Provider { provider, code, message } => {
                write!(f, "[{}] {}: {}", provider, code, message)
            }
            CloudError::Internal(msg) => write!(f, "Internal error: {}", msg),
            CloudError::Config(msg) => write!(f, "Configuration error: {}", msg),
            CloudError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            CloudError::Io(e) => write!(f, "IO error: {}", e),
            CloudError::ServiceError(msg) => write!(f, "Service error: {}", msg),
        }
    }
}

impl From<std::io::Error> for CloudError {
    fn from(err: std::io::Error) -> Self {
        CloudError::Io(err)
    }
}

impl From<serde_json::Error> for CloudError {
    fn from(err: serde_json::Error) -> Self {
        CloudError::Serialization(err.to_string())
    }
}

impl From<reqwest::Error> for CloudError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            CloudError::Timeout {
                operation: "HTTP request".to_string(),
                duration: std::time::Duration::from_secs(30),
            }
        } else if err.is_connect() {
            CloudError::Network(NetworkError::Connection(err.to_string()))
        } else {
            CloudError::Network(NetworkError::Request(err.to_string()))
        }
    }
}

/// Authentication-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// Missing credentials
    #[error("Missing credentials: {0}")]
    MissingCredentials(String),

    /// Invalid credentials
    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),

    /// Expired credentials
    #[error("Credentials expired")]
    ExpiredCredentials,

    /// Insufficient permissions
    #[error("Insufficient permissions: {0}")]
    InsufficientPermissions(String),

    /// Token refresh failed
    #[error("Failed to refresh token: {0}")]
    TokenRefreshFailed(String),
}

/// Network-specific errors.
#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
    /// Connection failed
    #[error("Connection failed: {0}")]
    Connection(String),

    /// DNS resolution failed
    #[error("DNS resolution failed: {0}")]
    DnsResolution(String),

    /// TLS/SSL error
    #[error("TLS error: {0}")]
    Tls(String),

    /// Request failed
    #[error("Request failed: {0}")]
    Request(String),

    /// Response parsing failed
    #[error("Failed to parse response: {0}")]
    ResponseParsing(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloud_error_display() {
        let err = CloudError::NotFound {
            resource_type: "Bucket".to_string(),
            resource_id: "my-bucket".to_string(),
        };
        assert_eq!(err.to_string(), "Bucket not found: my-bucket");
    }

    #[test]
    fn test_provider_error() {
        let err = CloudError::Provider {
            provider: "aws".to_string(),
            code: "NoSuchBucket".to_string(),
            message: "The specified bucket does not exist".to_string(),
        };
        assert!(err.to_string().contains("aws"));
        assert!(err.to_string().contains("NoSuchBucket"));
    }
}
