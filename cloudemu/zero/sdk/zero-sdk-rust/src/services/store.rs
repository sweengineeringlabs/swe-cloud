use crate::{ClientInner, ZeroSdkError, common::request};
use std::sync::Arc;
use serde_json::json;

pub struct StoreClient {
    inner: Arc<ClientInner>,
}

impl StoreClient {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn create_bucket(&self, name: &str) -> Result<(), ZeroSdkError> {
        request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::POST,
            "/store/buckets",
            Some(json!({ "name": name })),
        ).await?;
        Ok(())
    }

    pub async fn list_buckets(&self) -> Result<Vec<String>, ZeroSdkError> {
        let resp = request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::GET,
            "/store/buckets",
            None,
        ).await?;
        
        let buckets = resp["buckets"].as_array().ok_or_else(|| {
            ZeroSdkError::Internal("Invalid response format: missing buckets field".to_string())
        })?;
        
        Ok(buckets.iter().map(|v| v.as_str().unwrap_or_default().to_string()).collect())
    }
}
