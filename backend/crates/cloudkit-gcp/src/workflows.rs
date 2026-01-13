//! Google Cloud Workflows implementation.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cloudkit_spi::api::{
    Execution, ExecutionError, ExecutionFilter, ExecutionStatus, HistoryEvent,
    StartExecutionOptions, WorkflowDefinition, WorkflowService, WorkflowType,
};
use cloudkit_spi::common::{CloudError, CloudResult, Metadata};
use cloudkit_spi::core::CloudContext;
use google_cloud_auth::token_source::TokenSource;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

/// Google Cloud Workflows implementation.
pub struct GcpWorkflows {
    _context: Arc<CloudContext>,
    auth: Arc<Box<dyn TokenSource>>,
    project_id: String,
    client: Client,
    region: String,
}

impl GcpWorkflows {
    /// Create a new Workflows client.
    pub fn new(
        context: Arc<CloudContext>,
        auth: Arc<Box<dyn TokenSource>>,
        project_id: String,
    ) -> Self {
        Self {
            _context: context,
            auth,
            project_id,
            client: Client::new(),
            region: "us-central1".to_string(), // Default region
        }
    }

    async fn token(&self) -> CloudResult<String> {
        let token = self.auth.token().await.map_err(|e| CloudError::Provider {
            provider: "gcp".to_string(),
            code: "AuthError".to_string(),
            message: e.to_string(),
        })?;
        Ok(token.access_token)
    }

    fn workflows_base_url(&self) -> String {
        format!(
            "https://workflows.googleapis.com/v1/projects/{}/locations/{}",
            self.project_id, self.region
        )
    }

    fn executions_base_url(&self) -> String {
        format!(
            "https://workflowexecutions.googleapis.com/v1/projects/{}/locations/{}",
            self.project_id, self.region
        )
    }

    fn parse_workflow(&self, w: GcpWorkflow) -> WorkflowDefinition {
        let name = w.name.split('/').last().unwrap_or("unknown").to_string();
        WorkflowDefinition {
            name: name.clone(),
            arn: Some(w.name),
            description: Some(w.description.unwrap_or_default()),
            workflow_type: WorkflowType::Standard,
            definition: serde_json::from_str(&w.source_contents.unwrap_or_default())
                .unwrap_or(json!({})),
            role_arn: Some(w.service_account.unwrap_or_default()),
            created_at: w.create_time.and_then(|t| DateTime::parse_from_rfc3339(&t).ok().map(|dt| dt.with_timezone(&Utc))),
            tags: w.labels.unwrap_or_default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct GcpWorkflow {
    name: String,
    description: Option<String>,
    state: Option<String>,
    #[serde(rename = "sourceContents")]
    source_contents: Option<String>,
    #[serde(rename = "serviceAccount")]
    service_account: Option<String>,
    #[serde(rename = "createTime")]
    create_time: Option<String>,
    labels: Option<Metadata>,
}

#[derive(Serialize, Deserialize)]
struct ListWorkflowsResponse {
    workflows: Option<Vec<GcpWorkflow>>,
}

#[derive(Serialize, Deserialize)]
struct GcpExecution {
    name: String, // Resource name
    state: String, 
    argument: Option<String>, // JSON string
    result: Option<String>,   // JSON string
    error: Option<GcpExecutionError>,
    #[serde(rename = "startTime")]
    start_time: Option<String>,
    #[serde(rename = "endTime")]
    end_time: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct GcpExecutionError {
    payload: Option<String>,
    context: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ListExecutionsResponse {
    executions: Option<Vec<GcpExecution>>,
}

#[async_trait]
impl WorkflowService for GcpWorkflows {
    async fn create_workflow(&self, definition: WorkflowDefinition) -> CloudResult<String> {
        let token = self.token().await?;
        let url = format!("{}/workflows?workflowId={}", self.workflows_base_url(), definition.name);

        let source = definition.definition.to_string();
        let body = json!({
            "description": definition.description,
            "sourceContents": source,
            "serviceAccount": definition.role_arn,
            "labels": definition.tags
        });

        let resp = self.client.post(&url)
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
            return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: resp.status().as_u16().to_string(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        let w: GcpWorkflow = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        Ok(w.name)
    }

    async fn update_workflow(
        &self,
        workflow_arn: &str,
        definition: Value,
    ) -> CloudResult<()> {
        let token = self.token().await?;
        // workflow_arn might be full resource path or just name. Assume name if simple.
        let resource_name = if workflow_arn.contains('/') {
            workflow_arn.to_string()
        } else {
             format!("projects/{}/locations/{}/workflows/{}", self.project_id, self.region, workflow_arn)
        };
        
        let url = format!("https://workflows.googleapis.com/v1/{}?updateMask=sourceContents", resource_name);

        let body = json!({
            "sourceContents": definition.to_string(),
        });

        let resp = self.client.patch(&url)
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
            return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: resp.status().as_u16().to_string(),
                message: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(())
    }

    async fn delete_workflow(&self, workflow_arn: &str) -> CloudResult<()> {
        let token = self.token().await?;
         let resource_name = if workflow_arn.contains('/') {
            workflow_arn.to_string()
        } else {
             format!("projects/{}/locations/{}/workflows/{}", self.project_id, self.region, workflow_arn)
        };
        let url = format!("https://workflows.googleapis.com/v1/{}", resource_name);

        let resp = self.client.delete(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
             return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: resp.status().as_u16().to_string(),
                message: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(())
    }

    async fn describe_workflow(&self, workflow_arn: &str) -> CloudResult<WorkflowDefinition> {
        let token = self.token().await?;
         let resource_name = if workflow_arn.contains('/') {
            workflow_arn.to_string()
        } else {
             format!("projects/{}/locations/{}/workflows/{}", self.project_id, self.region, workflow_arn)
        };
        let url = format!("https://workflows.googleapis.com/v1/{}", resource_name);

        let resp = self.client.get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
             return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: resp.status().as_u16().to_string(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        let w: GcpWorkflow = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        Ok(self.parse_workflow(w))
    }

    async fn list_workflows(&self) -> CloudResult<Vec<WorkflowDefinition>> {
        let token = self.token().await?;
        let url = format!("{}/workflows", self.workflows_base_url());

        let resp = self.client.get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
             return Ok(vec![]);
        }

        let body: ListWorkflowsResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        let workflows = body.workflows.unwrap_or_default().into_iter()
            .map(|w| self.parse_workflow(w))
            .collect();
        Ok(workflows)
    }

    async fn start_execution(
        &self,
        workflow_arn: &str,
        input: Value,
        _options: StartExecutionOptions,
    ) -> CloudResult<Execution> {
        let token = self.token().await?;
         let resource_name = if workflow_arn.contains('/') {
            workflow_arn.to_string()
        } else {
             format!("projects/{}/locations/{}/workflows/{}", self.project_id, self.region, workflow_arn)
        };
        
        // executions are under the workflow
        let url = format!("https://workflowexecutions.googleapis.com/v1/{}/executions", resource_name);
        
        let body = json!({
            "argument": input.to_string()
        });

        let resp = self.client.post(&url)
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
             return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: resp.status().as_u16().to_string(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        let exec: GcpExecution = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
         Ok(Execution {
            execution_id: exec.name.clone(),
            workflow_arn: resource_name,
            name: Some(exec.name.split('/').last().unwrap_or_default().to_string()),
            status: match exec.state.as_str() {
                "ACTIVE" => ExecutionStatus::Running,
                "SUCCEEDED" => ExecutionStatus::Succeeded,
                "FAILED" => ExecutionStatus::Failed,
                "CANCELLED" => ExecutionStatus::Aborted,
                _ => ExecutionStatus::Running,
            },
            input: Some(input),
            output: None, // Not available immediately on start usually
            error: None,
            start_time: exec.start_time.and_then(|t| DateTime::parse_from_rfc3339(&t).ok().map(|dt| dt.with_timezone(&Utc))).unwrap_or_else(|| Utc::now()),
            stop_time: None,
        })
    }

    async fn stop_execution(
        &self,
        execution_id: &str,
        _error: Option<&str>,
        _cause: Option<&str>,
    ) -> CloudResult<()> {
        let token = self.token().await?;
         // execution_id from start_execution is full resource name in GCP
        let url = format!("https://workflowexecutions.googleapis.com/v1/{}:cancel", execution_id);

        let resp = self.client.post(&url)
            .bearer_auth(&token)
            .json(&json!({}))
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
             return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: resp.status().as_u16().to_string(),
                message: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(())
    }

    async fn describe_execution(&self, execution_id: &str) -> CloudResult<Execution> {
        let token = self.token().await?;
        let url = format!("https://workflowexecutions.googleapis.com/v1/{}", execution_id);

        let resp = self.client.get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
             return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: resp.status().as_u16().to_string(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        let exec: GcpExecution = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        
        let output = if let Some(r) = exec.result {
            serde_json::from_str(&r).ok()
        } else {
            None
        };
        
        let input = if let Some(a) = exec.argument {
            serde_json::from_str(&a).ok()
        } else {
            None
        };

        let workflow_arn = exec.name.split("/executions/").next().unwrap_or("unknown").to_string();

        Ok(Execution {
            execution_id: exec.name.clone(),
            workflow_arn,
            name: Some(exec.name.split('/').last().unwrap_or_default().to_string()),
            status: match exec.state.as_str() {
                "ACTIVE" => ExecutionStatus::Running,
                "SUCCEEDED" => ExecutionStatus::Succeeded,
                "FAILED" => ExecutionStatus::Failed,
                "CANCELLED" => ExecutionStatus::Aborted,
                _ => ExecutionStatus::Running,
            },
            input,
            output, 
            error: exec.error.map(|e| ExecutionError {
                error: "ExecutionFailed".to_string(),
                cause: e.payload.unwrap_or_else(|| e.context.unwrap_or_default()),
            }),
            start_time: exec.start_time.and_then(|t| DateTime::parse_from_rfc3339(&t).ok().map(|dt| dt.with_timezone(&Utc))).unwrap_or_else(|| Utc::now()),
            stop_time: exec.end_time.and_then(|t| DateTime::parse_from_rfc3339(&t).ok().map(|dt| dt.with_timezone(&Utc))),
        })
    }

    async fn list_executions(
        &self,
        workflow_arn: &str,
        _filter: ExecutionFilter,
    ) -> CloudResult<Vec<Execution>> {
         let token = self.token().await?;
         let resource_name = if workflow_arn.contains('/') {
            workflow_arn.to_string()
        } else {
             format!("projects/{}/locations/{}/workflows/{}", self.project_id, self.region, workflow_arn)
        };
        
        let url = format!("https://workflowexecutions.googleapis.com/v1/{}/executions", resource_name);

        let resp = self.client.get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
             return Ok(vec![]);
        }

        let body: ListExecutionsResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        let executions = body.executions.unwrap_or_default().into_iter().map(|exec| {
             Execution {
                execution_id: exec.name.clone(),
               workflow_arn: resource_name.clone(),
                name: Some(exec.name.split('/').last().unwrap_or_default().to_string()),
                status: match exec.state.as_str() {
                    "ACTIVE" => ExecutionStatus::Running,
                    "SUCCEEDED" => ExecutionStatus::Succeeded,
                    "FAILED" => ExecutionStatus::Failed,
                    "CANCELLED" => ExecutionStatus::Aborted,
                    _ => ExecutionStatus::Running,
                },
                input: None, // List doesn't usually return full input/output to save bandwidth in some APIs, assuming similar here or parsing is expensive
                output: None,
                error: None,
                start_time: exec.start_time.and_then(|t| DateTime::parse_from_rfc3339(&t).ok().map(|dt| dt.with_timezone(&Utc))).unwrap_or_else(|| Utc::now()),
                stop_time: exec.end_time.and_then(|t| DateTime::parse_from_rfc3339(&t).ok().map(|dt| dt.with_timezone(&Utc))),
            }
        }).collect();
        Ok(executions)
    }

    async fn get_execution_history(
        &self,
        _execution_id: &str,
    ) -> CloudResult<Vec<HistoryEvent>> {
        // GCP doesn't expose granular history steps in the same way as Step Functions via HTTP easily
        Ok(vec![])
    }

    async fn send_task_success(
        &self,
        task_token: &str,
        output: Value,
    ) -> CloudResult<()> {
        // GCP Workflows Callbacks use a unique URL (passed as the task token).
        // We act as the external system calling back.
        // task_token is expected to be the full callback URL.
        
        // Simple validation that it looks like a URL
        if !task_token.starts_with("https://") {
             return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: "InvalidToken".to_string(),
                message: "Task token must be a valid https callback URL for GCP Workflows".to_string(),
            });
        }
        
        let client = &self.client; 
        // Note: Callbacks might not require auth if "open", but usually require IAM.
        // We'll attach the same token we use for management.
        let token = self.token().await?;

        let resp = client.post(task_token)
            .bearer_auth(&token)
            .json(&output)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
             return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: resp.status().as_u16().to_string(),
                message: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(())
    }

    async fn send_task_failure(
        &self,
        _task_token: &str,
        _error: &str,
        _cause: &str,
    ) -> CloudResult<()> {
        // GCP Workflows callbacks are generic HTTP endpoints. 
        // There isn't a standard 'failure' endpoint. The workflow logic determines success/failure based on payload.
        Err(CloudError::Provider {
            provider: "gcp".to_string(),
            code: "NotSupported".to_string(),
            message: "GCP Workflows does not support explicit failure signals via callbacks. Send a success payload with error details instead.".to_string(),
        })
    }

    async fn send_task_heartbeat(&self, _task_token: &str) -> CloudResult<()> {
         Err(CloudError::Provider {
            provider: "gcp".to_string(),
            code: "NotSupported".to_string(),
            message: "GCP Workflows does not support heartbeats for callbacks.".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cloudkit_spi::core::ProviderType;

    #[tokio::test]
    #[ignore]
    async fn test_workflows_flow() {
        let project_id = std::env::var("GCP_PROJECT_ID")
            .expect("GCP_PROJECT_ID must be set for integration tests");

        let config = google_cloud_auth::project::Config {
            scopes: Some(&["https://www.googleapis.com/auth/cloud-platform"]),
            ..Default::default()
        };
        let auth = google_cloud_auth::project::create_token_source(config)
            .await
            .expect("Failed to create token source");

        let context = Arc::new(
            CloudContext::builder(ProviderType::Gcp)
                .build()
                .await
                .expect("Failed to create context"),
        );

        let workflows = GcpWorkflows::new(context, Arc::new(auth), project_id);

        let def = WorkflowDefinition::new(
            "test-workflow",
            json!([
                { "init": { "assign": [ { "message": "Hello" } ] } },
                { "return": { "return": "${message}" } }
            ]),
        );
        
        // This Create uses YAML/JSON based syntax. The generic definition is JSON. GCP Workflows uses YAML usually.
        // We pass it as string sourceContents.
    }
}

