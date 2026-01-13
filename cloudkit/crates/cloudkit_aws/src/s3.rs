use async_trait::async_trait;
use bytes::Bytes;
use cloudkit_api::{GetOptions, ListOptions, ObjectStorage, PutOptions};
use cloudkit_spi::{BucketMetadata, CloudError, CloudResult, ListResult, ObjectMetadata, PaginationToken};
use cloudkit_spi::CloudContext;
use std::sync::Arc;

/// AWS S3 object storage implementation.
pub struct S3Storage {
    _context: Arc<CloudContext>,
    client: aws_sdk_s3::Client,
}

impl S3Storage {
    /// Create a new S3 storage client.
    pub fn new(context: Arc<CloudContext>, sdk_config: aws_config::SdkConfig) -> Self {
        let client = aws_sdk_s3::Client::new(&sdk_config);
        Self { _context: context, client }
    }
}

#[async_trait]
impl ObjectStorage for S3Storage {
    async fn list_buckets(&self) -> CloudResult<Vec<BucketMetadata>> {
        let resp = self.client.list_buckets()
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.buckets().iter().map(|b| {
            BucketMetadata::new(
                b.name().unwrap_or_default(),
                "us-east-1" // Default region for listing
            )
        }).collect())
    }

    async fn create_bucket(&self, bucket: &str) -> CloudResult<()> {
        self.client.create_bucket()
            .bucket(bucket)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn delete_bucket(&self, bucket: &str) -> CloudResult<()> {
        self.client.delete_bucket()
            .bucket(bucket)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn bucket_exists(&self, bucket: &str) -> CloudResult<bool> {
        match self.client.head_bucket().bucket(bucket).send().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn put_object(&self, bucket: &str, key: &str, data: &[u8]) -> CloudResult<()> {
        self.put_object_with_options(bucket, key, data, PutOptions::default()).await
    }

    async fn put_object_with_options(
        &self,
        bucket: &str,
        key: &str,
        data: &[u8],
        options: PutOptions,
    ) -> CloudResult<()> {
        let mut req = self.client.put_object()
            .bucket(bucket)
            .key(key)
            .body(aws_sdk_s3::primitives::ByteStream::from(data.to_vec()));
            
        if let Some(ct) = options.content_type {
            req = req.content_type(ct);
        }
        
        if let Some(cc) = options.cache_control {
            req = req.cache_control(cc);
        }
        
        for (k, v) in options.metadata {
            req = req.metadata(k, v);
        }
        
        req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn get_object(&self, bucket: &str, key: &str) -> CloudResult<Bytes> {
        self.get_object_with_options(bucket, key, GetOptions::default()).await
    }

    async fn get_object_with_options(
        &self,
        bucket: &str,
        key: &str,
        options: GetOptions,
    ) -> CloudResult<Bytes> {
        let mut req = self.client.get_object()
            .bucket(bucket)
            .key(key);
            
        if let (Some(start), Some(end)) = (options.range_start, options.range_end) {
            req = req.range(format!("bytes={}-{}", start, end));
        }
        
        if let Some(etag) = options.if_match {
            req = req.if_match(etag);
        }
        
        if let Some(etag) = options.if_none_match {
            req = req.if_none_match(etag);
        }
        
        let resp = req.send().await.map_err(|e| {
            if e.to_string().contains("NoSuchKey") {
                CloudError::NotFound {
                    resource_type: "Object".to_string(),
                    resource_id: format!("{}/{}", bucket, key),
                }
            } else {
                CloudError::ServiceError(e.to_string())
            }
        })?;
        
        let data = resp.body.collect().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(data.into_bytes())
    }

    async fn head_object(&self, bucket: &str, key: &str) -> CloudResult<ObjectMetadata> {
        let resp = self.client.head_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                if e.to_string().contains("NotFound") || e.to_string().contains("404") {
                    CloudError::NotFound {
                        resource_type: "Object".to_string(),
                        resource_id: format!("{}/{}", bucket, key),
                    }
                } else {
                    CloudError::ServiceError(e.to_string())
                }
            })?;
            
        Ok(ObjectMetadata {
            key: key.to_string(),
            size: resp.content_length().unwrap_or(0) as u64,
            etag: resp.e_tag().map(|e| e.to_string()),
            last_modified: resp.last_modified().map(|d| chrono::DateTime::<chrono::Utc>::from_timestamp(d.secs(), 0).unwrap_or_default()).unwrap_or_default(),
            content_type: resp.content_type().map(|c| c.to_string()),
            storage_class: resp.storage_class().map(|s| s.as_str().to_string()),
            metadata: resp.metadata().unwrap_or(&std::collections::HashMap::new()).clone(),
        })
    }

    async fn delete_object(&self, bucket: &str, key: &str) -> CloudResult<()> {
        self.client.delete_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn delete_objects(&self, bucket: &str, keys: &[&str]) -> CloudResult<()> {
        let mut delete = aws_sdk_s3::types::Delete::builder();
        for key in keys {
            delete = delete.objects(aws_sdk_s3::types::ObjectIdentifier::builder()
                .key(key.to_string())
                .build()
                .unwrap());
        }
        
        self.client.delete_objects()
            .bucket(bucket)
            .delete(delete.build().unwrap())
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn copy_object(
        &self,
        source_bucket: &str,
        source_key: &str,
        dest_bucket: &str,
        dest_key: &str,
    ) -> CloudResult<()> {
        self.client.copy_object()
            .bucket(dest_bucket)
            .key(dest_key)
            .copy_source(format!("{}/{}", source_bucket, source_key))
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn object_exists(&self, bucket: &str, key: &str) -> CloudResult<bool> {
        match self.head_object(bucket, key).await {
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
        let mut req = self.client.list_objects_v2()
            .bucket(bucket);
            
        if let Some(prefix) = options.prefix {
            req = req.prefix(prefix);
        }
        
        if let Some(delim) = options.delimiter {
            req = req.delimiter(delim);
        }
        
        if let Some(max) = options.max_results {
            req = req.max_keys(max as i32);
        }
        
        if let Some(token) = options.continuation_token {
            req = req.continuation_token(token);
        }
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        let objects = resp.contents().iter().map(|o| {
            ObjectMetadata {
                key: o.key().unwrap_or_default().to_string(),
                size: o.size().unwrap_or(0) as u64,
                etag: o.e_tag().map(|e| e.to_string()),
                last_modified: o.last_modified().map(|d| chrono::DateTime::<chrono::Utc>::from_timestamp(d.secs(), 0).unwrap_or_default()).unwrap_or_default(),
                content_type: None,
                storage_class: o.storage_class().map(|s| s.as_str().to_string()),
                metadata: std::collections::HashMap::new(),
            }
        }).collect();
        
        Ok(ListResult::new(
            objects,
            resp.next_continuation_token().map(PaginationToken::some).unwrap_or(PaginationToken::none())
        ))
    }

    async fn presigned_get_url(
        &self,
        bucket: &str,
        key: &str,
        expires_in: std::time::Duration,
    ) -> CloudResult<String> {
        let presigning_config = aws_sdk_s3::presigning::PresigningConfig::builder()
            .expires_in(expires_in)
            .build()
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        let req = self.client.get_object()
            .bucket(bucket)
            .key(key)
            .presigned(presigning_config)
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(req.uri().to_string())
    }

    async fn presigned_put_url(
        &self,
        bucket: &str,
        key: &str,
        expires_in: std::time::Duration,
    ) -> CloudResult<String> {
        let presigning_config = aws_sdk_s3::presigning::PresigningConfig::builder()
            .expires_in(expires_in)
            .build()
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        let req = self.client.put_object()
            .bucket(bucket)
            .key(key)
            .presigned(presigning_config)
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(req.uri().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cloudkit_spi::ProviderType;

    #[tokio::test]
    async fn test_s3_storage_new() {
        let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let context = CloudContext::builder(ProviderType::Aws)
            .build()
            .await
            .unwrap();
        let _storage = S3Storage::new(Arc::new(context), sdk_config);
    }
}

