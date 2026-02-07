//! Error handling patterns example for CloudKit.
//!
//! This example demonstrates various error handling strategies.
//!
//! Run with: `cargo run --example error_handling`

use cloudkit::prelude::*;
use std::time::Duration;

/// Pattern 1: Basic error handling with match
async fn basic_error_handling<S: ObjectStorage>(storage: &S) {
    println!("Pattern 1: Basic Error Handling");
    println!("================================\n");

    match storage.get_object("bucket", "nonexistent-key").await {
        Ok(data) => {
            println!("  Downloaded {} bytes", data.len());
        }
        Err(CloudError::NotFound { resource_type, resource_id }) => {
            println!("  {} not found: {}", resource_type, resource_id);
            println!("  (This is expected for this example)");
        }
        Err(CloudError::Auth(e)) => {
            println!("  Authentication error: {}", e);
        }
        Err(CloudError::Network(e)) => {
            println!("  Network error: {}", e);
        }
        Err(e) => {
            println!("  Other error: {}", e);
        }
    }
}

/// Pattern 2: Using ? operator with custom error types
#[derive(Debug)]
enum AppError {
    Cloud(CloudError),
    InvalidFormat(String),
    MissingData(String),
}

impl From<CloudError> for AppError {
    fn from(e: CloudError) -> Self {
        AppError::Cloud(e)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Cloud(e) => write!(f, "Cloud error: {}", e),
            AppError::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
            AppError::MissingData(s) => write!(f, "Missing data: {}", s),
        }
    }
}

async fn app_operation<S: ObjectStorage>(storage: &S, bucket: &str, key: &str) -> Result<String, AppError> {
    // The ? operator automatically converts CloudError to AppError
    let data = storage.get_object(bucket, key).await?;
    
    let content = String::from_utf8(data.to_vec())
        .map_err(|_| AppError::InvalidFormat("Not valid UTF-8".to_string()))?;
    
    if content.is_empty() {
        return Err(AppError::MissingData("File is empty".to_string()));
    }
    
    Ok(content)
}

/// Pattern 3: Fallback values on error
async fn with_fallback<S: ObjectStorage>(storage: &S, bucket: &str, key: &str, default: &str) -> String {
    storage
        .get_object(bucket, key)
        .await
        .map(|b| String::from_utf8_lossy(&b).to_string())
        .unwrap_or_else(|e| {
            println!("  Using fallback due to: {}", e);
            default.to_string()
        })
}

/// Pattern 4: Retry with exponential backoff
async fn with_retry<S: ObjectStorage, F, T>(
    storage: &S,
    operation: F,
    max_attempts: u32,
) -> CloudResult<T>
where
    F: Fn(&S) -> std::pin::Pin<Box<dyn std::future::Future<Output = CloudResult<T>> + Send + '_>>,
{
    let mut attempt = 0;
    let mut last_error = None;

    while attempt < max_attempts {
        match operation(storage).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                attempt += 1;
                let delay = Duration::from_millis(100 * 2u64.pow(attempt));
                
                // Only retry retryable errors
                let should_retry = matches!(
                    &e,
                    CloudError::Network(_) | 
                    CloudError::Timeout { .. } | 
                    CloudError::RateLimited { .. } |
                    CloudError::ServiceUnavailable { .. }
                );

                if should_retry && attempt < max_attempts {
                    println!("  Retry attempt {} after {:?}: {}", attempt, delay, e);
                    tokio::time::sleep(delay).await;
                    last_error = Some(e);
                } else {
                    return Err(e);
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| CloudError::Internal("Max retries exceeded".to_string())))
}

/// Pattern 5: Collecting results from multiple operations
async fn batch_download<S: ObjectStorage>(
    storage: &S,
    bucket: &str,
    keys: &[&str],
) -> Vec<(String, Result<bytes::Bytes, CloudError>)> {
    let mut results = Vec::new();
    
    for key in keys {
        let result = storage.get_object(bucket, key).await;
        results.push((key.to_string(), result));
    }
    
    results
}

/// Pattern 6: Early return on first error
async fn download_all_or_fail<S: ObjectStorage>(
    storage: &S,
    bucket: &str,
    keys: &[&str],
) -> CloudResult<Vec<bytes::Bytes>> {
    let mut results = Vec::new();
    
    for key in keys {
        let data = storage.get_object(bucket, key).await?; // Returns early on error
        results.push(data);
    }
    
    Ok(results)
}

/// Pattern 7: Log and continue
async fn resilient_processing<S: ObjectStorage>(
    storage: &S,
    bucket: &str,
    keys: &[&str],
) -> Vec<bytes::Bytes> {
    let mut results = Vec::new();
    
    for key in keys {
        match storage.get_object(bucket, key).await {
            Ok(data) => {
                results.push(data);
            }
            Err(e) => {
                // Log error but continue processing
                tracing::warn!(key = %key, error = %e, "Failed to download, skipping");
            }
        }
    }
    
    results
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging examples
    tracing_subscriber::fmt::init();

    println!("CloudKit Error Handling Patterns");
    println!("=================================\n");

    // Create a simple context for demonstration
    let context = CloudKit::aws()
        .region(Region::aws_us_east_1())
        .build()
        .await?;

    println!("Note: These examples use stub implementations.");
    println!("In production, you would use actual provider clients.\n");

    // Demonstrate Pattern 1: Basic error handling
    // In real code: basic_error_handling(aws.storage()).await;
    
    println!("Pattern 2: Custom Error Types");
    println!("==============================");
    println!("  AppError wraps CloudError for application-specific handling\n");

    println!("Pattern 3: Fallback Values");
    println!("===========================");
    let _fallback = "default content";
    println!("  Use .unwrap_or_else() to provide fallback on error\n");

    println!("Pattern 4: Retry with Backoff");
    println!("==============================");
    println!("  Exponential backoff: 200ms, 400ms, 800ms, etc.\n");

    println!("Pattern 5: Collect All Results");
    println!("===============================");
    println!("  Returns Vec<(key, Result)> - handles each independently\n");

    println!("Pattern 6: Fail Fast");
    println!("=====================");
    println!("  Returns on first error using ? operator\n");

    println!("Pattern 7: Resilient Processing");
    println!("================================");
    println!("  Logs errors but continues processing other items\n");

    println!("✓ Error handling patterns demonstrated!");

    Ok(())
}
