//! Cloud provider and storage engine trait definitions.

use crate::{CloudResource, CloudResult, Request, ResourceFilter, Response, ServiceType};
use async_trait::async_trait;

/// Provider-agnostic cloud service interface.
///
/// Each cloud provider (AWS, Azure, GCP) implements this trait to handle
/// incoming HTTP requests and route them to the appropriate service handlers.
#[async_trait]
pub trait CloudProviderTrait: Send + Sync {
    /// Handle an incoming HTTP request.
    async fn handle_request(&self, req: Request) -> CloudResult<Response>;

    /// Get the list of supported services for this provider.
    fn supported_services(&self) -> Vec<ServiceType>;

    /// Get the default port for this provider.
    fn default_port(&self) -> u16;

    /// Get the provider name.
    fn provider_name(&self) -> &str;
}

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
