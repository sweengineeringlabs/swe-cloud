//! Azure Blob Storage service traits.

use async_trait::async_trait;
use azure_control_spi::CloudResult;
use std::collections::HashMap;

/// Azure Blob Storage service trait.
#[async_trait]
pub trait BlobService: Send + Sync {
    /// Create a container.
    async fn create_container(&self, container: &str) -> CloudResult<()>;

    /// Delete a container.
    async fn delete_container(&self, container: &str) -> CloudResult<()>;

    /// List containers.
    async fn list_containers(&self) -> CloudResult<Vec<String>>;

    /// Upload a blob.
    async fn put_blob(&self, container: &str, blob: &str, data: Vec<u8>, metadata: HashMap<String, String>) -> CloudResult<()>;

    /// Download a blob.
    async fn get_blob(&self, container: &str, blob: &str) -> CloudResult<Vec<u8>>;

    /// Delete a blob.
    async fn delete_blob(&self, container: &str, blob: &str) -> CloudResult<()>;

    /// List blobs in a container.
    async fn list_blobs(&self, container: &str, prefix: Option<&str>) -> CloudResult<Vec<String>>;
}
