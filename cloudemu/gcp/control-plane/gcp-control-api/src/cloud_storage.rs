//! GCP Cloud Storage service traits.

use async_trait::async_trait;
use gcp_control_spi::CloudResult;
use std::collections::HashMap;

/// GCP Cloud Storage service trait.
#[async_trait]
pub trait CloudStorageService: Send + Sync {
    /// Create a bucket.
    async fn create_bucket(&self, bucket: &str, project_id: &str) -> CloudResult<()>;

    /// Delete a bucket.
    async fn delete_bucket(&self, bucket: &str) -> CloudResult<()>;

    /// List buckets.
    async fn list_buckets(&self, project_id: &str) -> CloudResult<Vec<String>>;

    /// Upload an object.
    async fn upload_object(&self, bucket: &str, object: &str, data: Vec<u8>, metadata: HashMap<String, String>) -> CloudResult<()>;

    /// Download an object.
    async fn download_object(&self, bucket: &str, object: &str) -> CloudResult<Vec<u8>>;

    /// Delete an object.
    async fn delete_object(&self, bucket: &str, object: &str) -> CloudResult<()>;

    /// List objects.
    async fn list_objects(&self, bucket: &str, prefix: Option<&str>) -> CloudResult<Vec<String>>;
}
