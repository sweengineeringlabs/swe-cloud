//! GCP storage engine implementation.

use cloudemu_core::{CloudResource, CloudResult, ResourceFilter, StorageEngine};

/// GCP storage engine (stub implementation).
pub struct GcpStorageEngine;

impl GcpStorageEngine {
    /// Create a new GCP storage engine.
    pub fn new() -> Self {
        Self
    }
}

impl Default for GcpStorageEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl StorageEngine for GcpStorageEngine {
    async fn store(&self, _resource: CloudResource) -> CloudResult<()> {
        // TODO: Implement GCP storage
        Ok(())
    }

    async fn retrieve(&self, id: &str) -> CloudResult<CloudResource> {
        // TODO: Implement GCP storage
        Err(cloudemu_core::CloudError::NotFound {
            resource_type: "Resource".to_string(),
            resource_id: id.to_string(),
        })
    }

    async fn list(&self, _filter: ResourceFilter) -> CloudResult<Vec<CloudResource>> {
        // TODO: Implement GCP storage
        Ok(vec![])
    }

    async fn update(&self, _resource: CloudResource) -> CloudResult<()> {
        // TODO: Implement GCP storage
        Ok(())
    }

    async fn delete(&self, _id: &str) -> CloudResult<()> {
        // TODO: Implement GCP storage
        Ok(())
    }
}
