//! GCP client builder.

use cloudkit::common::{CloudConfig, CloudResult, Region};
use cloudkit::core::{CloudContext, ProviderType};
use std::sync::Arc;

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
}
