use crate::{ClientInner, ZeroSdkError, common::request};
use std::sync::Arc;
use serde_json::json;

pub struct FuncClient {
    inner: Arc<ClientInner>,
}

impl FuncClient {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn create_function(&self, name: &str, handler: &str, code: &str) -> Result<(), ZeroSdkError> {
        request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::POST,
            "/func/functions",
            Some(json!({ "name": name, "handler": handler, "code": code })),
        ).await?;
        Ok(())
    }

    pub async fn invoke(&self, name: &str, payload: serde_json::Value) -> Result<serde_json::Value, ZeroSdkError> {
        request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::POST,
            &format!("/func/functions/{}/invocations", name),
            Some(payload),
        ).await
    }

    pub async fn list_functions(&self) -> Result<Vec<String>, ZeroSdkError> {
        let resp = request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::GET,
            "/func/functions",
            None,
        ).await?;
        
        let functions = resp["functions"].as_array().ok_or_else(|| {
            ZeroSdkError::Internal("Invalid response format: missing functions field".to_string())
        })?;
        
        Ok(functions.iter().map(|v| v.as_str().unwrap_or_default().to_string()).collect())
    }
}
