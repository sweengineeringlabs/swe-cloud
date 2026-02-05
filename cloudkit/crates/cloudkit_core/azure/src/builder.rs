//! Azure client builder.

use cloudkit_spi::{CloudConfig, CloudResult, Region, CloudContext, ProviderType, CloudError};
use std::sync::Arc;

/// Azure client builder.
pub struct AzureBuilder {
    region: Option<Region>,
    config: Option<CloudConfig>,
    storage_account: Option<String>,
    keyvault_name: Option<String>,
}

impl AzureBuilder {
    /// Create a new Azure builder.
    pub fn new() -> Self {
        Self {
            region: None,
            config: None,
            storage_account: None,
            keyvault_name: None,
        }
    }

    /// Set the Azure region.
    pub fn region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    /// Set the storage account name.
    pub fn storage_account(mut self, account: impl Into<String>) -> Self {
        self.storage_account = Some(account.into());
        self
    }

    /// Set the Key Vault name.
    pub fn keyvault_name(mut self, name: impl Into<String>) -> Self {
        self.keyvault_name = Some(name.into());
        self
    }

    /// Set the configuration.
    pub fn config(mut self, config: CloudConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Set a custom endpoint.
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        let config = self.config.get_or_insert_with(CloudConfig::default);
        config.endpoint = Some(endpoint.into());
        self
    }

    /// Build the Azure client.
    pub async fn build(self) -> CloudResult<AzureClient> {
        let mut config = self.config.unwrap_or_default();
        
        if let Some(region) = self.region {
            config.region = region;
        }

        let context = CloudContext::builder(ProviderType::Azure)
            .config(config)
            .build()
            .await?;

        #[cfg(feature = "keyvault")]
        let secret_client = if let Some(ref kv_name) = self.keyvault_name {
            let url = format!("https://{}.vault.azure.net", kv_name);
            let creds = std::sync::Arc::new(
                azure_identity::DefaultAzureCredential::create(Default::default())
                    .map_err(|e| CloudError::Config(e.to_string()))?
            );
            Some(azure_security_keyvault::SecretClient::new(&url, creds).map_err(|e| CloudError::Config(e.to_string()))?)
        } else {
            None
        };

        Ok(AzureClient {
            context: Arc::new(context),
            storage_account: self.storage_account,
            #[cfg(feature = "keyvault")]
            secret_client,
        })
    }
}

impl Default for AzureBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Azure client.
pub struct AzureClient {
    context: Arc<CloudContext>,
    storage_account: Option<String>,
    #[cfg(feature = "keyvault")]
    secret_client: Option<azure_security_keyvault::SecretClient>,
}

impl AzureClient {
    /// Get the underlying context.
    pub fn context(&self) -> &CloudContext {
        &self.context
    }

    /// Get the storage account name.
    pub fn storage_account(&self) -> Option<&str> {
        self.storage_account.as_deref()
    }

    /// Get the Key Vault Secrets client.
    #[cfg(feature = "keyvault")]
    pub fn secrets(&self) -> Option<super::keyvault::AzureKeyVaultSecrets> {
        self.secret_client.as_ref().map(|client| {
            super::keyvault::AzureKeyVaultSecrets::new(self.context.clone(), client.clone())
        })
    }
}

