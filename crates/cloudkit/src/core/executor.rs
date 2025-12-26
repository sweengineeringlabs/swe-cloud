//! Operation executor with retry and metrics.

use crate::common::{CloudError, CloudResult};
use crate::spi::{MetricsCollector, OperationMetrics, OperationOutcome, RetryDecision, RetryPolicy};
use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Executor for cloud operations with retry and metrics support.
pub struct OperationExecutor {
    provider: String,
    service: String,
    retry_policy: Arc<dyn RetryPolicy>,
    metrics: Arc<dyn MetricsCollector>,
}

impl OperationExecutor {
    /// Create a new operation executor.
    pub fn new(
        provider: impl Into<String>,
        service: impl Into<String>,
        retry_policy: Arc<dyn RetryPolicy>,
        metrics: Arc<dyn MetricsCollector>,
    ) -> Self {
        Self {
            provider: provider.into(),
            service: service.into(),
            retry_policy,
            metrics,
        }
    }

    /// Execute an operation with retry and metrics.
    pub async fn execute<F, Fut, T>(
        &self,
        operation_name: &str,
        operation: F,
    ) -> CloudResult<T>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = CloudResult<T>>,
    {
        let start = Instant::now();
        let mut attempt = 0u32;
        let mut last_error: Option<CloudError> = None;

        loop {
            match operation().await {
                Ok(result) => {
                    let duration = start.elapsed();
                    self.record_success(operation_name, duration, attempt).await;
                    return Ok(result);
                }
                Err(err) => {
                    attempt += 1;
                    
                    match self.retry_policy.should_retry(&err, attempt) {
                        RetryDecision::Retry(delay) => {
                            tracing::warn!(
                                provider = %self.provider,
                                service = %self.service,
                                operation = %operation_name,
                                attempt = %attempt,
                                delay_ms = %delay.as_millis(),
                                error = %err,
                                "Retrying operation"
                            );
                            tokio::time::sleep(delay).await;
                            last_error = Some(err);
                        }
                        RetryDecision::DoNotRetry => {
                            let duration = start.elapsed();
                            self.record_failure(operation_name, duration, attempt, &err).await;
                            return Err(err);
                        }
                    }
                }
            }
        }
    }

    /// Execute an operation without retry.
    pub async fn execute_once<F, Fut, T>(
        &self,
        operation_name: &str,
        operation: F,
    ) -> CloudResult<T>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = CloudResult<T>>,
    {
        let start = Instant::now();

        match operation().await {
            Ok(result) => {
                let duration = start.elapsed();
                self.record_success(operation_name, duration, 0).await;
                Ok(result)
            }
            Err(err) => {
                let duration = start.elapsed();
                self.record_failure(operation_name, duration, 1, &err).await;
                Err(err)
            }
        }
    }

    async fn record_success(&self, operation: &str, duration: Duration, retry_count: u32) {
        let metrics = OperationMetrics::success(
            &self.provider,
            &self.service,
            operation,
            duration,
        ).with_retry_count(retry_count);

        self.metrics.record(metrics).await;
    }

    async fn record_failure(&self, operation: &str, duration: Duration, retry_count: u32, error: &CloudError) {
        let error_code = match error {
            CloudError::NotFound { .. } => "NotFound",
            CloudError::Auth(_) => "AuthError",
            CloudError::Network(_) => "NetworkError",
            CloudError::RateLimited { .. } => "RateLimited",
            CloudError::Timeout { .. } => "Timeout",
            CloudError::Validation(_) => "ValidationError",
            _ => "UnknownError",
        };

        let metrics = OperationMetrics::failure(
            &self.provider,
            &self.service,
            operation,
            duration,
            error_code,
        ).with_retry_count(retry_count);

        self.metrics.record(metrics).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spi::{ExponentialBackoff, NoopMetrics};
    use std::sync::atomic::{AtomicU32, Ordering};

    #[tokio::test]
    async fn test_execute_success() {
        let executor = OperationExecutor::new(
            "test",
            "test",
            Arc::new(ExponentialBackoff::new(3)),
            Arc::new(NoopMetrics),
        );

        let result = executor
            .execute("test_op", || async { Ok::<_, CloudError>(42) })
            .await;

        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_execute_with_retry() {
        let call_count = Arc::new(AtomicU32::new(0));
        let call_count_clone = call_count.clone();

        let executor = OperationExecutor::new(
            "test",
            "test",
            Arc::new(ExponentialBackoff::new(3).with_initial_delay(Duration::from_millis(1))),
            Arc::new(NoopMetrics),
        );

        let result = executor
            .execute("test_op", move || {
                let count = call_count_clone.fetch_add(1, Ordering::SeqCst);
                async move {
                    if count < 2 {
                        Err(CloudError::Network(crate::common::NetworkError::Connection("test".to_string())))
                    } else {
                        Ok::<_, CloudError>(42)
                    }
                }
            })
            .await;

        assert_eq!(result.unwrap(), 42);
        assert_eq!(call_count.load(Ordering::SeqCst), 3);
    }
}
