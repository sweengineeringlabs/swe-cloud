use async_trait::async_trait;
use cloudkit_spi::api::{Functions, InvocationType, InvokeOptions, InvokeResult};
use cloudkit_spi::common::{CloudError, CloudResult};
use cloudkit_spi::core::CloudContext;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

/// AWS Lambda functions implementation.
pub struct LambdaFunctions {
    _context: Arc<CloudContext>,
    client: aws_sdk_lambda::Client,
}

impl LambdaFunctions {
    /// Create a new Lambda functions client.
    pub fn new(context: Arc<CloudContext>, sdk_config: aws_config::SdkConfig) -> Self {
        let client = aws_sdk_lambda::Client::new(&sdk_config);
        Self { _context: context, client }
    }
}

#[async_trait]
impl Functions for LambdaFunctions {
    async fn invoke(
        &self,
        function_name: &str,
        payload: &[u8],
    ) -> CloudResult<InvokeResult> {
        self.invoke_with_options(function_name, payload, InvokeOptions::default()).await
    }

    async fn invoke_with_options(
        &self,
        function_name: &str,
        payload: &[u8],
        options: InvokeOptions,
    ) -> CloudResult<InvokeResult> {
        let mut req = self.client.invoke()
            .function_name(function_name)
            .payload(aws_sdk_lambda::primitives::Blob::new(payload))
            .set_invocation_type(match options.invocation_type {
                InvocationType::RequestResponse => Some(aws_sdk_lambda::types::InvocationType::RequestResponse),
                InvocationType::Event => Some(aws_sdk_lambda::types::InvocationType::Event),
                InvocationType::DryRun => Some(aws_sdk_lambda::types::InvocationType::DryRun),
            });
            
        if let Some(qualifier) = options.qualifier {
            req = req.qualifier(qualifier);
        }
        
        if let Some(context) = options.client_context {
            req = req.client_context(context);
        }
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        Ok(InvokeResult {
            status_code: resp.status_code() as u16,
            payload: resp.payload.as_ref().map(|b| b.as_ref().to_vec()),
            function_error: resp.function_error().map(|e| e.to_string()),
            executed_version: resp.executed_version().map(|v| v.to_string()),
            log_result: resp.log_result().map(|l| l.to_string()),
        })
    }

    async fn invoke_json<T, R>(&self, function_name: &str, payload: &T) -> CloudResult<R>
    where
        T: Serialize + Send + Sync,
        R: DeserializeOwned + Send {
        let payload_bytes = serde_json::to_vec(payload).map_err(|e| CloudError::Serialization(e.to_string()))?;
        let result = self.invoke(function_name, &payload_bytes).await?;
        
        if !result.is_success() {
            return Err(CloudError::ServiceError(result.function_error.unwrap_or_else(|| "Unknown Lambda error".to_string())));
        }
        
        result.parse_payload::<R>()?.ok_or_else(|| CloudError::Serialization("Empty response from Lambda".to_string()))
    }

    async fn invoke_async(
        &self,
        function_name: &str,
        payload: &[u8],
    ) -> CloudResult<()> {
        self.invoke_with_options(function_name, payload, InvokeOptions::new().async_invoke()).await?;
        Ok(())
    }

    async fn list_functions(&self) -> CloudResult<Vec<String>> {
        let resp = self.client.list_functions()
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.functions().iter().map(|f| f.function_name().unwrap_or_default().to_string()).collect())
    }

    async fn function_exists(&self, function_name: &str) -> CloudResult<bool> {
        match self.client.get_function().function_name(function_name).send().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

