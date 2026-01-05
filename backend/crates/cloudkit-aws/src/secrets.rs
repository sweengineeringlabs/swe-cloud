//! AWS Secrets Manager implementation.

use async_trait::async_trait;
use aws_sdk_secretsmanager::Client;
use cloudkit::api::{
    CreateSecretOptions, SecretMetadata, SecretVersion, SecretsManager,
};
use cloudkit::common::{CloudError, CloudResult, Metadata};
use cloudkit::core::CloudContext;
use std::sync::Arc;

/// AWS Secrets Manager implementation.
pub struct AwsSecretsManager {
    _context: Arc<CloudContext>,
    client: Client,
}

impl AwsSecretsManager {
    /// Create a new Secrets Manager client.
    pub fn new(context: Arc<CloudContext>, client: Client) -> Self {
        Self { _context: context, client }
    }

    fn map_err<E>(e: aws_sdk_secretsmanager::error::SdkError<E>) -> CloudError 
    where E: std::fmt::Debug {
        CloudError::Provider {
            provider: "aws".to_string(),
            code: "SecretsManagerError".to_string(),
            message: format!("{:?}", e),
        }
    }
}

#[async_trait]
impl SecretsManager for AwsSecretsManager {
    async fn create_secret(
        &self,
        name: &str,
        value: &str,
        _options: CreateSecretOptions,
    ) -> CloudResult<SecretMetadata> {
        self.client
            .create_secret()
            .name(name)
            .secret_string(value)
            .send()
            .await
            .map_err(Self::map_err)?;
            
        Ok(SecretMetadata::new(name))
    }

    async fn get_secret(&self, name: &str) -> CloudResult<String> {
        let resp = self.client
            .get_secret_value()
            .secret_id(name)
            .send()
            .await
            .map_err(Self::map_err)?;

        if let Some(s) = resp.secret_string {
            Ok(s)
        } else if let Some(b) = resp.secret_binary {
            String::from_utf8(b.into_inner()).map_err(|e| CloudError::Serialization(e.to_string()))
        } else {
            Err(CloudError::NotFound {
                resource_type: "SecretValue".to_string(),
                resource_id: name.to_string(),
            })
        }
    }

    async fn get_secret_version(&self, name: &str, version_id: &str) -> CloudResult<String> {
        let resp = self.client
            .get_secret_value()
            .secret_id(name)
            .version_id(version_id)
            .send()
            .await
            .map_err(Self::map_err)?;

        if let Some(s) = resp.secret_string {
            Ok(s)
        } else if let Some(b) = resp.secret_binary {
            String::from_utf8(b.into_inner()).map_err(|e| CloudError::Serialization(e.to_string()))
        } else {
            Err(CloudError::NotFound {
                resource_type: "SecretValue".to_string(),
                resource_id: format!("{}:{}", name, version_id),
            })
        }
    }

    async fn update_secret(&self, name: &str, value: &str) -> CloudResult<SecretMetadata> {
        self.client
            .put_secret_value()
            .secret_id(name)
            .secret_string(value)
            .send()
            .await
            .map_err(Self::map_err)?;
            
        Ok(SecretMetadata::new(name))
    }

    async fn delete_secret(&self, name: &str, force: bool) -> CloudResult<()> {
        let mut req = self.client.delete_secret().secret_id(name);
        
        if force {
            req = req.force_delete_without_recovery(true);
        }
        
        req.send().await.map_err(Self::map_err)?;
        Ok(())
    }

    async fn restore_secret(&self, name: &str) -> CloudResult<SecretMetadata> {
        self.client
            .restore_secret()
            .secret_id(name)
            .send()
            .await
            .map_err(Self::map_err)?;
            
        Ok(SecretMetadata::new(name))
    }

    async fn list_secrets(&self) -> CloudResult<Vec<SecretMetadata>> {
        let resp = self.client
            .list_secrets()
            .max_results(100) // Simplification
            .send()
            .await
            .map_err(Self::map_err)?;
            
        let secrets = resp.secret_list.unwrap_or_default();
        let metadata = secrets.into_iter().map(|s| {
            SecretMetadata::new(s.name.unwrap_or_default().as_str())
            // Populate meta...
        }).collect();
        
        Ok(metadata)
    }

    async fn describe_secret(&self, name: &str) -> CloudResult<SecretMetadata> {
        let resp = self.client
            .describe_secret()
            .secret_id(name)
            .send()
            .await
            .map_err(Self::map_err)?;
            
        Ok(SecretMetadata::new(resp.name.unwrap_or_default().as_str()))
    }

    async fn list_secret_versions(&self, name: &str) -> CloudResult<Vec<SecretVersion>> {
        let resp = self.client
            .list_secret_version_ids()
            .secret_id(name)
            .send()
            .await
            .map_err(Self::map_err)?;
            
        let versions = resp.versions.unwrap_or_default();
        let result = versions.into_iter().map(|v| {
            SecretVersion {
                version_id: v.version_id.unwrap_or_default(),
                stages: v.version_stages.unwrap_or_default(),
                created_at: None, // conversion needed
            }
        }).collect();
        
        Ok(result)
    }

    async fn rotate_secret(&self, name: &str) -> CloudResult<()> {
        self.client
            .rotate_secret()
            .secret_id(name)
            .send()
            .await
            .map_err(Self::map_err)?;
        Ok(())
    }

    async fn tag_secret(&self, name: &str, tags: Metadata) -> CloudResult<()> {
        let aws_tags: Vec<aws_sdk_secretsmanager::types::Tag> = tags.into_iter()
            .map(|(k, v)| aws_sdk_secretsmanager::types::Tag::builder().key(k).value(v).build())
            .collect();
            
        self.client
            .tag_resource()
            .secret_id(name)
            .set_tags(Some(aws_tags))
            .send()
            .await
            .map_err(Self::map_err)?;
        Ok(())
    }

    async fn untag_secret(&self, name: &str, tag_keys: &[&str]) -> CloudResult<()> {
        let keys: Vec<String> = tag_keys.iter().map(|s| s.to_string()).collect();
        
        self.client
            .untag_resource()
            .secret_id(name)
            .set_tag_keys(Some(keys))
            .send()
            .await
            .map_err(Self::map_err)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cloudkit::core::{ProviderType, CloudContext};
    use aws_config::BehaviorVersion;

    async fn create_client() -> Option<AwsSecretsManager> {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        // Check if creds available?
        // Just try to build client.
        let client = Client::new(&config);
        let context = Arc::new(CloudContext::builder(ProviderType::Aws).build().await.ok()?);
        Some(AwsSecretsManager::new(context, client))
    }

    #[tokio::test]
    #[ignore]
    async fn test_secrets_flow() {
        let manager = create_client().await.expect("Failed to create client");
        let name = "test-secret-" .to_string() + &uuid::Uuid::new_v4().to_string();
        
        // Create
        let _ = manager.create_secret(&name, "initial-value", CreateSecretOptions::default()).await.unwrap();
        
        // Get
        let val = manager.get_secret(&name).await.unwrap();
        assert_eq!(val, "initial-value");
        
        // Update
        manager.update_secret(&name, "new-value").await.unwrap();
        let val = manager.get_secret(&name).await.unwrap();
        assert_eq!(val, "new-value");
        
        // Delete
        manager.delete_secret(&name, true).await.unwrap();
    }
}
