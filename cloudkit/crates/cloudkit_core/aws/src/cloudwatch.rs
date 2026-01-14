//! AWS CloudWatch implementation for metrics and logging.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cloudkit_api::{
    AlarmConfig, AlarmState, LogEvent, LogLevel, LoggingService, MetricDatum, MetricQuery,
    MetricResult, MetricsService, MetricStatistic,
};
use cloudkit_spi::{CloudResult, CloudError};
use cloudkit_spi::CloudContext;
use std::sync::Arc;

/// AWS CloudWatch Metrics implementation.
pub struct CloudWatchMetrics {
    _context: Arc<CloudContext>,
    client: aws_sdk_cloudwatch::Client,
}

impl CloudWatchMetrics {
    /// Create a new CloudWatch Metrics client.
    pub fn new(context: Arc<CloudContext>, sdk_config: aws_config::SdkConfig) -> Self {
        let client = aws_sdk_cloudwatch::Client::new(&sdk_config);
        Self { _context: context, client }
    }
}

#[async_trait]
impl MetricsService for CloudWatchMetrics {
    async fn put_metric_data(&self, namespace: &str, data: Vec<MetricDatum>) -> CloudResult<()> {
        let mut req = self.client.put_metric_data().namespace(namespace);
        
        for datum in data {
            let mut d = aws_sdk_cloudwatch::types::MetricDatum::builder()
                .metric_name(datum.name)
                .value(datum.value);
                
            for (k, v) in datum.dimensions {
                d = d.dimensions(aws_sdk_cloudwatch::types::Dimension::builder().name(k).value(v).build());
            }
            
            req = req.metric_data(d.build());
        }
        
        req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn get_metric_data(
        &self,
        namespace: &str,
        query: MetricQuery,
    ) -> CloudResult<MetricResult> {
        let start_time = query.start_time.unwrap_or_else(|| Utc::now() - chrono::Duration::hours(1));
        let end_time = query.end_time.unwrap_or_else(Utc::now);

        let mut dimensions = Vec::new();
        for (k, v) in query.dimensions {
            dimensions.push(aws_sdk_cloudwatch::types::Dimension::builder().name(k).value(v).build());
        }

        let metric = aws_sdk_cloudwatch::types::Metric::builder()
            .namespace(namespace)
            .metric_name(&query.name)
            .set_dimensions(Some(dimensions))
            .build();

        let stat = match query.statistic {
            MetricStatistic::Average => "Average",
            MetricStatistic::Sum => "Sum",
            MetricStatistic::Minimum => "Minimum",
            MetricStatistic::Maximum => "Maximum",
            MetricStatistic::SampleCount => "SampleCount",
        };

        let data_query = aws_sdk_cloudwatch::types::MetricDataQuery::builder()
            .id("q1")
            .metric_stat(
                aws_sdk_cloudwatch::types::MetricStat::builder()
                    .metric(metric)
                    .period(query.period_seconds as i32)
                    .stat(stat)
                    .build()
            )
            .return_data(true)
            .build();

        let resp = self.client.get_metric_data()
            .metric_data_queries(data_query)
            .start_time(aws_sdk_cloudwatch::primitives::DateTime::from_secs(start_time.timestamp()))
            .end_time(aws_sdk_cloudwatch::primitives::DateTime::from_secs(end_time.timestamp()))
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;

        let mut data_points = Vec::new();
        if let Some(results) = resp.metric_data_results().first() {
            for (v, t) in results.values().iter().zip(results.timestamps().iter()) {
                data_points.push(cloudkit_api::MetricDataPoint {
                    timestamp: chrono::DateTime::<chrono::Utc>::from_timestamp(t.secs(), 0).unwrap_or_default(),
                    value: *v,
                    unit: cloudkit_api::MetricUnit::None, // CloudWatch doesn't always return unit in GetMetricData
                });
            }
        }

        Ok(MetricResult {
            name: query.name,
            data_points,
        })
    }

    async fn list_metrics(&self, namespace: Option<&str>) -> CloudResult<Vec<String>> {
        let mut req = self.client.list_metrics();
        if let Some(ns) = namespace {
            req = req.namespace(ns);
        }
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(resp.metrics().iter().map(|m| m.metric_name().unwrap_or_default().to_string()).collect())
    }

    async fn put_alarm(&self, namespace: &str, config: AlarmConfig) -> CloudResult<()> {
        let mut req = self.client.put_metric_alarm()
            .alarm_name(config.name)
            .namespace(namespace)
            .metric_name(config.metric_name)
            .threshold(config.threshold)
            .evaluation_periods(config.evaluation_periods as i32)
            .period(config.period_seconds as i32);
            
        req = req.comparison_operator(match config.comparison {
            cloudkit_api::ComparisonOperator::GreaterThan => aws_sdk_cloudwatch::types::ComparisonOperator::GreaterThanThreshold,
            cloudkit_api::ComparisonOperator::GreaterThanOrEqual => aws_sdk_cloudwatch::types::ComparisonOperator::GreaterThanOrEqualToThreshold,
            cloudkit_api::ComparisonOperator::LessThan => aws_sdk_cloudwatch::types::ComparisonOperator::LessThanThreshold,
            cloudkit_api::ComparisonOperator::LessThanOrEqual => aws_sdk_cloudwatch::types::ComparisonOperator::LessThanOrEqualToThreshold,
        });

        req = req.statistic(match config.statistic {
            MetricStatistic::Average => aws_sdk_cloudwatch::types::Statistic::Average,
            MetricStatistic::Sum => aws_sdk_cloudwatch::types::Statistic::Sum,
            MetricStatistic::Minimum => aws_sdk_cloudwatch::types::Statistic::Minimum,
            MetricStatistic::Maximum => aws_sdk_cloudwatch::types::Statistic::Maximum,
            MetricStatistic::SampleCount => aws_sdk_cloudwatch::types::Statistic::SampleCount,
        });

        for (k, v) in config.dimensions {
            req = req.dimensions(aws_sdk_cloudwatch::types::Dimension::builder().name(k).value(v).build());
        }
            
        req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn delete_alarm(&self, name: &str) -> CloudResult<()> {
        self.client.delete_alarms()
            .alarm_names(name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn get_alarm_state(&self, name: &str) -> CloudResult<AlarmState> {
        let resp = self.client.describe_alarms()
            .alarm_names(name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        if let Some(alarm) = resp.metric_alarms().first() {
            Ok(match alarm.state_value().unwrap() {
                aws_sdk_cloudwatch::types::StateValue::Ok => AlarmState::Ok,
                aws_sdk_cloudwatch::types::StateValue::Alarm => AlarmState::Alarm,
                aws_sdk_cloudwatch::types::StateValue::InsufficientData => AlarmState::InsufficientData,
                _ => AlarmState::Ok,
            })
        } else {
            Err(CloudError::NotFound {
                resource_type: "Alarm".to_string(),
                resource_id: name.to_string(),
            })
        }
    }
}

/// AWS CloudWatch Logs implementation.
pub struct CloudWatchLogs {
    _context: Arc<CloudContext>,
    client: aws_sdk_cloudwatchlogs::Client,
}

impl CloudWatchLogs {
    /// Create a new CloudWatch Logs client.
    pub fn new(context: Arc<CloudContext>, sdk_config: aws_config::SdkConfig) -> Self {
        let client = aws_sdk_cloudwatchlogs::Client::new(&sdk_config);
        Self { _context: context, client }
    }
}

#[async_trait]
impl LoggingService for CloudWatchLogs {
    async fn create_log_group(&self, name: &str) -> CloudResult<()> {
        self.client.create_log_group()
            .log_group_name(name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn delete_log_group(&self, name: &str) -> CloudResult<()> {
        self.client.delete_log_group()
            .log_group_name(name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn create_log_stream(&self, group: &str, stream: &str) -> CloudResult<()> {
        self.client.create_log_stream()
            .log_group_name(group)
            .log_stream_name(stream)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn put_log_events(
        &self,
        group: &str,
        stream: &str,
        events: Vec<LogEvent>,
    ) -> CloudResult<()> {
        let mut log_events = Vec::new();
        for event in events {
            log_events.push(aws_sdk_cloudwatchlogs::types::InputLogEvent::builder()
                .message(event.message)
                .timestamp(event.timestamp.timestamp_millis())
                .build()
                .unwrap());
        }
        
        self.client.put_log_events()
            .log_group_name(group)
            .log_stream_name(stream)
            .set_log_events(Some(log_events))
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn query_logs(
        &self,
        group: &str,
        query: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> CloudResult<Vec<LogEvent>> {
        let resp = self.client.start_query()
            .log_group_name(group)
            .query_string(query)
            .start_time(start_time.timestamp())
            .end_time(end_time.timestamp())
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;

        let query_id = resp.query_id().ok_or_else(|| CloudError::ServiceError("StartQuery failed to return query ID".to_string()))?;

        // Polling loop for results
        for _ in 0..20 { // Max 20 attempts
            let results = self.client.get_query_results()
                .query_id(query_id)
                .send()
                .await
                .map_err(|e| CloudError::ServiceError(e.to_string()))?;

            match results.status() {
                Some(&aws_sdk_cloudwatchlogs::types::QueryStatus::Complete) => {
                    let mut events = Vec::new();
                    for result in results.results() {
                        let mut message = String::new();
                        let mut timestamp = Utc::now();
                        let mut fields = std::collections::HashMap::new();
                        let mut log_level = LogLevel::Info;

                        for field in result {
                            let name = field.field().unwrap_or_default();
                            let value = field.value().unwrap_or_default();

                            match name {
                                "@message" => message = value.to_string(),
                                "@timestamp" => {
                                    // AWS returns timestamp as String in query results
                                    // Format can be "YYYY-MM-DD HH:MM:SS.SSS" or milliseconds since epoch
                                    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S%.3f") {
                                        timestamp = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, Utc);
                                    } else if let Ok(millis) = value.parse::<i64>() {
                                        if let Some(dt) = chrono::DateTime::<chrono::Utc>::from_timestamp(millis / 1000, (millis % 1000) as u32 * 1_000_000) {
                                            timestamp = dt;
                                        }
                                    }
                                },
                                "@level" | "level" | "status" => {
                                    // Try to map status/level
                                    let val_upper = value.to_uppercase();
                                    if val_upper.contains("ERROR") || val_upper.contains("FAIL") || val_upper.contains("CRIT") {
                                        log_level = LogLevel::Error;
                                    } else if val_upper.contains("WARN") {
                                        log_level = LogLevel::Warn;
                                    } else if val_upper.contains("DEBUG") {
                                        log_level = LogLevel::Debug;
                                    } else if val_upper.contains("TRACE") {
                                        log_level = LogLevel::Trace;
                                    } else if val_upper.contains("FATAL") {
                                        log_level = LogLevel::Fatal;
                                    }
                                    fields.insert(name.to_string(), value.to_string());
                                }
                                _ => { fields.insert(name.to_string(), value.to_string()); }
                            }
                        }

                        events.push(LogEvent {
                            message,
                            timestamp,
                            level: log_level,
                            fields,
                        });
                    }
                    return Ok(events);
                },
                Some(&aws_sdk_cloudwatchlogs::types::QueryStatus::Failed) | Some(&aws_sdk_cloudwatchlogs::types::QueryStatus::Cancelled) => {
                    return Err(CloudError::ServiceError(format!("Query failed with status: {:?}", results.status())));
                },
                _ => {
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
            }
        }

        Err(CloudError::Timeout {
            operation: "query_logs".to_string(),
            duration: std::time::Duration::from_secs(10),
        })
    }

    async fn list_log_groups(&self) -> CloudResult<Vec<String>> {
        let resp = self.client.describe_log_groups()
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.log_groups().iter().map(|g| g.log_group_name().unwrap_or_default().to_string()).collect())
    }

    async fn list_log_streams(&self, group: &str) -> CloudResult<Vec<String>> {
        let resp = self.client.describe_log_streams()
            .log_group_name(group)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.log_streams().iter().map(|s| s.log_stream_name().unwrap_or_default().to_string()).collect())
    }
}

/// AWS Monitoring implementation (Metrics + Logs).
pub struct AwsMonitoring {
    metrics: CloudWatchMetrics,
    logs: CloudWatchLogs,
}

impl AwsMonitoring {
    /// Create a new AWS Monitoring client.
    pub fn new(context: Arc<CloudContext>, sdk_config: aws_config::SdkConfig) -> Self {
        Self {
            metrics: CloudWatchMetrics::new(context.clone(), sdk_config.clone()),
            logs: CloudWatchLogs::new(context, sdk_config),
        }
    }

    /// Get the metrics service.
    pub fn metrics(&self) -> &CloudWatchMetrics {
        &self.metrics
    }

    /// Get the logging service.
    pub fn logs(&self) -> &CloudWatchLogs {
        &self.logs
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cloudkit_api::{
        AlarmConfig, ComparisonOperator, LogEvent, MetricDatum, MetricQuery, MetricStatistic,
        MetricUnit,
    };
    use cloudkit_spi::ProviderType;
    use std::collections::HashMap;

    async fn create_test_context() -> (Arc<CloudContext>, aws_config::SdkConfig) {
        let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        (
            Arc::new(
                CloudContext::builder(ProviderType::Aws)
                    .build()
                    .await
                    .unwrap(),
            ),
            sdk_config,
        )
    }

    // CloudWatch Metrics Tests

    #[tokio::test]
    async fn test_cloudwatch_metrics_new() {
        let (context, sdk_config) = create_test_context().await;
        let _metrics = CloudWatchMetrics::new(context, sdk_config);
    }

    #[tokio::test]
    async fn test_put_metric_data() {
        let (context, sdk_config) = create_test_context().await;
        let metrics = CloudWatchMetrics::new(context, sdk_config);

        let data = vec![
            MetricDatum::new("CPUUtilization", 75.5).with_unit(MetricUnit::Percent),
            MetricDatum::new("MemoryUsage", 1024.0).with_unit(MetricUnit::Megabytes),
        ];

        let result = metrics.put_metric_data("MyApp", data).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_metric_data() {
        let (context, sdk_config) = create_test_context().await;
        let metrics = CloudWatchMetrics::new(context, sdk_config);

        let query = MetricQuery {
            name: "CPUUtilization".to_string(),
            start_time: None,
            end_time: None,
            period_seconds: 60,
            statistic: MetricStatistic::Average,
            dimensions: HashMap::new(),
        };

        let result = metrics.get_metric_data("MyApp", query).await;
        assert!(result.is_ok());
        let metric_result = result.unwrap();
        assert_eq!(metric_result.name, "CPUUtilization");
    }

    #[tokio::test]
    async fn test_list_metrics() {
        let (context, sdk_config) = create_test_context().await;
        let metrics = CloudWatchMetrics::new(context, sdk_config);

        let result = metrics.list_metrics(Some("MyApp")).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_put_alarm() {
        let (context, sdk_config) = create_test_context().await;
        let metrics = CloudWatchMetrics::new(context, sdk_config);

        let config = AlarmConfig {
            name: "HighCPU".to_string(),
            description: Some("CPU too high".to_string()),
            metric_name: "CPUUtilization".to_string(),
            threshold: 80.0,
            comparison: ComparisonOperator::GreaterThan,
            evaluation_periods: 3,
            period_seconds: 300,
            statistic: MetricStatistic::Average,
            dimensions: HashMap::new(),
        };

        let result = metrics.put_alarm("MyApp", config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_alarm() {
        let (context, sdk_config) = create_test_context().await;
        let metrics = CloudWatchMetrics::new(context, sdk_config);

        let result = metrics.delete_alarm("HighCPU").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_alarm_state() {
        let (context, sdk_config) = create_test_context().await;
        let metrics = CloudWatchMetrics::new(context, sdk_config);

        let result = metrics.get_alarm_state("HighCPU").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), AlarmState::Ok);
    }

    // CloudWatch Logs Tests

    #[tokio::test]
    async fn test_cloudwatch_logs_new() {
        let (context, sdk_config) = create_test_context().await;
        let _logs = CloudWatchLogs::new(context, sdk_config);
    }

    #[tokio::test]
    async fn test_create_log_group() {
        let (context, sdk_config) = create_test_context().await;
        let logs = CloudWatchLogs::new(context, sdk_config);

        let result = logs.create_log_group("/aws/lambda/my-function").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_log_group() {
        let (context, sdk_config) = create_test_context().await;
        let logs = CloudWatchLogs::new(context, sdk_config);

        let result = logs.delete_log_group("/aws/lambda/my-function").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_log_stream() {
        let (context, sdk_config) = create_test_context().await;
        let logs = CloudWatchLogs::new(context, sdk_config);

        let result = logs
            .create_log_stream("/aws/lambda/my-function", "2024/01/01/[$LATEST]abc123")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_put_log_events() {
        let (context, sdk_config) = create_test_context().await;
        let logs = CloudWatchLogs::new(context, sdk_config);

        let events = vec![
            LogEvent::new("Starting execution"),
            LogEvent::new("Processing complete"),
        ];

        let result = logs
            .put_log_events("/aws/lambda/my-function", "stream-1", events)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_query_logs() {
        let (context, sdk_config) = create_test_context().await;
        let logs = CloudWatchLogs::new(context, sdk_config);

        let result = logs
            .query_logs(
                "/aws/lambda/my-function",
                "fields @timestamp, @message | limit 10",
                Utc::now() - chrono::Duration::hours(1),
                Utc::now(),
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_log_groups() {
        let (context, sdk_config) = create_test_context().await;
        let logs = CloudWatchLogs::new(context, sdk_config);

        let result = logs.list_log_groups().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_log_streams() {
        let (context, sdk_config) = create_test_context().await;
        let logs = CloudWatchLogs::new(context, sdk_config);

        let result = logs.list_log_streams("/aws/lambda/my-function").await;
        assert!(result.is_ok());
    }
}

