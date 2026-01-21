//! Custom provider example for CloudKit.
//!
//! This example demonstrates how to implement custom SPIs.
//!
//! Run with: `cargo run --example custom_provider`

use async_trait::async_trait;
use cloudkit::prelude::*;
use std::time::Duration;

/// Custom authentication provider that uses a vault.
struct VaultAuthProvider {
    vault_url: String,
    role: String,
}

impl VaultAuthProvider {
    fn new(vault_url: impl Into<String>, role: impl Into<String>) -> Self {
        Self {
            vault_url: vault_url.into(),
            role: role.into(),
        }
    }
}

#[async_trait]
impl AuthProvider for VaultAuthProvider {
    async fn get_credentials(&self) -> CloudResult<Credentials> {
        println!("  Fetching credentials from Vault at {}", self.vault_url);
        println!("  Using role: {}", self.role);
        
        // In a real implementation, you would:
        // 1. Authenticate with Vault
        // 2. Read secrets from the appropriate path
        // 3. Return the credentials
        
        // For demo purposes, return dummy credentials
        Ok(Credentials::new("vault-access-key", "vault-secret-key"))
    }

    async fn refresh_credentials(&self) -> CloudResult<Credentials> {
        println!("  Refreshing credentials from Vault...");
        self.get_credentials().await
    }
}

/// Custom retry policy with circuit breaker logic.
struct CircuitBreakerRetry {
    max_failures: u32,
    current_failures: std::sync::atomic::AtomicU32,
    circuit_open: std::sync::atomic::AtomicBool,
}

impl CircuitBreakerRetry {
    fn new(max_failures: u32) -> Self {
        Self {
            max_failures,
            current_failures: std::sync::atomic::AtomicU32::new(0),
            circuit_open: std::sync::atomic::AtomicBool::new(false),
        }
    }
}

#[async_trait]
impl RetryPolicy for CircuitBreakerRetry {
    fn should_retry(&self, error: &CloudError, attempt: u32) -> RetryDecision {
        use std::sync::atomic::Ordering;
        
        // Check if circuit is open
        if self.circuit_open.load(Ordering::Relaxed) {
            println!("  Circuit breaker OPEN - not retrying");
            return RetryDecision::DoNotRetry;
        }

        // Check if we've exceeded max attempts
        if attempt >= self.max_attempts() {
            return RetryDecision::DoNotRetry;
        }

        // Increment failure count
        let failures = self.current_failures.fetch_add(1, Ordering::Relaxed) + 1;
        
        if failures >= self.max_failures {
            println!("  Too many failures ({}) - opening circuit", failures);
            self.circuit_open.store(true, Ordering::Relaxed);
            return RetryDecision::DoNotRetry;
        }

        // Only retry network-related errors
        match error {
            CloudError::Network(_) | CloudError::Timeout { .. } => {
                let delay = Duration::from_millis(100 * 2u64.pow(attempt));
                println!("  Retrying in {:?} (attempt {})", delay, attempt);
                RetryDecision::Retry(delay)
            }
            _ => RetryDecision::DoNotRetry,
        }
    }

    fn max_attempts(&self) -> u32 {
        5
    }
}

/// Custom metrics collector that prints to console.
struct ConsoleMetrics;

#[async_trait]
impl MetricsCollector for ConsoleMetrics {
    async fn record(&self, metrics: OperationMetrics) {
        println!(
            "  📊 [{}] {}.{} completed in {:?} ({:?})",
            metrics.provider,
            metrics.service,
            metrics.operation,
            metrics.duration,
            metrics.outcome
        );
    }

    async fn increment_counter(&self, name: &str, value: u64, tags: &[(&str, &str)]) {
        println!("  📊 Counter {} += {} {:?}", name, value, tags);
    }

    async fn record_gauge(&self, name: &str, value: f64, tags: &[(&str, &str)]) {
        println!("  📊 Gauge {} = {} {:?}", name, value, tags);
    }

    async fn record_histogram(&self, name: &str, value: f64, tags: &[(&str, &str)]) {
        println!("  📊 Histogram {} = {} {:?}", name, value, tags);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CloudKit Custom Provider Example");
    println!("=================================\n");

    // Demonstrate custom auth provider
    println!("1. Custom Auth Provider (Vault)");
    println!("-------------------------------");
    let vault_auth = VaultAuthProvider::new("https://vault.example.com", "cloud-reader");
    let creds = vault_auth.get_credentials().await?;
    println!("  ✓ Got credentials: {}...", &creds.access_key[..5]);

    // Demonstrate custom retry policy
    println!("\n2. Custom Retry Policy (Circuit Breaker)");
    println!("-----------------------------------------");
    let retry = CircuitBreakerRetry::new(3);
    
    let network_error = CloudError::Network(
        NetworkError::Connection("timeout".to_string())
    );
    
    match retry.should_retry(&network_error, 0) {
        RetryDecision::Retry(delay) => println!("  Would retry after {:?}", delay),
        RetryDecision::DoNotRetry => println!("  Would not retry"),
    }

    // Demonstrate custom metrics
    println!("\n3. Custom Metrics Collector");
    println!("---------------------------");
    let metrics = ConsoleMetrics;
    
    metrics.record(OperationMetrics::success(
        "aws",
        "s3",
        "put_object",
        Duration::from_millis(150),
    )).await;

    println!("\n✓ Custom provider example complete!");

    Ok(())
}
