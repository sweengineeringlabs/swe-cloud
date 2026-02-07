//! # Workflow API
//!
//! Cross-cloud workflow orchestration operations.
//!
//! ## Implementations
//!
//! - **AWS**: Step Functions
//! - **Azure**: Logic Apps / Durable Functions
//! - **GCP**: Workflows

use async_trait::async_trait;
use cloudkit_spi::{CloudResult, Metadata};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Workflow (state machine) definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    /// Workflow name.
    pub name: String,
    /// ARN or resource identifier.
    pub arn: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Workflow type.
    pub workflow_type: WorkflowType,
    /// Definition in ASL (Amazon States Language) or equivalent JSON.
    pub definition: Value,
    /// IAM role ARN for execution.
    pub role_arn: Option<String>,
    /// When created.
    pub created_at: Option<DateTime<Utc>>,
    /// Tags.
    pub tags: Metadata,
}

impl WorkflowDefinition {
    /// Create a new workflow definition.
    pub fn new(name: impl Into<String>, definition: Value) -> Self {
        Self {
            name: name.into(),
            arn: None,
            description: None,
            workflow_type: WorkflowType::Standard,
            definition,
            role_arn: None,
            created_at: None,
            tags: Metadata::new(),
        }
    }
}

/// Workflow type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WorkflowType {
    /// Standard workflow - exactly-once execution, durable.
    #[default]
    Standard,
    /// Express workflow - at-least-once, high throughput.
    Express,
}

/// Workflow execution status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    /// The execution is currently running.
    Running,
    /// The execution completed successfully.
    Succeeded,
    /// The execution failed.
    Failed,
    /// The execution timed out.
    TimedOut,
    /// The execution was aborted by the user.
    Aborted,
    /// The execution is pending redrive (Step Functions specific).
    PendingRedrive,
}

/// Workflow execution details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Execution {
    /// Execution ARN/ID.
    pub execution_id: String,
    /// Workflow ARN.
    pub workflow_arn: String,
    /// Execution name.
    pub name: Option<String>,
    /// Status.
    pub status: ExecutionStatus,
    /// Input JSON.
    pub input: Option<Value>,
    /// Output JSON (if completed).
    pub output: Option<Value>,
    /// Error info (if failed).
    pub error: Option<ExecutionError>,
    /// Start time.
    pub start_time: DateTime<Utc>,
    /// Stop time (if completed).
    pub stop_time: Option<DateTime<Utc>>,
}

/// Execution error details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionError {
    /// Error type/code.
    pub error: String,
    /// Error cause/message.
    pub cause: String,
}

/// Options for starting an execution.
#[derive(Debug, Clone, Default)]
pub struct StartExecutionOptions {
    /// Execution name (for idempotency).
    pub name: Option<String>,
    /// Trace header for distributed tracing.
    pub trace_header: Option<String>,
}

/// Execution history event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEvent {
    /// Event ID.
    pub id: u64,
    /// Event type.
    pub event_type: String,
    /// Timestamp.
    pub timestamp: DateTime<Utc>,
    /// Event details.
    pub details: Value,
}

/// Filter for listing executions.
#[derive(Debug, Clone, Default)]
pub struct ExecutionFilter {
    /// Filter by status.
    pub status: Option<ExecutionStatus>,
    /// Max results.
    pub max_results: Option<u32>,
}

/// Workflow orchestration operations.
#[async_trait]
pub trait WorkflowService: Send + Sync {
    /// Create a new workflow (state machine).
    async fn create_workflow(&self, definition: WorkflowDefinition) -> CloudResult<String>;

    /// Update a workflow definition.
    async fn update_workflow(
        &self,
        workflow_arn: &str,
        definition: Value,
    ) -> CloudResult<()>;

    /// Delete a workflow.
    async fn delete_workflow(&self, workflow_arn: &str) -> CloudResult<()>;

    /// Get workflow details.
    async fn describe_workflow(&self, workflow_arn: &str) -> CloudResult<WorkflowDefinition>;

    /// List all workflows.
    async fn list_workflows(&self) -> CloudResult<Vec<WorkflowDefinition>>;

    /// Start a new execution.
    async fn start_execution(
        &self,
        workflow_arn: &str,
        input: Value,
        options: StartExecutionOptions,
    ) -> CloudResult<Execution>;

    /// Stop a running execution.
    async fn stop_execution(
        &self,
        execution_id: &str,
        error: Option<&str>,
        cause: Option<&str>,
    ) -> CloudResult<()>;

    /// Get execution details.
    async fn describe_execution(&self, execution_id: &str) -> CloudResult<Execution>;

    /// List executions for a workflow.
    async fn list_executions(
        &self,
        workflow_arn: &str,
        filter: ExecutionFilter,
    ) -> CloudResult<Vec<Execution>>;

    /// Get execution history.
    async fn get_execution_history(
        &self,
        execution_id: &str,
    ) -> CloudResult<Vec<HistoryEvent>>;

    /// Send a task success result (for callback patterns).
    async fn send_task_success(
        &self,
        task_token: &str,
        output: Value,
    ) -> CloudResult<()>;

    /// Send a task failure result.
    async fn send_task_failure(
        &self,
        task_token: &str,
        error: &str,
        cause: &str,
    ) -> CloudResult<()>;

    /// Send a heartbeat for long-running tasks.
    async fn send_task_heartbeat(&self, task_token: &str) -> CloudResult<()>;
}

