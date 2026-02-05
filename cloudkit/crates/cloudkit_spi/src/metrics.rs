//! Metrics collector SPI.

use async_trait::async_trait;
use std::time::Duration;

/// Operation outcome for metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationOutcome {
    /// Operation succeeded
    Success,
    /// Operation failed
    Failure,
    /// Operation was retried
    Retry,
}

/// Metrics for a cloud operation.
#[derive(Debug, Clone)]
pub struct OperationMetrics {
    /// Provider name
    pub provider: String,
    /// Service name (e.g., "s3", "dynamodb")
    pub service: String,
    /// Operation name (e.g., "get_object", "put_item")
    pub operation: String,
    /// Duration of the operation
    pub duration: Duration,
    /// Outcome of the operation
    pub outcome: OperationOutcome,
    /// Number of retry attempts
    pub retry_count: u32,
    /// Bytes transferred (if applicable)
    pub bytes_transferred: Option<u64>,
    /// HTTP status code (if applicable)
    pub status_code: Option<u16>,
    /// Error code (if failed)
    pub error_code: Option<String>,
}

impl OperationMetrics {
    /// Create a new successful operation metric.
    pub fn success(
        provider: impl Into<String>,
        service: impl Into<String>,
        operation: impl Into<String>,
        duration: Duration,
    ) -> Self {
        Self {
            provider: provider.into(),
            service: service.into(),
            operation: operation.into(),
            duration,
            outcome: OperationOutcome::Success,
            retry_count: 0,
            bytes_transferred: None,
            status_code: None,
            error_code: None,
        }
    }

    /// Create a new failed operation metric.
    pub fn failure(
        provider: impl Into<String>,
        service: impl Into<String>,
        operation: impl Into<String>,
        duration: Duration,
        error_code: impl Into<String>,
    ) -> Self {
        Self {
            provider: provider.into(),
            service: service.into(),
            operation: operation.into(),
            duration,
            outcome: OperationOutcome::Failure,
            retry_count: 0,
            bytes_transferred: None,
            status_code: None,
            error_code: Some(error_code.into()),
        }
    }

    /// Set the retry count.
    pub fn with_retry_count(mut self, count: u32) -> Self {
        self.retry_count = count;
        self
    }

    /// Set bytes transferred.
    pub fn with_bytes(mut self, bytes: u64) -> Self {
        self.bytes_transferred = Some(bytes);
        self
    }

    /// Set HTTP status code.
    pub fn with_status_code(mut self, code: u16) -> Self {
        self.status_code = Some(code);
        self
    }
}

/// Metrics collector trait for observability integration.
///
/// Implement this trait to send metrics to your observability platform.
#[async_trait]
pub trait MetricsCollector: Send + Sync {
    /// Record an operation metric.
    async fn record(&self, metrics: OperationMetrics);

    /// Increment a counter.
    async fn increment_counter(&self, name: &str, value: u64, tags: &[(&str, &str)]);

    /// Record a gauge value.
    async fn record_gauge(&self, name: &str, value: f64, tags: &[(&str, &str)]);

    /// Record a histogram value.
    async fn record_histogram(&self, name: &str, value: f64, tags: &[(&str, &str)]);
}

/// No-op metrics collector (default).
#[derive(Debug, Default, Clone)]
pub struct NoopMetrics;

#[async_trait]
impl MetricsCollector for NoopMetrics {
    async fn record(&self, _metrics: OperationMetrics) {}
    async fn increment_counter(&self, _name: &str, _value: u64, _tags: &[(&str, &str)]) {}
    async fn record_gauge(&self, _name: &str, _value: f64, _tags: &[(&str, &str)]) {}
    async fn record_histogram(&self, _name: &str, _value: f64, _tags: &[(&str, &str)]) {}
}

/// Logging metrics collector (logs all metrics via tracing).
#[derive(Debug, Default, Clone)]
pub struct LoggingMetrics;

#[async_trait]
impl MetricsCollector for LoggingMetrics {
    async fn record(&self, metrics: OperationMetrics) {
        tracing::info!(
            provider = %metrics.provider,
            service = %metrics.service,
            operation = %metrics.operation,
            duration_ms = %metrics.duration.as_millis(),
            outcome = ?metrics.outcome,
            retry_count = %metrics.retry_count,
            "Cloud operation completed"
        );
    }

    async fn increment_counter(&self, name: &str, value: u64, tags: &[(&str, &str)]) {
        tracing::debug!(counter = %name, value = %value, tags = ?tags, "Counter incremented");
    }

    async fn record_gauge(&self, name: &str, value: f64, tags: &[(&str, &str)]) {
        tracing::debug!(gauge = %name, value = %value, tags = ?tags, "Gauge recorded");
    }

    async fn record_histogram(&self, name: &str, value: f64, tags: &[(&str, &str)]) {
        tracing::debug!(histogram = %name, value = %value, tags = ?tags, "Histogram recorded");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_metrics_success() {
        let metrics = OperationMetrics::success("aws", "s3", "get_object", Duration::from_millis(100));
        assert_eq!(metrics.outcome, OperationOutcome::Success);
        assert_eq!(metrics.retry_count, 0);
    }

    #[test]
    fn test_operation_metrics_with_bytes() {
        let metrics = OperationMetrics::success("aws", "s3", "put_object", Duration::from_millis(200))
            .with_bytes(1024);
        assert_eq!(metrics.bytes_transferred, Some(1024));
    }
}
