//! # Monitoring API
//!
//! Cross-cloud monitoring operations for metrics and logs.
//!
//! ## Implementations
//!
//! - **AWS**: CloudWatch + CloudWatch Logs
//! - **Azure**: Azure Monitor
//! - **GCP**: Cloud Monitoring + Cloud Logging

use async_trait::async_trait;
use cloudkit_spi::CloudResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single metric data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDatum {
    /// Metric name.
    pub name: String,
    /// Numeric value.
    pub value: f64,
    /// Unit of measurement.
    pub unit: MetricUnit,
    /// Timestamp of the data point.
    pub timestamp: DateTime<Utc>,
    /// Dimensions (key-value pairs for filtering).
    pub dimensions: HashMap<String, String>,
}

impl MetricDatum {
    /// Create a new metric datum.
    pub fn new(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value,
            unit: MetricUnit::None,
            timestamp: Utc::now(),
            dimensions: HashMap::new(),
        }
    }

    /// Set the unit.
    pub fn with_unit(mut self, unit: MetricUnit) -> Self {
        self.unit = unit;
        self
    }

    /// Add a dimension.
    pub fn with_dimension(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.dimensions.insert(key.into(), value.into());
        self
    }
}

/// Metric units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MetricUnit {
    #[default]
    /// No unit.
    None,
    /// Seconds.
    Seconds,
    /// Milliseconds.
    Milliseconds,
    /// Microseconds.
    Microseconds,
    /// Bytes.
    Bytes,
    /// Kilobytes.
    Kilobytes,
    /// Megabytes.
    Megabytes,
    /// Gigabytes.
    Gigabytes,
    /// Count.
    Count,
    /// Percent.
    Percent,
    /// Bytes per second.
    BytesPerSecond,
    /// Count per second.
    CountPerSecond,
}

/// A log event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    /// Log message.
    pub message: String,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
    /// Log level.
    pub level: LogLevel,
    /// Additional fields.
    pub fields: HashMap<String, String>,
}

impl LogEvent {
    /// Create a new log event.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            timestamp: Utc::now(),
            level: LogLevel::Info,
            fields: HashMap::new(),
        }
    }

    /// Set log level.
    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Add a field.
    pub fn with_field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.insert(key.into(), value.into());
        self
    }
}

/// Log levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LogLevel {
    /// Fine-grained messages for tracing execution flow.
    Trace,
    /// Information for debugging.
    Debug,
    #[default]
    /// General informational messages.
    Info,
    /// Warning messages for potential issues.
    Warn,
    /// Error messages for failed operations.
    Error,
    /// Fatal error messages for critical system failures.
    Fatal,
}

/// Query for retrieving metrics.
#[derive(Debug, Clone, Default)]
pub struct MetricQuery {
    /// Metric name.
    pub name: String,
    /// Start time.
    pub start_time: Option<DateTime<Utc>>,
    /// End time.
    pub end_time: Option<DateTime<Utc>>,
    /// Period in seconds.
    pub period_seconds: u32,
    /// Statistic to retrieve.
    pub statistic: MetricStatistic,
    /// Dimension filters.
    pub dimensions: HashMap<String, String>,
}

/// Metric statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MetricStatistic {
    #[default]
    /// Average value.
    Average,
    /// Sum of all values.
    Sum,
    /// Minimum value.
    Minimum,
    /// Maximum value.
    Maximum,
    /// Number of samples.
    SampleCount,
}

/// Result of a metric query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricResult {
    /// Metric name.
    pub name: String,
    /// Data points.
    pub data_points: Vec<MetricDataPoint>,
}

/// A single data point in query results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDataPoint {
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
    /// Value.
    pub value: f64,
    /// Unit.
    pub unit: MetricUnit,
}

/// Alarm state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlarmState {
    Ok,
    Alarm,
    InsufficientData,
}

/// Alarm configuration.
#[derive(Debug, Clone)]
pub struct AlarmConfig {
    /// Alarm name.
    pub name: String,
    /// Description.
    pub description: Option<String>,
    /// Metric name.
    pub metric_name: String,
    /// Threshold value.
    pub threshold: f64,
    /// Comparison operator.
    pub comparison: ComparisonOperator,
    /// Evaluation periods.
    pub evaluation_periods: u32,
    /// Period in seconds.
    pub period_seconds: u32,
    /// Statistic.
    pub statistic: MetricStatistic,
    /// Dimensions.
    pub dimensions: HashMap<String, String>,
}

/// Comparison operators for alarms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOperator {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

/// Metrics operations.
#[async_trait]
pub trait MetricsService: Send + Sync {
    /// Publish metric data.
    async fn put_metric_data(&self, namespace: &str, data: Vec<MetricDatum>) -> CloudResult<()>;

    /// Query metric data.
    async fn get_metric_data(
        &self,
        namespace: &str,
        query: MetricQuery,
    ) -> CloudResult<MetricResult>;

    /// List available metrics.
    async fn list_metrics(&self, namespace: Option<&str>) -> CloudResult<Vec<String>>;

    /// Create or update an alarm.
    async fn put_alarm(&self, namespace: &str, config: AlarmConfig) -> CloudResult<()>;

    /// Delete an alarm.
    async fn delete_alarm(&self, name: &str) -> CloudResult<()>;

    /// Get alarm state.
    async fn get_alarm_state(&self, name: &str) -> CloudResult<AlarmState>;
}

/// Logging operations.
#[async_trait]
pub trait LoggingService: Send + Sync {
    /// Create a log group.
    async fn create_log_group(&self, name: &str) -> CloudResult<()>;

    /// Delete a log group.
    async fn delete_log_group(&self, name: &str) -> CloudResult<()>;

    /// Create a log stream.
    async fn create_log_stream(&self, group: &str, stream: &str) -> CloudResult<()>;

    /// Put log events.
    async fn put_log_events(
        &self,
        group: &str,
        stream: &str,
        events: Vec<LogEvent>,
    ) -> CloudResult<()>;

    /// Query logs.
    async fn query_logs(
        &self,
        group: &str,
        query: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> CloudResult<Vec<LogEvent>>;

    /// List log groups.
    async fn list_log_groups(&self) -> CloudResult<Vec<String>>;

    /// List log streams in a group.
    async fn list_log_streams(&self, group: &str) -> CloudResult<Vec<String>>;
}

