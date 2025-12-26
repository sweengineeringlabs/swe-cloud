//! Retry policy SPI.

use crate::common::CloudError;
use async_trait::async_trait;
use std::time::Duration;

/// Retry decision.
#[derive(Debug, Clone)]
pub enum RetryDecision {
    /// Retry after the specified delay.
    Retry(Duration),
    /// Do not retry.
    DoNotRetry,
}

/// Retry policy trait for custom retry strategies.
///
/// Implement this trait to customize how failed operations are retried.
#[async_trait]
pub trait RetryPolicy: Send + Sync {
    /// Determine if an error should be retried.
    fn should_retry(&self, error: &CloudError, attempt: u32) -> RetryDecision;

    /// Get the maximum number of retry attempts.
    fn max_attempts(&self) -> u32;
}

/// Exponential backoff retry policy.
#[derive(Debug, Clone)]
pub struct ExponentialBackoff {
    /// Initial delay
    pub initial_delay: Duration,
    /// Maximum delay
    pub max_delay: Duration,
    /// Maximum number of attempts
    pub max_attempts: u32,
    /// Backoff multiplier
    pub multiplier: f64,
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            max_attempts: 3,
            multiplier: 2.0,
        }
    }
}

impl ExponentialBackoff {
    /// Create a new exponential backoff policy.
    pub fn new(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            ..Default::default()
        }
    }

    /// Set the initial delay.
    pub fn with_initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    /// Set the maximum delay.
    pub fn with_max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    /// Set the backoff multiplier.
    pub fn with_multiplier(mut self, multiplier: f64) -> Self {
        self.multiplier = multiplier;
        self
    }

    /// Calculate delay for a given attempt.
    fn calculate_delay(&self, attempt: u32) -> Duration {
        let delay_ms = self.initial_delay.as_millis() as f64 
            * self.multiplier.powi(attempt as i32);
        let delay = Duration::from_millis(delay_ms as u64);
        std::cmp::min(delay, self.max_delay)
    }
}

#[async_trait]
impl RetryPolicy for ExponentialBackoff {
    fn should_retry(&self, error: &CloudError, attempt: u32) -> RetryDecision {
        if attempt >= self.max_attempts {
            return RetryDecision::DoNotRetry;
        }

        // Determine if error is retryable
        let is_retryable = matches!(
            error,
            CloudError::Network(_)
            | CloudError::RateLimited { .. }
            | CloudError::ServiceUnavailable { .. }
            | CloudError::Timeout { .. }
        );

        if is_retryable {
            RetryDecision::Retry(self.calculate_delay(attempt))
        } else {
            RetryDecision::DoNotRetry
        }
    }

    fn max_attempts(&self) -> u32 {
        self.max_attempts
    }
}

/// No retry policy.
#[derive(Debug, Clone, Default)]
pub struct NoRetry;

#[async_trait]
impl RetryPolicy for NoRetry {
    fn should_retry(&self, _error: &CloudError, _attempt: u32) -> RetryDecision {
        RetryDecision::DoNotRetry
    }

    fn max_attempts(&self) -> u32 {
        1
    }
}

/// Fixed delay retry policy.
#[derive(Debug, Clone)]
pub struct FixedDelay {
    /// Fixed delay between retries
    pub delay: Duration,
    /// Maximum number of attempts
    pub max_attempts: u32,
}

impl FixedDelay {
    /// Create a new fixed delay policy.
    pub fn new(delay: Duration, max_attempts: u32) -> Self {
        Self { delay, max_attempts }
    }
}

#[async_trait]
impl RetryPolicy for FixedDelay {
    fn should_retry(&self, error: &CloudError, attempt: u32) -> RetryDecision {
        if attempt >= self.max_attempts {
            return RetryDecision::DoNotRetry;
        }

        let is_retryable = matches!(
            error,
            CloudError::Network(_)
            | CloudError::RateLimited { .. }
            | CloudError::ServiceUnavailable { .. }
            | CloudError::Timeout { .. }
        );

        if is_retryable {
            RetryDecision::Retry(self.delay)
        } else {
            RetryDecision::DoNotRetry
        }
    }

    fn max_attempts(&self) -> u32 {
        self.max_attempts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_backoff_delay() {
        let policy = ExponentialBackoff::default();
        
        let delay0 = policy.calculate_delay(0);
        let delay1 = policy.calculate_delay(1);
        let delay2 = policy.calculate_delay(2);
        
        assert_eq!(delay0, Duration::from_millis(100));
        assert_eq!(delay1, Duration::from_millis(200));
        assert_eq!(delay2, Duration::from_millis(400));
    }

    #[test]
    fn test_exponential_backoff_max_delay() {
        let policy = ExponentialBackoff::default()
            .with_max_delay(Duration::from_millis(150));
        
        let delay2 = policy.calculate_delay(2);
        assert_eq!(delay2, Duration::from_millis(150));
    }

    #[test]
    fn test_should_retry_network_error() {
        let policy = ExponentialBackoff::new(3);
        let error = CloudError::Network(crate::common::NetworkError::Connection("test".to_string()));
        
        match policy.should_retry(&error, 0) {
            RetryDecision::Retry(_) => {}
            RetryDecision::DoNotRetry => panic!("Expected retry"),
        }
    }

    #[test]
    fn test_should_not_retry_validation_error() {
        let policy = ExponentialBackoff::new(3);
        let error = CloudError::Validation("invalid input".to_string());
        
        match policy.should_retry(&error, 0) {
            RetryDecision::DoNotRetry => {}
            RetryDecision::Retry(_) => panic!("Expected no retry"),
        }
    }
}
