//! GCP client builder.

use cloudkit::common::{CloudConfig, CloudResult, Region};
use cloudkit::core::{CloudContext, ProviderType};
use std::sync::Arc;

#[cfg(feature = "gcs")]
use crate::gcs::GcsStorage;
#[cfg(feature = "pubsub")]
use crate::pubsub::GcpPubSub;
#[cfg(feature = "firestore")]
use crate::firestore::GcpFirestore;
#[cfg(feature = "secrets")]
use crate::secrets::GcpSecretManager;
#[cfg(feature = "monitor")]
use crate::monitor::GcpMonitor;
#[cfg(feature = "eventarc")]
use crate::eventarc::GcpEventarc;
#[cfg(feature = "identity")]
use crate::identity::GcpIdentity;
#[cfg(feature = "kms")]
use crate::kms::GcpKms;
#[cfg(feature = "workflows")]
use crate::workflows::GcpWorkflows;

/// GCP client builder.
pub struct GcpBuilder {
    region: Option<Region>,
    config: Option<CloudConfig>,
    project_id: Option<String>,
}

impl GcpBuilder {
    /// Create a new GCP builder.
    pub fn new() -> Self {
        Self {
            region: None,
            config: None,
            project_id: None,
        }
    }

    /// Set the GCP region.
    pub fn region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    /// Set the project ID.
    pub fn project(mut self, project_id: impl Into<String>) -> Self {
        self.project_id = Some(project_id.into());
        self
    }

    /// Set the configuration.
    pub fn config(mut self, config: CloudConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Build the GCP client.
    pub async fn build(self) -> CloudResult<GcpClient> {
        let mut config = self.config.unwrap_or_default();
        
        if let Some(region) = self.region {
            config.region = region;
        }

        let context = CloudContext::builder(ProviderType::Gcp)
            .config(config)
            .build()
            .await?;

        Ok(GcpClient {
            context: Arc::new(context),
            project_id: self.project_id,
        })
    }
}

impl Default for GcpBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// GCP client.
pub struct GcpClient {
    context: Arc<CloudContext>,
    project_id: Option<String>,
}

impl GcpClient {
    /// Get the underlying context.
    pub fn context(&self) -> &CloudContext {
        &self.context
    }

    /// Get the project ID.
    pub fn project_id(&self) -> Option<&str> {
        self.project_id.as_deref()
    }

    /// Get object storage client.
    #[cfg(feature = "gcs")]
    pub fn storage(&self) -> GcsStorage {
        GcsStorage::new(self.context.clone())
    }

    /// Get message queue client.
    #[cfg(feature = "pubsub")]
    pub fn queue(&self) -> GcpPubSub {
        GcpPubSub::new(self.context.clone())
    }

    /// Get key-value store client.
    #[cfg(feature = "firestore")]
    pub fn kv_store(&self) -> GcpFirestore {
        GcpFirestore::new(self.context.clone())
    }

    /// Get secrets manager client.
    #[cfg(feature = "secrets")]
    pub fn secrets(&self) -> GcpSecretManager {
        GcpSecretManager::new(self.context.clone())
    }

    /// Get monitor client (metrics & logging).
    #[cfg(feature = "monitor")]
    pub fn monitor(&self) -> GcpMonitor {
        GcpMonitor::new(self.context.clone())
    }

    /// Get event bus client.
    #[cfg(feature = "eventarc")]
    pub fn events(&self) -> GcpEventarc {
        GcpEventarc::new(self.context.clone())
    }

    /// Get identity provider client.
    #[cfg(feature = "identity")]
    pub fn identity(&self) -> GcpIdentity {
        GcpIdentity::new(self.context.clone())
    }

    /// Get KMS client.
    #[cfg(feature = "kms")]
    pub fn kms(&self) -> GcpKms {
        GcpKms::new(self.context.clone())
    }

    /// Get workflows client.
    #[cfg(feature = "workflows")]
    pub fn workflows(&self) -> GcpWorkflows {
        GcpWorkflows::new(self.context.clone())
    }
}
