//! Storage engine trait for provider-agnostic persistence.

use crate::{CloudResource, CloudResult, ResourceFilter};
use async_trait::async_trait;

/// Provider-agnostic storage interface.
///
/// This trait defines the persistence contract that all cloud providers must implement.
/// It enables CloudEmu to support multiple storage backends (SQLite, PostgreSQL, etc.)
/// while maintaining a consistent interface.
#[async_trait]
pub trait StorageEngine: Send + Sync {
    /// Store a cloud resource.
    async fn store(&self, resource: CloudResource) -> CloudResult<()>;

    /// Retrieve a resource by ID.
    async fn retrieve(&self, id: &str) -> CloudResult<CloudResource>;

    /// List resources matching a filter.
    async fn list(&self, filter: ResourceFilter) -> CloudResult<Vec<CloudResource>>;

    /// Update a resource.
    async fn update(&self, resource: CloudResource) -> CloudResult<()>;

    /// Delete a resource by ID.
    async fn delete(&self, id: &str) -> CloudResult<()>;

    /// Check if a resource exists.
    async fn exists(&self, id: &str) -> CloudResult<bool> {
        match self.retrieve(id).await {
            Ok(_) => Ok(true),
            Err(crate::CloudError::NotFound { .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Get all resources for a specific provider.
    async fn list_by_provider(
        &self,
        provider: crate::CloudProvider,
    ) -> CloudResult<Vec<CloudResource>> {
        let filter = ResourceFilter::new().provider(provider);
        self.list(filter).await
    }

    /// Get all resources for a specific service type.
    async fn list_by_service(
        &self,
        service_type: crate::ServiceType,
    ) -> CloudResult<Vec<CloudResource>> {
        let filter = ResourceFilter::new().service_type(service_type);
        self.list(filter).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CloudProvider, ServiceType};
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    /// In-memory storage implementation for testing.
    struct InMemoryStorage {
        resources: Arc<RwLock<HashMap<String, CloudResource>>>,
    }

    impl InMemoryStorage {
        fn new() -> Self {
            Self {
                resources: Arc::new(RwLock::new(HashMap::new())),
            }
        }
    }

    #[async_trait]
    impl StorageEngine for InMemoryStorage {
        async fn store(&self, resource: CloudResource) -> CloudResult<()> {
            let mut map = self.resources.write().await;
            map.insert(resource.id.clone(), resource);
            Ok(())
        }

        async fn retrieve(&self, id: &str) -> CloudResult<CloudResource> {
            let map = self.resources.read().await;
            map.get(id)
                .cloned()
                .ok_or_else(|| crate::CloudError::NotFound {
                    resource_type: "Resource".to_string(),
                    resource_id: id.to_string(),
                })
        }

        async fn list(&self, filter: ResourceFilter) -> CloudResult<Vec<CloudResource>> {
            let map = self.resources.read().await;
            Ok(map
                .values()
                .filter(|r| filter.matches(r))
                .cloned()
                .collect())
        }

        async fn update(&self, resource: CloudResource) -> CloudResult<()> {
            let mut map = self.resources.write().await;
            if !map.contains_key(&resource.id) {
                return Err(crate::CloudError::NotFound {
                    resource_type: "Resource".to_string(),
                    resource_id: resource.id.clone(),
                });
            }
            map.insert(resource.id.clone(), resource);
            Ok(())
        }

        async fn delete(&self, id: &str) -> CloudResult<()> {
            let mut map = self.resources.write().await;
            map.remove(id).ok_or_else(|| crate::CloudError::NotFound {
                resource_type: "Resource".to_string(),
                resource_id: id.to_string(),
            })?;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_storage_engine() {
        let storage = InMemoryStorage::new();

        // Store a resource
        let resource = CloudResource::new(
            "test-bucket",
            CloudProvider::Aws,
            ServiceType::ObjectStorage,
            "my-bucket",
        );
        storage.store(resource.clone()).await.unwrap();

        // Retrieve it
        let retrieved = storage.retrieve("test-bucket").await.unwrap();
        assert_eq!(retrieved.id, "test-bucket");
        assert_eq!(retrieved.name, "my-bucket");

        // List resources
        let resources = storage.list(ResourceFilter::new()).await.unwrap();
        assert_eq!(resources.len(), 1);

        // Delete it
        storage.delete("test-bucket").await.unwrap();

        // Verify it's gone
        assert!(!storage.exists("test-bucket").await.unwrap());
    }
}
