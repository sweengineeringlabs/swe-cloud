use cloudkit_api::{ObjectStorage, PutOptions, GetOptions, ListOptions};
use cloudkit_spi::{BucketMetadata, CloudResult, ListResult, ObjectMetadata, CloudError};
use async_trait::async_trait;
use bytes::Bytes;
use zero_sdk::ZeroClient;

pub struct ZeroStore {
    client: ZeroClient,
}

impl ZeroStore {
    pub fn new(client: ZeroClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ObjectStorage for ZeroStore {
    async fn list_buckets(&self) -> CloudResult<Vec<BucketMetadata>> {
        let buckets = self.client.store().list_buckets().await
            .map_err(|e| CloudError::Internal(e.to_string()))?;
        
        Ok(buckets.into_iter()
            .map(|name| BucketMetadata::new(name, "zero-local"))
            .collect())
    }

    async fn create_bucket(&self, bucket: &str) -> CloudResult<()> {
        self.client.store().create_bucket(bucket).await
            .map_err(|e| CloudError::Internal(e.to_string()))
    }

    async fn delete_bucket(&self, _bucket: &str) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn bucket_exists(&self, bucket: &str) -> CloudResult<bool> {
        let buckets = self.list_buckets().await?;
        Ok(buckets.iter().any(|b| b.name == bucket))
    }

    async fn put_object(&self, bucket: &str, key: &str, data: &[u8]) -> CloudResult<()> {
        self.put_object_with_options(bucket, key, data, PutOptions::default()).await
    }

    async fn put_object_with_options(
        &self,
        _bucket: &str,
        _key: &str,
        _data: &[u8],
        _options: PutOptions,
    ) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn get_object(&self, bucket: &str, key: &str) -> CloudResult<Bytes> {
        self.get_object_with_options(bucket, key, GetOptions::default()).await
    }

    async fn get_object_with_options(
        &self,
        _bucket: &str,
        _key: &str,
        _options: GetOptions,
    ) -> CloudResult<Bytes> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn head_object(&self, _bucket: &str, _key: &str) -> CloudResult<ObjectMetadata> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn delete_object(&self, _bucket: &str, _key: &str) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn delete_objects(&self, _bucket: &str, _keys: &[&str]) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn copy_object(
        &self,
        _source_bucket: &str,
        _source_key: &str,
        _dest_bucket: &str,
        _dest_key: &str,
    ) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn object_exists(&self, _bucket: &str, _key: &str) -> CloudResult<bool> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn list_objects(
        &self,
        _bucket: &str,
        _options: ListOptions,
    ) -> CloudResult<ListResult<ObjectMetadata>> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn presigned_get_url(
        &self,
        _bucket: &str,
        _key: &str,
        _expires_in: std::time::Duration,
    ) -> CloudResult<String> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn presigned_put_url(
        &self,
        _bucket: &str,
        _key: &str,
        _expires_in: std::time::Duration,
    ) -> CloudResult<String> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }
}
