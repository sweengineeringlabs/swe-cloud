//! Object storage trait for blob/object storage operations.

use crate::common::{BucketMetadata, CloudResult, ListResult, ObjectMetadata};
use async_trait::async_trait;
use bytes::Bytes;

/// Options for put operations.
#[derive(Debug, Clone, Default)]
pub struct PutOptions {
    /// Content type
    pub content_type: Option<String>,
    /// Cache control header
    pub cache_control: Option<String>,
    /// Content encoding
    pub content_encoding: Option<String>,
    /// Custom metadata
    pub metadata: std::collections::HashMap<String, String>,
    /// Storage class (provider-specific)
    pub storage_class: Option<String>,
}

impl PutOptions {
    /// Create new put options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set content type.
    pub fn content_type(mut self, ct: impl Into<String>) -> Self {
        self.content_type = Some(ct.into());
        self
    }

    /// Set cache control.
    pub fn cache_control(mut self, cc: impl Into<String>) -> Self {
        self.cache_control = Some(cc.into());
        self
    }

    /// Add custom metadata.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Options for get operations.
#[derive(Debug, Clone, Default)]
pub struct GetOptions {
    /// Range start (bytes)
    pub range_start: Option<u64>,
    /// Range end (bytes)
    pub range_end: Option<u64>,
    /// If-match ETag
    pub if_match: Option<String>,
    /// If-none-match ETag
    pub if_none_match: Option<String>,
}

impl GetOptions {
    /// Create new get options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set byte range.
    pub fn range(mut self, start: u64, end: u64) -> Self {
        self.range_start = Some(start);
        self.range_end = Some(end);
        self
    }
}

/// Options for list operations.
#[derive(Debug, Clone, Default)]
pub struct ListOptions {
    /// Prefix filter
    pub prefix: Option<String>,
    /// Delimiter for hierarchy
    pub delimiter: Option<String>,
    /// Maximum number of results
    pub max_results: Option<u32>,
    /// Continuation token
    pub continuation_token: Option<String>,
}

impl ListOptions {
    /// Create new list options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set prefix filter.
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Set delimiter.
    pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.delimiter = Some(delimiter.into());
        self
    }

    /// Set max results.
    pub fn max_results(mut self, max: u32) -> Self {
        self.max_results = Some(max);
        self
    }

    /// Set continuation token.
    pub fn continuation_token(mut self, token: impl Into<String>) -> Self {
        self.continuation_token = Some(token.into());
        self
    }
}

/// Object storage service trait.
///
/// This trait abstracts blob/object storage operations across cloud providers:
/// - AWS S3
/// - Azure Blob Storage  
/// - Google Cloud Storage
/// - Oracle Object Storage
///
/// # Example
///
/// ```rust,ignore
/// use cloudkit::api::ObjectStorage;
///
/// async fn upload<S: ObjectStorage>(storage: &S) -> CloudResult<()> {
///     storage.put_object("bucket", "key", b"data").await?;
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait ObjectStorage: Send + Sync {
    // =========================================================================
    // Bucket Operations
    // =========================================================================

    /// List all buckets/containers.
    async fn list_buckets(&self) -> CloudResult<Vec<BucketMetadata>>;

    /// Create a new bucket/container.
    async fn create_bucket(&self, bucket: &str) -> CloudResult<()>;

    /// Delete a bucket/container.
    async fn delete_bucket(&self, bucket: &str) -> CloudResult<()>;

    /// Check if a bucket exists.
    async fn bucket_exists(&self, bucket: &str) -> CloudResult<bool>;

    // =========================================================================
    // Object Operations
    // =========================================================================

    /// Upload an object.
    async fn put_object(&self, bucket: &str, key: &str, data: &[u8]) -> CloudResult<()>;

    /// Upload an object with options.
    async fn put_object_with_options(
        &self,
        bucket: &str,
        key: &str,
        data: &[u8],
        options: PutOptions,
    ) -> CloudResult<()>;

    /// Download an object.
    async fn get_object(&self, bucket: &str, key: &str) -> CloudResult<Bytes>;

    /// Download an object with options.
    async fn get_object_with_options(
        &self,
        bucket: &str,
        key: &str,
        options: GetOptions,
    ) -> CloudResult<Bytes>;

    /// Get object metadata without downloading.
    async fn head_object(&self, bucket: &str, key: &str) -> CloudResult<ObjectMetadata>;

    /// Delete an object.
    async fn delete_object(&self, bucket: &str, key: &str) -> CloudResult<()>;

    /// Delete multiple objects.
    async fn delete_objects(&self, bucket: &str, keys: &[&str]) -> CloudResult<()>;

    /// Copy an object.
    async fn copy_object(
        &self,
        source_bucket: &str,
        source_key: &str,
        dest_bucket: &str,
        dest_key: &str,
    ) -> CloudResult<()>;

    /// Check if an object exists.
    async fn object_exists(&self, bucket: &str, key: &str) -> CloudResult<bool>;

    /// List objects in a bucket.
    async fn list_objects(
        &self,
        bucket: &str,
        options: ListOptions,
    ) -> CloudResult<ListResult<ObjectMetadata>>;

    // =========================================================================
    // Presigned URLs
    // =========================================================================

    /// Generate a presigned URL for downloading.
    async fn presigned_get_url(
        &self,
        bucket: &str,
        key: &str,
        expires_in: std::time::Duration,
    ) -> CloudResult<String>;

    /// Generate a presigned URL for uploading.
    async fn presigned_put_url(
        &self,
        bucket: &str,
        key: &str,
        expires_in: std::time::Duration,
    ) -> CloudResult<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_options_builder() {
        let options = PutOptions::new()
            .content_type("application/json")
            .cache_control("max-age=3600")
            .metadata("custom", "value");

        assert_eq!(options.content_type, Some("application/json".to_string()));
        assert_eq!(options.cache_control, Some("max-age=3600".to_string()));
        assert_eq!(options.metadata.get("custom"), Some(&"value".to_string()));
    }

    #[test]
    fn test_list_options_builder() {
        let options = ListOptions::new()
            .prefix("folder/")
            .delimiter("/")
            .max_results(100);

        assert_eq!(options.prefix, Some("folder/".to_string()));
        assert_eq!(options.delimiter, Some("/".to_string()));
        assert_eq!(options.max_results, Some(100));
    }
}
