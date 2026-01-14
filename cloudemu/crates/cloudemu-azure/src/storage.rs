//! Azure storage engine implementation.

use cloudemu_core::{CloudResource, CloudResult, ResourceFilter, StorageEngine};

/// Azure storage engine (stub implementation).
pub struct AzureStorageEngine;

impl AzureStorageEngine {
    /// Create a new Azure storage engine.
    pub fn new() -> Self {
        Self
    }
}

impl Default for AzureStorageEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl StorageEngine for AzureStorageEngine {
    async fn store(&self, _resource: CloudResource) -> CloudResult<()> {
        // TODO: Implement Azure storage
        Ok(())
    }

    async fn retrieve(&self, id: &str) -> CloudResult<CloudResource> {
        // TODO: Implement Azure storage
        Err(cloudemu_core::CloudError::NotFound {
            resource_type: "Resource".to_string(),
            resource_id: id.to_string(),
        })
    }

    async fn list(&self, _filter: ResourceFilter) -> CloudResult<Vec<CloudResource>> {
        // TODO: Implement Azure storage
        Ok(vec![])
    }

    async fn update(&self, _resource: CloudResource) -> CloudResult<()> {
        // TODO: Implement Azure storage
        Ok(())
    }

    async fn delete(&self, _id: &str) -> CloudResult<()> {
        // TODO: Implement Azure storage
        Ok(())
    }
}
