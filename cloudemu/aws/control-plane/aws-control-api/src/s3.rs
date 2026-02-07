//! S3 service traits.

use async_trait::async_trait;
use aws_control_spi::CloudResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// S3 bucket metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketInfo {
    /// Bucket name
    pub name: String,
    /// Creation date (ISO 8601)
    pub creation_date: String,
}

/// S3 object metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectInfo {
    /// Object key
    pub key: String,
    /// Size in bytes
    pub size: u64,
    /// Last modified (ISO 8601)
    pub last_modified: String,
    /// ETag
    pub etag: String,
}

/// S3 storage service trait.
#[async_trait]
pub trait S3Service: Send + Sync {
    /// Create a bucket.
    async fn create_bucket(&self, bucket: &str) -> CloudResult<()>;

    /// Delete a bucket.
    async fn delete_bucket(&self, bucket: &str) -> CloudResult<()>;

    /// List all buckets.
    async fn list_buckets(&self) -> CloudResult<Vec<BucketInfo>>;

    /// Put an object.
    async fn put_object(&self, bucket: &str, key: &str, data: Vec<u8>, metadata: HashMap<String, String>) -> CloudResult<String>;

    /// Get an object.
    async fn get_object(&self, bucket: &str, key: &str) -> CloudResult<(Vec<u8>, HashMap<String, String>)>;

    /// Delete an object.
    async fn delete_object(&self, bucket: &str, key: &str) -> CloudResult<()>;

    /// Head object (metadata only).
    async fn head_object(&self, bucket: &str, key: &str) -> CloudResult<ObjectInfo>;

    /// List objects in a bucket.
    async fn list_objects(&self, bucket: &str, prefix: Option<&str>, max_keys: Option<usize>) -> CloudResult<Vec<ObjectInfo>>;
}
