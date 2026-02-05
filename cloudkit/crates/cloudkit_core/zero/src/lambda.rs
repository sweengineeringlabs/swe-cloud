use cloudkit_api::{Functions, InvokeOptions, InvokeResult, InvocationType};
use cloudkit_spi::{CloudResult, CloudError};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use zero_sdk::ZeroClient;

pub struct ZeroFunc {
    client: ZeroClient,
}

impl ZeroFunc {
    pub fn new(client: ZeroClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Functions for ZeroFunc {
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
        _options: InvokeOptions,
    ) -> CloudResult<InvokeResult> {
        let json_payload: serde_json::Value = serde_json::from_slice(payload)
            .unwrap_or(serde_json::json!({ "raw": String::from_utf8_lossy(payload) }));

        let resp = self.client.func().invoke(function_name, json_payload).await
            .map_err(|e| CloudError::Internal(e.to_string()))?;

        Ok(InvokeResult {
            status_code: 200,
            payload: Some(serde_json::to_vec(&resp).unwrap_or_default()),
            function_error: None,
            executed_version: None,
            log_result: None,
        })
    }

    async fn invoke_json<T, R>(&self, function_name: &str, payload: &T) -> CloudResult<R>
    where
        T: Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        let payload_bytes = serde_json::to_vec(payload)?;
        let result = self.invoke(function_name, &payload_bytes).await?;
        
        let response: R = serde_json::from_slice(&result.payload.unwrap_or_default())?;
        Ok(response)
    }

    async fn invoke_async(
        &self,
        function_name: &str,
        payload: &[u8],
    ) -> CloudResult<()> {
        let opts = InvokeOptions {
            invocation_type: InvocationType::Event,
            ..Default::default()
        };
        self.invoke_with_options(function_name, payload, opts).await?;
        Ok(())
    }

    async fn list_functions(&self) -> CloudResult<Vec<String>> {
        self.client.func().list_functions().await
            .map_err(|e| CloudError::Internal(e.to_string()))
    }

    async fn function_exists(&self, function_name: &str) -> CloudResult<bool> {
        let funcs = self.list_functions().await?;
        Ok(funcs.contains(&function_name.to_string()))
    }
}
