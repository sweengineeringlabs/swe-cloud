//! Testing example for CloudKit.
//!
//! This example demonstrates how to test code that uses CloudKit.
//!
//! Run with: `cargo run --example testing`

use async_trait::async_trait;
use bytes::Bytes;
use cloudkit::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// =============================================================================
// Mock Implementation
// =============================================================================

/// In-memory mock storage for testing.
#[derive(Default)]
pub struct MockStorage {
    buckets: Arc<Mutex<HashMap<String, HashMap<String, Vec<u8>>>>>,
}

impl MockStorage {
    pub fn new() -> Self {
        Self::default()
    }

    /// Pre-populate with test data.
    pub fn with_data(bucket: &str, key: &str, data: &[u8]) -> Self {
        let storage = Self::new();
        {
            let mut buckets = storage.buckets.lock().unwrap();
            let bucket_data = buckets.entry(bucket.to_string()).or_default();
            bucket_data.insert(key.to_string(), data.to_vec());
        }
        storage
    }
}

#[async_trait]
impl ObjectStorage for MockStorage {
    async fn list_buckets(&self) -> CloudResult<Vec<BucketMetadata>> {
        let buckets = self.buckets.lock().unwrap();
        Ok(buckets.keys().map(|name| BucketMetadata {
            name: name.clone(),
            created_at: chrono::Utc::now(),
            region: "mock".to_string(),
            versioning_enabled: false,
        }).collect())
    }

    async fn create_bucket(&self, bucket: &str) -> CloudResult<()> {
        let mut buckets = self.buckets.lock().unwrap();
        if buckets.contains_key(bucket) {
            return Err(CloudError::AlreadyExists {
                resource_type: "Bucket".to_string(),
                resource_id: bucket.to_string(),
            });
        }
        buckets.insert(bucket.to_string(), HashMap::new());
        Ok(())
    }

    async fn delete_bucket(&self, bucket: &str) -> CloudResult<()> {
        let mut buckets = self.buckets.lock().unwrap();
        buckets.remove(bucket);
        Ok(())
    }

    async fn bucket_exists(&self, bucket: &str) -> CloudResult<bool> {
        let buckets = self.buckets.lock().unwrap();
        Ok(buckets.contains_key(bucket))
    }

    async fn put_object(&self, bucket: &str, key: &str, data: &[u8]) -> CloudResult<()> {
        let mut buckets = self.buckets.lock().unwrap();
        let bucket_data = buckets.entry(bucket.to_string()).or_default();
        bucket_data.insert(key.to_string(), data.to_vec());
        Ok(())
    }

    async fn put_object_with_options(
        &self,
        bucket: &str,
        key: &str,
        data: &[u8],
        _options: PutOptions,
    ) -> CloudResult<()> {
        self.put_object(bucket, key, data).await
    }

    async fn get_object(&self, bucket: &str, key: &str) -> CloudResult<Bytes> {
        let buckets = self.buckets.lock().unwrap();
        let bucket_data = buckets.get(bucket).ok_or_else(|| CloudError::NotFound {
            resource_type: "Bucket".to_string(),
            resource_id: bucket.to_string(),
        })?;
        
        bucket_data.get(key)
            .map(|data| Bytes::from(data.clone()))
            .ok_or_else(|| CloudError::NotFound {
                resource_type: "Object".to_string(),
                resource_id: format!("{}/{}", bucket, key),
            })
    }

    async fn get_object_with_options(
        &self,
        bucket: &str,
        key: &str,
        _options: GetOptions,
    ) -> CloudResult<Bytes> {
        self.get_object(bucket, key).await
    }

    async fn head_object(&self, bucket: &str, key: &str) -> CloudResult<ObjectMetadata> {
        let data = self.get_object(bucket, key).await?;
        Ok(ObjectMetadata {
            key: key.to_string(),
            size: data.len() as u64,
            content_type: Some("application/octet-stream".to_string()),
            etag: Some("mock-etag".to_string()),
            last_modified: chrono::Utc::now(),
            storage_class: None,
            metadata: HashMap::new(),
        })
    }

    async fn delete_object(&self, bucket: &str, key: &str) -> CloudResult<()> {
        let mut buckets = self.buckets.lock().unwrap();
        if let Some(bucket_data) = buckets.get_mut(bucket) {
            bucket_data.remove(key);
        }
        Ok(())
    }

    async fn delete_objects(&self, bucket: &str, keys: &[&str]) -> CloudResult<()> {
        for key in keys {
            self.delete_object(bucket, key).await?;
        }
        Ok(())
    }

    async fn copy_object(
        &self,
        source_bucket: &str,
        source_key: &str,
        dest_bucket: &str,
        dest_key: &str,
    ) -> CloudResult<()> {
        let data = self.get_object(source_bucket, source_key).await?;
        self.put_object(dest_bucket, dest_key, &data).await
    }

    async fn object_exists(&self, bucket: &str, key: &str) -> CloudResult<bool> {
        match self.get_object(bucket, key).await {
            Ok(_) => Ok(true),
            Err(CloudError::NotFound { .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn list_objects(
        &self,
        bucket: &str,
        options: ListOptions,
    ) -> CloudResult<ListResult<ObjectMetadata>> {
        let buckets = self.buckets.lock().unwrap();
        let bucket_data = buckets.get(bucket).ok_or_else(|| CloudError::NotFound {
            resource_type: "Bucket".to_string(),
            resource_id: bucket.to_string(),
        })?;

        let prefix = options.prefix.unwrap_or_default();
        let max = options.max_results.unwrap_or(1000) as usize;

        let items: Vec<ObjectMetadata> = bucket_data
            .iter()
            .filter(|(key, _)| key.starts_with(&prefix))
            .take(max)
            .map(|(key, data)| ObjectMetadata {
                key: key.clone(),
                size: data.len() as u64,
                content_type: Some("application/octet-stream".to_string()),
                etag: Some("mock-etag".to_string()),
                last_modified: chrono::Utc::now(),
                storage_class: None,
                metadata: HashMap::new(),
            })
            .collect();

        Ok(ListResult::new(items, PaginationToken::none()))
    }

    async fn presigned_get_url(
        &self,
        bucket: &str,
        key: &str,
        _expires_in: std::time::Duration,
    ) -> CloudResult<String> {
        Ok(format!("https://mock.storage/{}/{}", bucket, key))
    }

    async fn presigned_put_url(
        &self,
        bucket: &str,
        key: &str,
        _expires_in: std::time::Duration,
    ) -> CloudResult<String> {
        Ok(format!("https://mock.storage/{}/{}?upload=true", bucket, key))
    }
}

// =============================================================================
// Example Business Logic
// =============================================================================

/// Example service that uses ObjectStorage.
pub struct FileService<S: ObjectStorage> {
    storage: S,
    bucket: String,
}

impl<S: ObjectStorage> FileService<S> {
    pub fn new(storage: S, bucket: String) -> Self {
        Self { storage, bucket }
    }

    pub async fn save_file(&self, name: &str, content: &[u8]) -> CloudResult<()> {
        self.storage.put_object(&self.bucket, name, content).await
    }

    pub async fn load_file(&self, name: &str) -> CloudResult<Vec<u8>> {
        let data = self.storage.get_object(&self.bucket, name).await?;
        Ok(data.to_vec())
    }

    pub async fn file_exists(&self, name: &str) -> CloudResult<bool> {
        self.storage.object_exists(&self.bucket, name).await
    }

    pub async fn delete_file(&self, name: &str) -> CloudResult<()> {
        self.storage.delete_object(&self.bucket, name).await
    }

    pub async fn list_files(&self, prefix: &str) -> CloudResult<Vec<String>> {
        let result = self.storage
            .list_objects(&self.bucket, ListOptions::new().prefix(prefix))
            .await?;
        
        Ok(result.items.into_iter().map(|obj| obj.key).collect())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_save_and_load() {
        let storage = MockStorage::new();
        let service = FileService::new(storage, "test-bucket".to_string());

        // Save a file
        service.save_file("test.txt", b"Hello, World!").await.unwrap();

        // Load it back
        let content = service.load_file("test.txt").await.unwrap();
        assert_eq!(content, b"Hello, World!");
    }

    #[tokio::test]
    async fn test_file_not_found() {
        let storage = MockStorage::new();
        let service = FileService::new(storage, "test-bucket".to_string());

        let result = service.load_file("nonexistent.txt").await;
        
        assert!(matches!(
            result,
            Err(CloudError::NotFound { resource_type, .. }) if resource_type == "Object"
        ));
    }

    #[tokio::test]
    async fn test_file_exists() {
        let storage = MockStorage::new();
        let service = FileService::new(storage, "test-bucket".to_string());

        // File doesn't exist yet
        assert!(!service.file_exists("test.txt").await.unwrap());

        // Create the file
        service.save_file("test.txt", b"data").await.unwrap();

        // Now it exists
        assert!(service.file_exists("test.txt").await.unwrap());
    }

    #[tokio::test]
    async fn test_delete_file() {
        let storage = MockStorage::new();
        let service = FileService::new(storage, "test-bucket".to_string());

        service.save_file("test.txt", b"data").await.unwrap();
        assert!(service.file_exists("test.txt").await.unwrap());

        service.delete_file("test.txt").await.unwrap();
        assert!(!service.file_exists("test.txt").await.unwrap());
    }

    #[tokio::test]
    async fn test_list_files() {
        let storage = MockStorage::new();
        let service = FileService::new(storage, "test-bucket".to_string());

        service.save_file("docs/a.txt", b"a").await.unwrap();
        service.save_file("docs/b.txt", b"b").await.unwrap();
        service.save_file("images/c.png", b"c").await.unwrap();

        let docs = service.list_files("docs/").await.unwrap();
        assert_eq!(docs.len(), 2);

        let images = service.list_files("images/").await.unwrap();
        assert_eq!(images.len(), 1);
    }

    #[tokio::test]
    async fn test_with_prepopulated_data() {
        let storage = MockStorage::with_data("bucket", "existing.txt", b"existing content");
        let service = FileService::new(storage, "bucket".to_string());

        let content = service.load_file("existing.txt").await.unwrap();
        assert_eq!(content, b"existing content");
    }
}

// =============================================================================
// Main
// =============================================================================

#[tokio::main]
async fn main() {
    println!("CloudKit Testing Example");
    println!("========================\n");

    // Create mock storage
    let storage = MockStorage::new();
    let service = FileService::new(storage, "my-bucket".to_string());

    // Demonstrate usage
    println!("1. Saving file...");
    service.save_file("hello.txt", b"Hello, CloudKit!").await.unwrap();
    println!("   ✓ Saved hello.txt\n");

    println!("2. Loading file...");
    let content = service.load_file("hello.txt").await.unwrap();
    println!("   ✓ Content: {}\n", String::from_utf8_lossy(&content));

    println!("3. Checking existence...");
    let exists = service.file_exists("hello.txt").await.unwrap();
    println!("   ✓ hello.txt exists: {}\n", exists);

    println!("4. Listing files...");
    service.save_file("docs/readme.md", b"# README").await.unwrap();
    service.save_file("docs/guide.md", b"# Guide").await.unwrap();
    let files = service.list_files("docs/").await.unwrap();
    println!("   ✓ Files in docs/: {:?}\n", files);

    println!("5. Deleting file...");
    service.delete_file("hello.txt").await.unwrap();
    let exists = service.file_exists("hello.txt").await.unwrap();
    println!("   ✓ hello.txt exists after delete: {}\n", exists);

    println!("6. Handling errors...");
    match service.load_file("nonexistent.txt").await {
        Ok(_) => println!("   Unexpected success"),
        Err(CloudError::NotFound { resource_type, resource_id }) => {
            println!("   ✓ Got expected error: {} not found: {}", resource_type, resource_id);
        }
        Err(e) => println!("   Unexpected error: {}", e),
    }

    println!("\n✓ Testing example complete!");
    println!("\nTo run the tests: cargo test --example testing");
}
