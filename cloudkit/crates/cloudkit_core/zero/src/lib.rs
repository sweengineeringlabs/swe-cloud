//! ZeroCloud provider for CloudKit.

pub mod s3;
pub mod dynamodb;
pub mod lambda;
pub mod sqs;
pub mod iam;

use cloudkit_spi::{CloudConfig, CloudResult, CloudContext, ProviderType, Region};
use std::sync::Arc;
use zero_sdk::ZeroClient as RawZeroClient;

/// Builder for ZeroCloud provider.
pub struct ZeroBuilder {
    config: Option<CloudConfig>,
}

impl ZeroBuilder {
    /// Create a new ZeroCloud builder.
    pub fn new() -> Self {
        Self { config: None }
    }

    /// Set the cloud configuration.
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

    /// Set the region.
    pub fn region(mut self, region: Region) -> Self {
        let config = self.config.get_or_insert_with(CloudConfig::default);
        config.region = region;
        self
    }

    /// Build the ZeroCloud provider clients.
    pub async fn build(self) -> CloudResult<ZeroClient> {
        let config = self.config.unwrap_or_default();
        let endpoint = config.endpoint.clone().unwrap_or_else(|| "http://localhost:8080".to_string());
        
        let context = CloudContext::builder(ProviderType::Zero)
            .config(config)
            .build()
            .await?;

        Ok(ZeroClient {
            context: Arc::new(context),
            sdk: RawZeroClient::new(endpoint),
        })
    }
}

impl Default for ZeroBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// ZeroCloud client with CloudKit service implementations.
pub struct ZeroClient {
    context: Arc<CloudContext>,
    sdk: RawZeroClient,
}

impl ZeroClient {
    /// Get the underlying context.
    pub fn context(&self) -> &CloudContext {
        &self.context
    }

    /// Get the storage client.
    pub fn storage(&self) -> s3::ZeroStore {
        s3::ZeroStore::new(self.sdk.clone())
    }

    /// Get the key-value store client.
    pub fn kv_store(&self) -> dynamodb::ZeroDb {
        dynamodb::ZeroDb::new(self.sdk.clone())
    }

    /// Get the message queue client.
    pub fn queue(&self) -> sqs::ZeroQueue {
        sqs::ZeroQueue::new(self.sdk.clone())
    }

    /// Get the serverless functions client.
    pub fn functions(&self) -> lambda::ZeroFunc {
        lambda::ZeroFunc::new(self.sdk.clone())
    }

    /// Get the identity provider client.
    pub fn identity(&self) -> iam::ZeroId {
        iam::ZeroId::new(self.sdk.clone())
    }
}
