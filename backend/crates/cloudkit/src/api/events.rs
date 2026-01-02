//! # Events API
//!
//! Cross-cloud event bus operations for event-driven architectures.
//!
//! ## Implementations
//!
//! - **AWS**: EventBridge
//! - **Azure**: Event Grid
//! - **GCP**: Eventarc

use async_trait::async_trait;
use crate::common::CloudResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// An event to be published.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unique event ID.
    pub id: String,
    /// Event source (e.g., "myapp.orders").
    pub source: String,
    /// Event type/detail-type (e.g., "OrderCreated").
    pub detail_type: String,
    /// Event payload as JSON.
    pub detail: Value,
    /// Event time.
    pub time: DateTime<Utc>,
    /// Resources associated with the event.
    pub resources: Vec<String>,
    /// Trace header for distributed tracing.
    pub trace_header: Option<String>,
}

impl Event {
    /// Create a new event.
    pub fn new(source: impl Into<String>, detail_type: impl Into<String>, detail: Value) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            source: source.into(),
            detail_type: detail_type.into(),
            detail,
            time: Utc::now(),
            resources: vec![],
            trace_header: None,
        }
    }

    /// Add a resource.
    pub fn with_resource(mut self, resource: impl Into<String>) -> Self {
        self.resources.push(resource.into());
        self
    }

    /// Set trace header.
    pub fn with_trace_header(mut self, header: impl Into<String>) -> Self {
        self.trace_header = Some(header.into());
        self
    }
}

/// Result of publishing events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutEventsResult {
    /// Number of successfully published events.
    pub successful_count: usize,
    /// Number of failed events.
    pub failed_count: usize,
    /// Details of failed entries.
    pub failed_entries: Vec<FailedEntry>,
}

/// A failed event entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedEntry {
    /// Event ID.
    pub event_id: String,
    /// Error code.
    pub error_code: String,
    /// Error message.
    pub error_message: String,
}

/// Event rule for routing events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRule {
    /// Rule name.
    pub name: String,
    /// Description.
    pub description: Option<String>,
    /// Event pattern (JSON) for matching events.
    pub event_pattern: Option<Value>,
    /// Schedule expression (e.g., "rate(5 minutes)").
    pub schedule_expression: Option<String>,
    /// Rule state.
    pub state: RuleState,
    /// ARN or resource identifier.
    pub arn: Option<String>,
}

impl EventRule {
    /// Create a new pattern-based rule.
    pub fn pattern(name: impl Into<String>, pattern: Value) -> Self {
        Self {
            name: name.into(),
            description: None,
            event_pattern: Some(pattern),
            schedule_expression: None,
            state: RuleState::Enabled,
            arn: None,
        }
    }

    /// Create a new schedule-based rule.
    pub fn schedule(name: impl Into<String>, expression: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            event_pattern: None,
            schedule_expression: Some(expression.into()),
            state: RuleState::Enabled,
            arn: None,
        }
    }
}

/// Rule state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RuleState {
    #[default]
    Enabled,
    Disabled,
}

/// Event target for a rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTarget {
    /// Target ID.
    pub id: String,
    /// Target ARN (e.g., Lambda function ARN, SQS queue ARN).
    pub arn: String,
    /// Input transformer template.
    pub input_template: Option<String>,
    /// Input path mappings.
    pub input_paths: HashMap<String, String>,
}

impl EventTarget {
    /// Create a new target.
    pub fn new(id: impl Into<String>, arn: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            arn: arn.into(),
            input_template: None,
            input_paths: HashMap::new(),
        }
    }
}

/// Event bus operations.
#[async_trait]
pub trait EventBus: Send + Sync {
    /// Publish events to the event bus.
    async fn put_events(
        &self,
        bus_name: &str,
        events: Vec<Event>,
    ) -> CloudResult<PutEventsResult>;

    /// Create a custom event bus.
    async fn create_event_bus(&self, name: &str) -> CloudResult<String>;

    /// Delete an event bus.
    async fn delete_event_bus(&self, name: &str) -> CloudResult<()>;

    /// List event buses.
    async fn list_event_buses(&self) -> CloudResult<Vec<String>>;

    /// Create or update an event rule.
    async fn put_rule(&self, bus_name: &str, rule: EventRule) -> CloudResult<String>;

    /// Delete a rule.
    async fn delete_rule(&self, bus_name: &str, rule_name: &str) -> CloudResult<()>;

    /// Enable a rule.
    async fn enable_rule(&self, bus_name: &str, rule_name: &str) -> CloudResult<()>;

    /// Disable a rule.
    async fn disable_rule(&self, bus_name: &str, rule_name: &str) -> CloudResult<()>;

    /// List rules for an event bus.
    async fn list_rules(&self, bus_name: &str) -> CloudResult<Vec<EventRule>>;

    /// Add targets to a rule.
    async fn put_targets(
        &self,
        bus_name: &str,
        rule_name: &str,
        targets: Vec<EventTarget>,
    ) -> CloudResult<()>;

    /// Remove targets from a rule.
    async fn remove_targets(
        &self,
        bus_name: &str,
        rule_name: &str,
        target_ids: &[&str],
    ) -> CloudResult<()>;

    /// List targets for a rule.
    async fn list_targets(
        &self,
        bus_name: &str,
        rule_name: &str,
    ) -> CloudResult<Vec<EventTarget>>;
}
