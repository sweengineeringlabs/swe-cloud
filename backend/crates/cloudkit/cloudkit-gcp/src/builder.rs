//! GCP client builder.

use cloudkit::common::{CloudConfig, CloudResult, Region};
use cloudkit::core::{CloudContext, ProviderType};
use std::sync::Arc;

#[cfg(feature = "eventarc")]
use crate::eventarc::GcpEventarc;
#[cfg(feature = "firestore")]
use crate::firestore::GcpFirestore;
#[cfg(feature = "gcs")]
use crate::gcs::GcsStorage;
#[cfg(feature = "identity")]
use crate::identity::GcpIdentity;
#[cfg(feature = "kms")]
use crate::kms::GcpKms;
#[cfg(feature = "monitor")]
use crate::monitor::GcpMonitor;
#[cfg(feature = "pubsub")]
use crate::pubsub::GcpPubSub;
#[cfg(feature = "secrets")]
use crate::secrets::GcpSecretManager;
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

        if let Some(ref region) = self.region {
            config.region = region.clone();
        }

        let context = CloudContext::builder(ProviderType::Gcp)
            .config(config)
            .build()
            .await?;

        // Initialize services
        #[cfg(feature = "gcs")]
        let gcs_client = {
            let config = google_cloud_storage::client::ClientConfig::default()
                .with_auth()
                .await
                .map_err(|e| cloudkit::common::CloudError::Config(e.to_string()))?;
            google_cloud_storage::client::Client::new(config)
        };

        #[cfg(feature = "pubsub")]
        let pubsub_client = {
            let config = google_cloud_pubsub::client::ClientConfig::default()
                .with_auth()
                .await
                .map_err(|e| cloudkit::common::CloudError::Config(e.to_string()))?;
            google_cloud_pubsub::client::Client::new(config)
                .await
                .map_err(|e| cloudkit::common::CloudError::Provider {
                    provider: "gcp".to_string(),
                    code: "PubSubInit".to_string(),
                    message: e.to_string(),
                })?
        };

        #[cfg(feature = "firestore")]
        let firestore_client = {
            let project_id_val = self.project_id.clone().unwrap_or_default();
            firestore::FirestoreDb::new(&project_id_val)
                .await
                .map_err(|e| cloudkit::common::CloudError::Provider {
                    provider: "gcp".to_string(),
                    code: "FirestoreInit".to_string(),
                    message: e.to_string(),
                })?
        };
        #[cfg(feature = "secrets")]
        let secrets_auth = {
            let config = google_cloud_auth::project::Config {
                scopes: Some(&["https://www.googleapis.com/auth/cloud-platform"]),
                ..Default::default()
            };
            let ts = google_cloud_auth::project::create_token_source(config).await.map_err(|e| 
                cloudkit::common::CloudError::Provider { 
                    provider: "gcp".to_string(), 
                    code: "AuthError".to_string(), 
                    message: e.to_string() 
                }
            )?;
            Arc::new(ts)
        };

        #[cfg(feature = "monitor")]
        let monitor_auth = {
            let config = google_cloud_auth::project::Config {
                scopes: Some(&["https://www.googleapis.com/auth/cloud-platform"]),
                ..Default::default()
            };
            let ts = google_cloud_auth::project::create_token_source(config).await.map_err(|e| 
                cloudkit::common::CloudError::Provider { 
                    provider: "gcp".to_string(), 
                    code: "AuthError".to_string(), 
                    message: e.to_string() 
                }
            )?;
            Arc::new(ts)
        };

        #[cfg(feature = "kms")]
        let kms_auth = {
            let config = google_cloud_auth::project::Config {
                scopes: Some(&["https://www.googleapis.com/auth/cloud-platform"]),
                ..Default::default()
            };
            let ts = google_cloud_auth::project::create_token_source(config).await.map_err(|e| 
                cloudkit::common::CloudError::Provider { 
                    provider: "gcp".to_string(), 
                    code: "AuthError".to_string(), 
                    message: e.to_string() 
                }
            )?;
            Arc::new(ts)
        };

        #[cfg(feature = "identity")]
        let identity_auth = {
            let config = google_cloud_auth::project::Config {
                scopes: Some(&["https://www.googleapis.com/auth/cloud-platform"]),
                ..Default::default()
            };
            let ts = google_cloud_auth::project::create_token_source(config).await.map_err(|e| 
                cloudkit::common::CloudError::Provider { 
                    provider: "gcp".to_string(), 
                    code: "AuthError".to_string(), 
                    message: e.to_string() 
                }
            )?;
            Arc::new(ts)
        };

        #[cfg(feature = "eventarc")]
        let eventarc_auth = {
            let config = google_cloud_auth::project::Config {
                scopes: Some(&["https://www.googleapis.com/auth/cloud-platform"]),
                ..Default::default()
            };
            let ts = google_cloud_auth::project::create_token_source(config).await.map_err(|e| 
                cloudkit::common::CloudError::Provider { 
                    provider: "gcp".to_string(), 
                    code: "AuthError".to_string(), 
                    message: e.to_string() 
                }
            )?;
            Arc::new(ts)
        };

        #[cfg(feature = "workflows")]
        let workflows_auth = {
            let config = google_cloud_auth::project::Config {
                scopes: Some(&["https://www.googleapis.com/auth/cloud-platform"]),
                ..Default::default()
            };
            let ts = google_cloud_auth::project::create_token_source(config).await.map_err(|e| 
                cloudkit::common::CloudError::Provider { 
                    provider: "gcp".to_string(), 
                    code: "AuthError".to_string(), 
                    message: e.to_string() 
                }
            )?;
            Arc::new(ts)
        };


        Ok(GcpClient {
            context: Arc::new(context),
            project_id: self.project_id,
            #[cfg(feature = "gcs")]
            gcs_client,
            #[cfg(feature = "pubsub")]
            pubsub_client,
            #[cfg(feature = "firestore")]
            firestore_client,
            #[cfg(feature = "secrets")]
            secrets_auth,
            #[cfg(feature = "monitor")]
            monitor_auth,
            #[cfg(feature = "kms")]
            kms_auth,
            #[cfg(feature = "identity")]
            identity_auth,
            #[cfg(feature = "eventarc")]
            eventarc_auth,
            #[cfg(feature = "workflows")]
            workflows_auth,
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
    #[cfg(feature = "gcs")]
    gcs_client: google_cloud_storage::client::Client,
    #[cfg(feature = "pubsub")]
    pubsub_client: google_cloud_pubsub::client::Client,
    #[cfg(feature = "firestore")]
    firestore_client: firestore::FirestoreDb,
    #[cfg(feature = "secrets")]
    secrets_auth: Arc<Box<dyn google_cloud_auth::token_source::TokenSource>>,
    #[cfg(feature = "monitor")]
    monitor_auth: Arc<Box<dyn google_cloud_auth::token_source::TokenSource>>,
    #[cfg(feature = "kms")]
    kms_auth: Arc<Box<dyn google_cloud_auth::token_source::TokenSource>>,
    #[cfg(feature = "identity")]
    identity_auth: Arc<Box<dyn google_cloud_auth::token_source::TokenSource>>,
    #[cfg(feature = "eventarc")]
    eventarc_auth: Arc<Box<dyn google_cloud_auth::token_source::TokenSource>>,
    #[cfg(feature = "workflows")]
    workflows_auth: Arc<Box<dyn google_cloud_auth::token_source::TokenSource>>,
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
        GcsStorage::new(
            self.context.clone(),
            self.gcs_client.clone(),
            self.project_id.clone().unwrap_or_default(),
        )
    }

    /// Get message queue client.
    #[cfg(feature = "pubsub")]
    pub fn queue(&self) -> GcpPubSub {
        GcpPubSub::new(
            self.context.clone(),
            self.pubsub_client.clone(),
            self.project_id.clone().unwrap_or_default(),
        )
    }

    /// Get key-value store client.
    #[cfg(feature = "firestore")]
    pub fn kv_store(&self) -> GcpFirestore {
        GcpFirestore::new(self.context.clone(), self.firestore_client.clone())
    }

    /// Get secrets manager client.
    #[cfg(feature = "secrets")]
    pub fn secrets(&self) -> GcpSecretManager {
        GcpSecretManager::new(
            self.context.clone(),
            self.secrets_auth.clone(),
            self.project_id.clone().unwrap_or_default()
        )
    }

    /// Get monitor client (metrics & logging).
    #[cfg(feature = "monitor")]
    pub fn monitor(&self) -> GcpMonitor {
        GcpMonitor::new(
            self.context.clone(),
            self.monitor_auth.clone(),
            self.project_id.clone().unwrap_or_default()
        )
    }

    /// Get event bus client.
    #[cfg(feature = "eventarc")]
    pub fn events(&self) -> GcpEventarc {
        GcpEventarc::new(
            self.context.clone(),
            self.eventarc_auth.clone(),
            self.project_id.clone().unwrap_or_default()
        )
    }

    /// Get identity provider client.
    #[cfg(feature = "identity")]
    pub fn identity(&self) -> GcpIdentity {
        GcpIdentity::new(
            self.context.clone(),
            self.identity_auth.clone(),
            self.project_id.clone().unwrap_or_default()
        )
    }

    /// Get KMS client.
    #[cfg(feature = "kms")]
    pub fn kms(&self) -> GcpKms {
        GcpKms::new(
            self.context.clone(),
            self.kms_auth.clone(),
            self.project_id.clone().unwrap_or_default()
        )
    }

    /// Get workflows client.
    #[cfg(feature = "workflows")]
    pub fn workflows(&self) -> GcpWorkflows {
        GcpWorkflows::new(
            self.context.clone(),
            self.workflows_auth.clone(),
            self.project_id.clone().unwrap_or_default()
        )
    }
}
