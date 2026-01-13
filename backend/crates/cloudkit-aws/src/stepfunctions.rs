//! AWS Step Functions implementation.

use async_trait::async_trait;
use cloudkit_spi::api::{
    Execution, ExecutionFilter, ExecutionStatus, HistoryEvent, StartExecutionOptions,
    WorkflowDefinition, WorkflowService, WorkflowType,
};
use cloudkit_spi::common::{CloudError, CloudResult, Metadata};
use cloudkit_spi::core::CloudContext;
use serde_json::Value;
use std::sync::Arc;

/// AWS Step Functions implementation.
pub struct AwsWorkflow {
    _context: Arc<CloudContext>,
    client: aws_sdk_sfn::Client,
}

impl AwsWorkflow {
    /// Create a new Step Functions client.
    pub fn new(context: Arc<CloudContext>, sdk_config: aws_config::SdkConfig) -> Self {
        let client = aws_sdk_sfn::Client::new(&sdk_config);
        Self { _context: context, client }
    }

    fn get_role_arn(&self) -> CloudResult<String> {
        self._context.config.parameters.get("aws.sfn.role_arn")
            .cloned()
            .or_else(|| std::env::var("AWS_SFN_ROLE_ARN").ok())
            .ok_or_else(|| CloudError::Config("Missing Step Functions Role ARN. Set 'aws.sfn.role_arn' in config or 'AWS_SFN_ROLE_ARN' env var.".into()))
    }
}

#[async_trait]
impl WorkflowService for AwsWorkflow {
    async fn create_workflow(&self, definition: WorkflowDefinition) -> CloudResult<String> {
        let mut req = self.client.create_state_machine()
            .name(definition.name)
            .definition(definition.definition.to_string())
            .role_arn(definition.role_arn.unwrap_or(self.get_role_arn()?));
            
        if definition.workflow_type == WorkflowType::Express {
            req = req.r#type(aws_sdk_sfn::types::StateMachineType::Express);
        }
        
        let resp = req.send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.state_machine_arn().to_string())
    }

    async fn update_workflow(
        &self,
        workflow_arn: &str,
        definition: Value,
    ) -> CloudResult<()> {
        self.client.update_state_machine()
            .state_machine_arn(workflow_arn)
            .definition(definition.to_string())
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn delete_workflow(&self, workflow_arn: &str) -> CloudResult<()> {
        self.client.delete_state_machine()
            .state_machine_arn(workflow_arn)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn describe_workflow(&self, workflow_arn: &str) -> CloudResult<WorkflowDefinition> {
        let resp = self.client.describe_state_machine()
            .state_machine_arn(workflow_arn)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(WorkflowDefinition {
            name: resp.name().to_string(),
            arn: Some(resp.state_machine_arn().to_string()),
            description: None,
            workflow_type: match resp.r#type() {
                &aws_sdk_sfn::types::StateMachineType::Express => WorkflowType::Express,
                _ => WorkflowType::Standard,
            },
            definition: serde_json::from_str(resp.definition()).unwrap_or(Value::Null),
            role_arn: Some(resp.role_arn().to_string()),
            created_at: Some(chrono::DateTime::<chrono::Utc>::from_timestamp(resp.creation_date().secs(), 0).unwrap_or_default()),
            tags: Metadata::new(),
        })
    }

    async fn list_workflows(&self) -> CloudResult<Vec<WorkflowDefinition>> {
        let resp = self.client.list_state_machines()
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.state_machines().iter().map(|s| {
            WorkflowDefinition {
                name: s.name().to_string(),
                arn: Some(s.state_machine_arn().to_string()),
                description: None,
                workflow_type: match s.r#type() {
                    &aws_sdk_sfn::types::StateMachineType::Express => WorkflowType::Express,
                    _ => WorkflowType::Standard,
                },
                definition: Value::Null,
                role_arn: None,
                created_at: Some(chrono::DateTime::<chrono::Utc>::from_timestamp(s.creation_date().secs(), 0).unwrap_or_default()),
                tags: Metadata::new(),
            }
        }).collect())
    }

    async fn start_execution(
        &self,
        workflow_arn: &str,
        input: Value,
        options: StartExecutionOptions,
    ) -> CloudResult<Execution> {
        let mut req = self.client.start_execution()
            .state_machine_arn(workflow_arn)
            .input(input.to_string());
            
        if let Some(name) = options.name {
            req = req.name(name);
        }
        
        if let Some(trace) = options.trace_header {
            req = req.trace_header(trace);
        }
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        Ok(Execution {
            execution_id: resp.execution_arn().to_string(),
            workflow_arn: workflow_arn.to_string(),
            name: None,
            status: ExecutionStatus::Running,
            input: Some(input),
            output: None,
            error: None,
            start_time: chrono::DateTime::<chrono::Utc>::from_timestamp(resp.start_date().secs(), 0).unwrap_or_default(),
            stop_time: None,
        })
    }

    async fn stop_execution(
        &self,
        execution_id: &str,
        error: Option<&str>,
        cause: Option<&str>,
    ) -> CloudResult<()> {
        let mut req = self.client.stop_execution()
            .execution_arn(execution_id);
            
        if let Some(e) = error {
            req = req.error(e);
        }
        if let Some(c) = cause {
            req = req.cause(c);
        }
        
        req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn describe_execution(&self, execution_id: &str) -> CloudResult<Execution> {
        let resp = self.client.describe_execution()
            .execution_arn(execution_id)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(Execution {
            execution_id: resp.execution_arn().to_string(),
            workflow_arn: resp.state_machine_arn().to_string(),
            name: Some(resp.name().unwrap_or_default().to_string()),
            status: match resp.status() {
                &aws_sdk_sfn::types::ExecutionStatus::Running => ExecutionStatus::Running,
                &aws_sdk_sfn::types::ExecutionStatus::Succeeded => ExecutionStatus::Succeeded,
                &aws_sdk_sfn::types::ExecutionStatus::Failed => ExecutionStatus::Failed,
                &aws_sdk_sfn::types::ExecutionStatus::TimedOut => ExecutionStatus::TimedOut,
                &aws_sdk_sfn::types::ExecutionStatus::Aborted => ExecutionStatus::Aborted,
                _ => ExecutionStatus::Running,
            },
            input: resp.input().and_then(|i| serde_json::from_str(i).ok()),
            output: resp.output().and_then(|o| serde_json::from_str(o).ok()),
            error: None, // Could map from status if failed
            start_time: chrono::DateTime::<chrono::Utc>::from_timestamp(resp.start_date().secs(), 0).unwrap_or_default(),
            stop_time: resp.stop_date().map(|d| chrono::DateTime::<chrono::Utc>::from_timestamp(d.secs(), 0).unwrap_or_default()),
        })
    }

    async fn list_executions(
        &self,
        workflow_arn: &str,
        filter: ExecutionFilter,
    ) -> CloudResult<Vec<Execution>> {
        let mut req = self.client.list_executions()
            .state_machine_arn(workflow_arn);
            
        if let Some(status) = filter.status {
            req = req.set_status_filter(Some(match status {
                ExecutionStatus::Running => aws_sdk_sfn::types::ExecutionStatus::Running,
                ExecutionStatus::Succeeded => aws_sdk_sfn::types::ExecutionStatus::Succeeded,
                ExecutionStatus::Failed => aws_sdk_sfn::types::ExecutionStatus::Failed,
                ExecutionStatus::TimedOut => aws_sdk_sfn::types::ExecutionStatus::TimedOut,
                ExecutionStatus::Aborted => aws_sdk_sfn::types::ExecutionStatus::Aborted,
                ExecutionStatus::PendingRedrive => aws_sdk_sfn::types::ExecutionStatus::Running, // Closest match
            }));
        }
        
        if let Some(max) = filter.max_results {
            req = req.max_results(max as i32);
        }
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        Ok(resp.executions().iter().map(|e| {
            Execution {
                execution_id: e.execution_arn().to_string(),
                workflow_arn: workflow_arn.to_string(),
                name: Some(e.name().to_string()),
                status: match e.status() {
                    &aws_sdk_sfn::types::ExecutionStatus::Running => ExecutionStatus::Running,
                    &aws_sdk_sfn::types::ExecutionStatus::Succeeded => ExecutionStatus::Succeeded,
                    &aws_sdk_sfn::types::ExecutionStatus::Failed => ExecutionStatus::Failed,
                    &aws_sdk_sfn::types::ExecutionStatus::TimedOut => ExecutionStatus::TimedOut,
                    &aws_sdk_sfn::types::ExecutionStatus::Aborted => ExecutionStatus::Aborted,
                    _ => ExecutionStatus::Running,
                },
                input: None,
                output: None,
                error: None,
                start_time: chrono::DateTime::<chrono::Utc>::from_timestamp(e.start_date().secs(), 0).unwrap_or_default(),
                stop_time: e.stop_date().map(|d| chrono::DateTime::<chrono::Utc>::from_timestamp(d.secs(), 0).unwrap_or_default()),
            }
        }).collect())
    }

    async fn get_execution_history(
        &self,
        execution_id: &str,
    ) -> CloudResult<Vec<HistoryEvent>> {
        let resp = self.client.get_execution_history()
            .execution_arn(execution_id)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.events().iter().enumerate().map(|(i, e)| {
            HistoryEvent {
                id: (i as u64) + 1, // HistoryEvent uses u64
                timestamp: chrono::DateTime::<chrono::Utc>::from_timestamp(e.timestamp().secs(), 0).unwrap_or_default(),
                event_type: format!("{:?}", e.r#type()),
                details: Value::Null,
            }
        }).collect())
    }

    async fn send_task_success(
        &self,
        task_token: &str,
        output: Value,
    ) -> CloudResult<()> {
        self.client.send_task_success()
            .task_token(task_token)
            .output(output.to_string())
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn send_task_failure(
        &self,
        task_token: &str,
        error: &str,
        cause: &str,
    ) -> CloudResult<()> {
        self.client.send_task_failure()
            .task_token(task_token)
            .error(error)
            .cause(cause)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn send_task_heartbeat(&self, task_token: &str) -> CloudResult<()> {
        self.client.send_task_heartbeat()
            .task_token(task_token)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cloudkit_spi::core::ProviderType;
    use serde_json::json;

    async fn create_test_context() -> Arc<CloudContext> {
        Arc::new(
            CloudContext::builder(ProviderType::Aws)
                .build()
                .await
                .unwrap(),
        )
    }

    #[tokio::test]
    async fn test_step_functions_new() {
        let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let context = create_test_context().await;
        let _sf = AwsWorkflow::new(context, sdk_config);
    }
}

