pub mod services;
pub mod error;

pub use error::ZeroSdkError;
use std::sync::Arc;

#[derive(Clone)]
pub struct ZeroClient {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    base_url: String,
    http: reqwest::Client,
}

impl ZeroClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            inner: Arc::new(ClientInner {
                base_url: base_url.into().trim_end_matches('/').to_string(),
                http: reqwest::Client::new(),
            }),
        }
    }

    pub fn from_env() -> Self {
        let url = std::env::var("ZERO_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
        Self::new(url)
    }

    pub fn store(&self) -> services::store::StoreClient {
        services::store::StoreClient::new(self.inner.clone())
    }

    pub fn db(&self) -> services::db::DbClient {
        services::db::DbClient::new(self.inner.clone())
    }

    pub fn func(&self) -> services::func::FuncClient {
        services::func::FuncClient::new(self.inner.clone())
    }

    pub fn queue(&self) -> services::queue::QueueClient {
        services::queue::QueueClient::new(self.inner.clone())
    }

    pub fn iam(&self) -> services::iam::IamClient {
        services::iam::IamClient::new(self.inner.clone())
    }

    pub fn lb(&self) -> services::lb::LbClient {
        services::lb::LbClient::new(self.inner.clone())
    }
}

pub(crate) mod common {
    use super::*;
    use serde::de::DeserializeOwned;

    pub async fn request<T: DeserializeOwned>(
        inner: &Arc<ClientInner>,
        method: reqwest::Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<T, ZeroSdkError> {
        let url = format!("{}/v1/{}", inner.base_url, path.trim_start_matches('/'));
        let mut req = inner.http.request(method, &url);
        
        if let Some(b) = body {
            req = req.json(&b);
        }

        let resp = req.send().await.map_err(ZeroSdkError::Http)?;
        
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(ZeroSdkError::Api { status, body });
        }

        resp.json::<T>().await.map_err(ZeroSdkError::Http)
    }
}
