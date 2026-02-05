use crate::{ClientInner, ZeroSdkError, common::request};
use std::sync::Arc;
use serde_json::json;

pub struct DbClient {
    inner: Arc<ClientInner>,
}

impl DbClient {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn create_table(&self, name: &str, pk: &str) -> Result<(), ZeroSdkError> {
        request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::POST,
            "/db/tables",
            Some(json!({ "name": name, "pk": pk })),
        ).await?;
        Ok(())
    }

    pub async fn put_item(&self, table: &str, pk_value: &str, item: serde_json::Value) -> Result<(), ZeroSdkError> {
        let mut body = item;
        // Ensure pk is present if not in payload
        if body.get("pk").is_none() {
            if let Some(obj) = body.as_object_mut() {
                obj.insert("pk".to_string(), json!(pk_value));
            }
        }
        
        request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::POST,
            &format!("/db/tables/{}/items", table),
            Some(body),
        ).await?;
        Ok(())
    }

    pub async fn list_tables(&self) -> Result<Vec<String>, ZeroSdkError> {
        let resp = request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::GET,
            "/db/tables",
            None,
        ).await?;
        
        let tables = resp["tables"].as_array().ok_or_else(|| {
            ZeroSdkError::Internal("Invalid response format: missing tables field".to_string())
        })?;
        
        Ok(tables.iter().map(|v| v.as_str().unwrap_or_default().to_string()).collect())
    }
}
