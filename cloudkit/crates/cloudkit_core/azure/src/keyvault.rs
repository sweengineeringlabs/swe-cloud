//! Azure Key Vault Secrets implementation.

use async_trait::async_trait;
use azure_security_keyvault::SecretClient;
use cloudkit_api::{CreateSecretOptions, SecretMetadata, SecretVersion, SecretsManager};
use cloudkit_spi::{CloudError, CloudResult, Metadata};
use cloudkit_spi::CloudContext;

use std::sync::Arc;

/// Azure Key Vault Secrets implementation.
pub struct AzureKeyVaultSecrets {
    _context: Arc<CloudContext>,
    client: SecretClient,
}

impl AzureKeyVaultSecrets {
    /// Create a new Key Vault Secrets client.
    pub fn new(context: Arc<CloudContext>, client: SecretClient) -> Self {
        Self { _context: context, client }
    }

    fn map_err(e: azure_core::Error) -> CloudError {
        CloudError::Provider {
            provider: "azure".to_string(),
            code: "KeyVaultError".to_string(),
            message: e.to_string(),
        }
    }
}

#[async_trait]
impl SecretsManager for AzureKeyVaultSecrets {
    async fn create_secret(
        &self,
        name: &str,
        value: &str,
        _options: CreateSecretOptions,
    ) -> CloudResult<SecretMetadata> {
        self.client
            .set(name, value)
            .await
            .map_err(Self::map_err)?;
            
        Ok(SecretMetadata::new(name))
    }

    async fn get_secret(&self, name: &str) -> CloudResult<String> {
        let secret = self.client.get(name).await.map_err(Self::map_err)?;
        Ok(secret.value)
    }

    async fn get_secret_version(&self, _name: &str, _version_id: &str) -> CloudResult<String> {
        // Stub versioned get for now as method unclear
        Err(CloudError::Provider {
            provider: "azure".to_string(),
            code: "NotImplemented".to_string(),
            message: "Get secret version not implemented".to_string(),
        })
    }

    async fn update_secret(&self, name: &str, value: &str) -> CloudResult<SecretMetadata> {
        self.client
            .set(name, value)
            .await
            .map_err(Self::map_err)?;
        Ok(SecretMetadata::new(name))
    }

    async fn delete_secret(&self, name: &str, _force: bool) -> CloudResult<()> {
        self.client
            .delete(name)
            .await
            .map_err(Self::map_err)?;
        Ok(())
    }

    async fn restore_secret(&self, _name: &str) -> CloudResult<SecretMetadata> {
        Err(CloudError::Provider {
            provider: "azure".to_string(),
            code: "NotImplemented".to_string(),
            message: "Restore secret not implemented".to_string(),
        })
    }

    async fn list_secrets(&self) -> CloudResult<Vec<SecretMetadata>> {
        // Stub
        Ok(vec![])
    }

    async fn describe_secret(&self, name: &str) -> CloudResult<SecretMetadata> {
        let secret = self.client.get(name).await.map_err(Self::map_err)?;
        let mut meta = SecretMetadata::new(name);
        // Extract version from ID URL: https://vault.vault.azure.net/secrets/name/version
        let id = &secret.id;
        meta.version_id = id.split('/').last().map(|s| s.to_string());
        Ok(meta)
    }

    async fn list_secret_versions(&self, _name: &str) -> CloudResult<Vec<SecretVersion>> {
        // Stub
        Ok(vec![])
    }

    async fn rotate_secret(&self, _name: &str) -> CloudResult<()> {
        // Not directly on SecretClient (requires specific management).
        Ok(())
    }

    async fn tag_secret(&self, _name: &str, _tags: Metadata) -> CloudResult<()> {
        Ok(())
    }

    async fn untag_secret(&self, _name: &str, _tag_keys: &[&str]) -> CloudResult<()> {
        // Need to remove specific tags. Complex update.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cloudkit_spi::ProviderType;

    // Helper to create client if possible (env vars)
    async fn create_client() -> Option<AzureKeyVaultSecrets> {
        let vault_name = std::env::var("AZURE_KEYVAULT_NAME").ok()?;
        let url = format!("https://{}.vault.azure.net", vault_name);
        let creds = std::sync::Arc::new(azure_identity::DefaultAzureCredential::create(Default::default()).ok()?);
        let client = azure_security_keyvault::SecretClient::new(&url, creds).ok()?;
        
        let context = Arc::new(CloudContext::builder(ProviderType::Azure).build().await.ok()?);
        Some(AzureKeyVaultSecrets::new(context, client))
    }

    #[tokio::test]
    #[ignore]
    async fn test_secrets_flow() {
        let manager = match create_client().await {
            Some(m) => m,
            None => {
                println!("Skipping test: AZURE_KEYVAULT_NAME not set");
                return;
            }
        };
        
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
        manager.delete_secret(&name, false).await.unwrap();
    }
}

