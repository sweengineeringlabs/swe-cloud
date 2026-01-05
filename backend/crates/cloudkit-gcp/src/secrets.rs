//! GCP Secret Manager implementation.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cloudkit::api::{
    CreateSecretOptions, SecretMetadata, SecretVersion, SecretsManager,
};
use cloudkit::common::{CloudError, CloudResult, Metadata};
use cloudkit::core::CloudContext;
use google_cloud_auth::token_source::TokenSource;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use base64::prelude::*;

/// GCP Secret Manager implementation.
pub struct GcpSecretManager {
    _context: Arc<CloudContext>,
    auth: Arc<Box<dyn TokenSource>>,
    project_id: String,
    client: Client,
}

impl GcpSecretManager {
    /// Create a new Secret Manager client.
    pub fn new(
        context: Arc<CloudContext>, 
        auth: Arc<Box<dyn TokenSource>>, 
        project_id: String
    ) -> Self {
        Self {
            _context: context,
            auth,
            project_id,
            client: Client::new(),
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

    fn base_url(&self) -> String {
        format!("https://secretmanager.googleapis.com/v1/projects/{}", self.project_id)
    }
}

#[derive(Deserialize)]
struct AccessResponse {
    payload: Payload,
}
#[derive(Deserialize)]
struct Payload {
    data: String,
}
#[derive(Deserialize)]
struct SecretResponse {
    name: String,
    labels: Option<Metadata>,
    etag: Option<String>,
}
#[derive(Deserialize)]
struct ListSecretsResponse {
    secrets: Option<Vec<SecretResponse>>,
}
#[derive(Deserialize)]
struct SecretVersionResponse {
    name: String,
    state: String,
    #[serde(rename = "createTime")]
    create_time: Option<String>,
}
#[derive(Deserialize)]
struct ListSecretVersionsResponse {
    versions: Option<Vec<SecretVersionResponse>>,
}

#[async_trait]
impl SecretsManager for GcpSecretManager {
    async fn create_secret(
        &self,
        name: &str,
        value: &str,
        _options: CreateSecretOptions,
    ) -> CloudResult<SecretMetadata> {
        let token = self.token().await?;
        
        // 1. Create Secret (Container)
        let url = format!("{}/secrets?secretId={}", self.base_url(), name);
        let body = json!({
            "replication": {
                "automatic": {}
            }
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

        // 2. Add Version
        let url = format!("{}/secrets/{}:addVersion", self.base_url(), name);
        let body = json!({
            "payload": {
                "data": BASE64_STANDARD.encode(value)
            }
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
        
        Ok(SecretMetadata::new(name))
    }

    async fn get_secret(&self, name: &str) -> CloudResult<String> {
        let token = self.token().await?;
        let url = format!("{}/secrets/{}/versions/latest:access", self.base_url(), name);
        
        let resp = self.client.get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if resp.status() == StatusCode::NOT_FOUND {
            return Err(CloudError::NotFound { resource_type: "Secret".into(), resource_id: name.into() });
        }
        if !resp.status().is_success() {
             return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: resp.status().as_u16().to_string(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        let body: AccessResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        let decoded = BASE64_STANDARD.decode(&body.payload.data).map_err(|e| CloudError::Serialization(e.to_string()))?;
        String::from_utf8(decoded).map_err(|e| CloudError::Serialization(e.to_string()))
    }

    async fn get_secret_version(&self, name: &str, version_id: &str) -> CloudResult<String> {
        let token = self.token().await?;
        let url = format!("{}/secrets/{}/versions/{}:access", self.base_url(), name, version_id);
        
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

        let body: AccessResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        let decoded = BASE64_STANDARD.decode(&body.payload.data).map_err(|e| CloudError::Serialization(e.to_string()))?;
        String::from_utf8(decoded).map_err(|e| CloudError::Serialization(e.to_string()))
    }

    async fn update_secret(&self, name: &str, value: &str) -> CloudResult<SecretMetadata> {
        let token = self.token().await?;
        let url = format!("{}/secrets/{}:addVersion", self.base_url(), name);
        let body = json!({
            "payload": {
                "data": BASE64_STANDARD.encode(value)
            }
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
        
        Ok(SecretMetadata::new(name))
    }

    async fn delete_secret(&self, name: &str, _force: bool) -> CloudResult<()> {
        let token = self.token().await?;
        let url = format!("{}/secrets/{}", self.base_url(), name);
        
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

    async fn restore_secret(&self, _name: &str) -> CloudResult<SecretMetadata> {
        Err(CloudError::Provider {
            provider: "gcp".to_string(),
            code: "NotImplemented".to_string(),
            message: "Restore secret not implemented".to_string(),
        })
    }

    async fn list_secrets(&self) -> CloudResult<Vec<SecretMetadata>> {
        let token = self.token().await?;
        let url = format!("{}/secrets?pageSize=100", self.base_url());
        
        let resp = self.client.get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
            return Ok(vec![]); // Simplified handling, or error
        }

        let body: ListSecretsResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        let secrets = body.secrets.unwrap_or_default();
        let meta = secrets.into_iter().map(|s| {
            let short_name = s.name.split('/').last().unwrap_or("unknown").to_string();
            let mut m = SecretMetadata::new(short_name);
            m.tags = s.labels.unwrap_or_default();
            m
        }).collect();
        
        Ok(meta)
    }

    async fn describe_secret(&self, name: &str) -> CloudResult<SecretMetadata> {
        let token = self.token().await?;
        let url = format!("{}/secrets/{}", self.base_url(), name);
        
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
        
        let body: SecretResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        let short_name = body.name.split('/').last().unwrap_or("unknown").to_string();
        let mut m = SecretMetadata::new(short_name);
        m.tags = body.labels.unwrap_or_default();
        Ok(m)
    }

    async fn list_secret_versions(&self, name: &str) -> CloudResult<Vec<SecretVersion>> {
        let token = self.token().await?;
        let url = format!("{}/secrets/{}/versions", self.base_url(), name);
        
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
        
        let body: ListSecretVersionsResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        let versions = body.versions.unwrap_or_default().into_iter().map(|v| {
            let version_id = v.name.split('/').last().unwrap_or("unknown").to_string();
             SecretVersion {
                version_id,
                stages: vec![v.state], // Map state to stage-like string
                created_at: v.create_time.and_then(|t| DateTime::parse_from_rfc3339(&t).ok().map(|dt| dt.with_timezone(&Utc))),
            }
        }).collect();
        Ok(versions)
    }

    async fn rotate_secret(&self, _name: &str) -> CloudResult<()> {
        // GCP requires configured rotation, no direct API to force rotate without setup
        Ok(())
    }

    async fn tag_secret(&self, name: &str, tags: Metadata) -> CloudResult<()> {
        let token = self.token().await?;
        let url = format!("{}/secrets/{}", self.base_url(), name);
        
        // 1. Get current secret
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
        let secret: SecretResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        
        // 2. Update labels
        let mut labels = secret.labels.unwrap_or_default();
        labels.extend(tags);
        
        // 3. Patch
        let url = format!("{}/secrets/{}?updateMask=labels", self.base_url(), name);
        let body = json!({
            "name": secret.name, 
            "labels": labels,
            "etag": secret.etag
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

    async fn untag_secret(&self, name: &str, tag_keys: &[&str]) -> CloudResult<()> {
        let token = self.token().await?;
        let url = format!("{}/secrets/{}", self.base_url(), name);
        
        let resp = self.client.get(&url).bearer_auth(&token).send().await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;
            
        if !resp.status().is_success() {
             return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: resp.status().as_u16().to_string(),
                message: resp.text().await.unwrap_or_default(),
            });
        }
        let secret: SecretResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        
        let mut labels = secret.labels.unwrap_or_default();
        for k in tag_keys {
            labels.remove(*k);
        }
        
        let url = format!("{}/secrets/{}?updateMask=labels", self.base_url(), name);
        let body = json!({
            "name": secret.name, 
            "labels": labels,
            "etag": secret.etag
        });
        
        let resp = self.client.patch(&url).bearer_auth(&token).json(&body).send().await
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_secrets_flow() {
        // Mock testing or real if env configured.
        // Requires GcpBuilder logic to inject auth.
        // Integration test would be external.
    }
}
