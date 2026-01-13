# 05 - Error Handling

## Document Information

| Field | Value |
|-------|-------|
| **Version** | 1.0.0 |
| **Status** | Design Complete |
| **Last Updated** | 2025-12-26 |

---

## 1. Error Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      Error Hierarchy                             │
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                     CloudError                           │   │
│   │                   (Main Error Type)                      │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│         ┌──────────────────┼──────────────────┐                 │
│         │                  │                  │                 │
│         ▼                  ▼                  ▼                 │
│   ┌───────────┐     ┌───────────┐     ┌───────────┐            │
│   │ AuthError │     │NetworkErr │     │ Provider  │            │
│   │           │     │           │     │   Error   │            │
│   │• Missing  │     │• Connect  │     │           │            │
│   │• Invalid  │     │• DNS      │     │• AWS      │            │
│   │• Expired  │     │• TLS      │     │• Azure    │            │
│   │• Perms    │     │• Timeout  │     │• GCP      │            │
│   └───────────┘     └───────────┘     └───────────┘            │
│                                                                  │
│   Other variants:                                                │
│   • NotFound          • AlreadyExists                           │
│   • Validation        • RateLimited                              │
│   • ServiceUnavailable • Timeout                                 │
│   • Internal          • Config                                   │
│   • Serialization     • Io                                       │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. CloudError Enum

```rust
#[derive(thiserror::Error, Debug)]
pub enum CloudError {
    /// Authentication and authorization errors
    #[error("Authentication error: {0}")]
    Auth(#[from] AuthError),

    /// Network-related errors
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),

    /// Resource not found
    #[error("{resource_type} not found: {resource_id}")]
    NotFound {
        resource_type: String,
        resource_id: String,
    },

    /// Resource already exists
    #[error("{resource_type} already exists: {resource_id}")]
    AlreadyExists {
        resource_type: String,
        resource_id: String,
    },

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Rate limited
    #[error("Rate limited, retry after: {retry_after:?}")]
    RateLimited { retry_after: Option<Duration> },

    /// Service unavailable
    #[error("Service unavailable: {service}")]
    ServiceUnavailable {
        service: String,
        message: Option<String>,
    },

    /// Operation timeout
    #[error("Operation '{operation}' timed out after {duration:?}")]
    Timeout {
        operation: String,
        duration: Duration,
    },

    /// Provider-specific error
    #[error("[{provider}] {code}: {message}")]
    Provider {
        provider: String,
        code: String,
        message: String,
    },

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
```

---

## 3. Error Conversion Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    Error Conversion Flow                         │
│                                                                  │
│   Provider-Specific Error                                        │
│   (aws_sdk_s3::Error, azure_core::Error, etc.)                  │
│           │                                                      │
│           ▼                                                      │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │            Provider Crate Conversion                     │   │
│   │                                                          │   │
│   │   impl From<aws_sdk_s3::Error> for CloudError {         │   │
│   │       fn from(err: aws_sdk_s3::Error) -> Self {         │   │
│   │           match err {                                    │   │
│   │               NoSuchKey => CloudError::NotFound { .. }  │   │
│   │               NoSuchBucket => CloudError::NotFound{..}  │   │
│   │               AccessDenied => CloudError::Auth(..)      │   │
│   │               _ => CloudError::Provider { .. }          │   │
│   │           }                                              │   │
│   │       }                                                  │   │
│   │   }                                                      │   │
│   │                                                          │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│                            ▼                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                    CloudError                            │   │
│   │              (Unified Error Type)                        │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │                                     │
│                            ▼                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                  Application Code                        │   │
│   │                                                          │   │
│   │   match error {                                          │   │
│   │       CloudError::NotFound { .. } => // Handle 404      │   │
│   │       CloudError::Auth(_) => // Handle auth             │   │
│   │       _ => // Handle other                               │   │
│   │   }                                                      │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 4. Error Categorization

### Retryable Errors

```
┌─────────────────────────────────────────────────────────────────┐
│                    Retryable vs Non-Retryable                    │
│                                                                  │
│   ┌──────────────────────────┐  ┌──────────────────────────┐   │
│   │     RETRYABLE            │  │    NON-RETRYABLE         │   │
│   │                          │  │                          │   │
│   │  ✓ Network::Connection   │  │  ✗ Auth::InvalidCreds    │   │
│   │  ✓ Network::Timeout      │  │  ✗ Auth::NoPermission    │   │
│   │  ✓ RateLimited           │  │  ✗ Validation            │   │
│   │  ✓ ServiceUnavailable    │  │  ✗ NotFound (usually)    │   │
│   │  ✓ Timeout               │  │  ✗ AlreadyExists         │   │
│   │  ✓ Provider (5xx)        │  │  ✗ Config                │   │
│   │                          │  │                          │   │
│   └──────────────────────────┘  └──────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Is Retryable Check

```rust
impl CloudError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            CloudError::Network(NetworkError::Connection(_))
                | CloudError::Network(NetworkError::Timeout(_))
                | CloudError::RateLimited { .. }
                | CloudError::ServiceUnavailable { .. }
                | CloudError::Timeout { .. }
        )
    }
}
```

---

## 5. Error Handling Patterns

### Pattern 1: Match on Specific Errors

```
┌─────────────────────────────────────────────────────────────────┐
│   match storage.get_object("bucket", "key").await {             │
│       Ok(data) => {                                              │
│           // Process data                                        │
│       }                                                          │
│       Err(CloudError::NotFound { .. }) => {                     │
│           // Create default or return empty                      │
│       }                                                          │
│       Err(CloudError::Auth(AuthError::ExpiredCredentials)) => { │
│           // Refresh credentials and retry                       │
│       }                                                          │
│       Err(e) => {                                                │
│           return Err(e.into());                                  │
│       }                                                          │
│   }                                                              │
└─────────────────────────────────────────────────────────────────┘
```

### Pattern 2: Propagate with Context

```
┌─────────────────────────────────────────────────────────────────┐
│   let data = storage                                             │
│       .get_object("bucket", "key")                              │
│       .await                                                     │
│       .map_err(|e| format!("Failed to load config: {}", e))?;   │
└─────────────────────────────────────────────────────────────────┘
```

### Pattern 3: Fallback on Error

```
┌─────────────────────────────────────────────────────────────────┐
│   let data = storage                                             │
│       .get_object("bucket", "key")                              │
│       .await                                                     │
│       .unwrap_or_else(|_| Bytes::from_static(b"default"));      │
└─────────────────────────────────────────────────────────────────┘
```

### Pattern 4: Retry on Retryable

```
┌─────────────────────────────────────────────────────────────────┐
│   async fn with_retry<T, F, Fut>(f: F, max: u32) -> Result<T>   │
│   where F: Fn() -> Fut, Fut: Future<Output = CloudResult<T>> {  │
│       let mut attempt = 0;                                       │
│       loop {                                                     │
│           match f().await {                                      │
│               Ok(v) => return Ok(v),                             │
│               Err(e) if e.is_retryable() && attempt < max => {  │
│                   attempt += 1;                                  │
│                   sleep(Duration::from_millis(100 * 2^attempt)); │
│               }                                                  │
│               Err(e) => return Err(e),                          │
│           }                                                      │
│       }                                                          │
│   }                                                              │
└─────────────────────────────────────────────────────────────────┘
```

---

## 6. Provider Error Mapping

### AWS Error Mapping

| AWS Error | CloudError |
|-----------|------------|
| `NoSuchKey` | `NotFound { resource_type: "Object" }` |
| `NoSuchBucket` | `NotFound { resource_type: "Bucket" }` |
| `BucketAlreadyExists` | `AlreadyExists { resource_type: "Bucket" }` |
| `AccessDenied` | `Auth(AuthError::InsufficientPermissions)` |
| `InvalidAccessKeyId` | `Auth(AuthError::InvalidCredentials)` |
| `ExpiredToken` | `Auth(AuthError::ExpiredCredentials)` |
| `Throttling` | `RateLimited { retry_after }` |
| `ServiceUnavailable` | `ServiceUnavailable { service: "aws" }` |

### Azure Error Mapping

| Azure Error | CloudError |
|-------------|------------|
| `BlobNotFound` | `NotFound { resource_type: "Blob" }` |
| `ContainerNotFound` | `NotFound { resource_type: "Container" }` |
| `ContainerAlreadyExists` | `AlreadyExists` |
| `AuthenticationFailed` | `Auth(AuthError::InvalidCredentials)` |
| `AuthorizationFailure` | `Auth(AuthError::InsufficientPermissions)` |

### GCP Error Mapping

| GCP Error | CloudError |
|-----------|------------|
| `NOT_FOUND` | `NotFound` |
| `ALREADY_EXISTS` | `AlreadyExists` |
| `PERMISSION_DENIED` | `Auth(AuthError::InsufficientPermissions)` |
| `UNAUTHENTICATED` | `Auth(AuthError::InvalidCredentials)` |
| `RESOURCE_EXHAUSTED` | `RateLimited` |

---

## 7. Related Documents

- [03-api-design.md](03-api-design.md) - API contracts
- [07-spi-extensions.md](07-spi-extensions.md) - Custom retry policies
