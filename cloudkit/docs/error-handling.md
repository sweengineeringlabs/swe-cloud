# Error Handling

CloudKit provides a unified error handling system across all cloud providers.

## CloudError Enum

The main error type is `CloudError`:

```rust
pub enum CloudError {
    Auth(AuthError),
    Network(NetworkError),
    NotFound { resource_type: String, resource_id: String },
    AlreadyExists { resource_type: String, resource_id: String },
    Validation(String),
    RateLimited { retry_after: Option<Duration> },
    ServiceUnavailable { service: String, message: Option<String> },
    Timeout { operation: String, duration: Duration },
    Provider { provider: String, code: String, message: String },
    Internal(String),
    Config(String),
    Serialization(String),
    Io(std::io::Error),
}
```

## Error Categories

### Authentication Errors

```rust
match error {
    CloudError::Auth(AuthError::MissingCredentials(msg)) => {
        eprintln!("Credentials not found: {}", msg);
        // Check environment variables
    }
    CloudError::Auth(AuthError::InvalidCredentials(msg)) => {
        eprintln!("Invalid credentials: {}", msg);
        // Verify credential values
    }
    CloudError::Auth(AuthError::ExpiredCredentials) => {
        eprintln!("Credentials expired");
        // Refresh credentials
    }
    CloudError::Auth(AuthError::InsufficientPermissions(msg)) => {
        eprintln!("Permission denied: {}", msg);
        // Check IAM policies
    }
    _ => {}
}
```

### Network Errors

```rust
match error {
    CloudError::Network(NetworkError::Connection(msg)) => {
        eprintln!("Connection failed: {}", msg);
        // Check internet connectivity
    }
    CloudError::Network(NetworkError::DnsResolution(msg)) => {
        eprintln!("DNS lookup failed: {}", msg);
        // Check DNS settings
    }
    CloudError::Network(NetworkError::Tls(msg)) => {
        eprintln!("TLS error: {}", msg);
        // Check certificates
    }
    CloudError::Timeout { operation, duration } => {
        eprintln!("Operation '{}' timed out after {:?}", operation, duration);
        // Increase timeout or retry
    }
    _ => {}
}
```

### Resource Errors

```rust
match error {
    CloudError::NotFound { resource_type, resource_id } => {
        eprintln!("{} not found: {}", resource_type, resource_id);
        // Create resource or check name
    }
    CloudError::AlreadyExists { resource_type, resource_id } => {
        eprintln!("{} already exists: {}", resource_type, resource_id);
        // Use different name or update existing
    }
    _ => {}
}
```

### Rate Limiting

```rust
match error {
    CloudError::RateLimited { retry_after } => {
        if let Some(duration) = retry_after {
            eprintln!("Rate limited, retry after {:?}", duration);
            tokio::time::sleep(duration).await;
            // Retry operation
        }
    }
    _ => {}
}
```

## Error Handling Patterns

### Match on Specific Errors

```rust
async fn get_or_create_bucket<S: ObjectStorage>(
    storage: &S,
    bucket: &str,
) -> CloudResult<()> {
    match storage.bucket_exists(bucket).await {
        Ok(true) => {
            println!("Bucket exists");
            Ok(())
        }
        Ok(false) => {
            println!("Creating bucket");
            storage.create_bucket(bucket).await
        }
        Err(CloudError::Auth(e)) => {
            eprintln!("Auth error: {}", e);
            Err(CloudError::Auth(e))
        }
        Err(e) => Err(e),
    }
}
```

### Using Result Combinators

```rust
async fn safe_download<S: ObjectStorage>(
    storage: &S,
    bucket: &str,
    key: &str,
) -> Option<bytes::Bytes> {
    storage.get_object(bucket, key).await.ok()
}

async fn download_with_default<S: ObjectStorage>(
    storage: &S,
    bucket: &str,
    key: &str,
    default: &[u8],
) -> bytes::Bytes {
    storage
        .get_object(bucket, key)
        .await
        .unwrap_or_else(|_| bytes::Bytes::from(default.to_vec()))
}
```

### Using ? Operator with Context

```rust
use std::error::Error;

async fn process_file<S: ObjectStorage>(
    storage: &S,
    bucket: &str,
    key: &str,
) -> Result<String, Box<dyn Error>> {
    let data = storage
        .get_object(bucket, key)
        .await
        .map_err(|e| format!("Failed to download {}/{}: {}", bucket, key, e))?;
    
    let content = String::from_utf8(data.to_vec())
        .map_err(|e| format!("Invalid UTF-8 in {}/{}: {}", bucket, key, e))?;
    
    Ok(content)
}
```

### Retry with Custom Policy

```rust
use cloudkit::spi::{RetryPolicy, RetryDecision, ExponentialBackoff};

async fn upload_with_retry<S: ObjectStorage>(
    storage: &S,
    bucket: &str,
    key: &str,
    data: &[u8],
    max_attempts: u32,
) -> CloudResult<()> {
    let policy = ExponentialBackoff::new(max_attempts);
    let mut attempt = 0;

    loop {
        match storage.put_object(bucket, key, data).await {
            Ok(()) => return Ok(()),
            Err(e) => {
                attempt += 1;
                match policy.should_retry(&e, attempt) {
                    RetryDecision::Retry(delay) => {
                        tracing::warn!("Retry {} after {:?}: {}", attempt, delay, e);
                        tokio::time::sleep(delay).await;
                    }
                    RetryDecision::DoNotRetry => return Err(e),
                }
            }
        }
    }
}
```

## Converting Errors

### From Provider Errors

```rust
// CloudKit automatically converts provider-specific errors
// For example, AWS SDK errors become CloudError::Provider

// Manual conversion if needed:
fn from_aws_error(code: &str, message: &str) -> CloudError {
    CloudError::Provider {
        provider: "aws".to_string(),
        code: code.to_string(),
        message: message.to_string(),
    }
}
```

### To Application Errors

```rust
#[derive(Debug)]
enum AppError {
    Cloud(CloudError),
    Business(String),
}

impl From<CloudError> for AppError {
    fn from(e: CloudError) -> Self {
        AppError::Cloud(e)
    }
}

async fn app_operation<S: ObjectStorage>(storage: &S) -> Result<(), AppError> {
    storage.get_object("bucket", "key").await?;  // Auto-converts
    Ok(())
}
```

## Logging Errors

```rust
use tracing::{error, warn};

async fn operation_with_logging<S: ObjectStorage>(storage: &S) -> CloudResult<()> {
    match storage.get_object("bucket", "key").await {
        Ok(data) => {
            tracing::info!("Downloaded {} bytes", data.len());
            Ok(())
        }
        Err(CloudError::NotFound { .. }) => {
            warn!("Object not found, using default");
            Ok(())
        }
        Err(e) => {
            error!(error = ?e, "Failed to download object");
            Err(e)
        }
    }
}
```

## Best Practices

1. **Be Specific**: Match on specific error variants when behavior differs
2. **Log Appropriately**: Log errors at appropriate levels (warn for retryable, error for fatal)
3. **Provide Context**: Add context when propagating errors
4. **Retry Wisely**: Only retry transient errors, use backoff
5. **Fail Fast**: Don't retry validation or permission errors
6. **Clean Up**: Ensure resources are cleaned up on error
