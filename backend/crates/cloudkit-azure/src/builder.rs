//! Azure client builder.

use cloudkit::common::{CloudConfig, CloudResult, Region};
use cloudkit::core::{CloudContext, ProviderType};
use std::sync::Arc;

/// Azure client builder.
pub struct AzureBuilder {
    region: Option<Region>,
    config: Option<CloudConfig>,
    storage_account: Option<String>,
}

impl AzureBuilder {
    /// Create a new Azure builder.
    pub fn new() -> Self {
        Self {
            region: None,
            config: None,
            storage_account: None,
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

    /// Set the configuration.
    pub fn config(mut self, config: CloudConfig) -> Self {
        self.config = Some(config);
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

        Ok(AzureClient {
            context: Arc::new(context),
            storage_account: self.storage_account,
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
}
