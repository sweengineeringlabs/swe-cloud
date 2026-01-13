//! Google Cloud Storage implementation.

use async_trait::async_trait;
use bytes::Bytes;
use cloudkit_spi::api::{GetOptions, ListOptions, ObjectStorage, PutOptions};
use cloudkit_spi::common::{
    BucketMetadata, CloudError, CloudResult, ListResult, ObjectMetadata, PaginationToken,
};
use cloudkit_spi::core::CloudContext;
use google_cloud_storage::client::Client;
use google_cloud_storage::http::buckets::delete::DeleteBucketRequest;
use google_cloud_storage::http::buckets::insert::{
    BucketCreationConfig, InsertBucketParam, InsertBucketRequest,
};
use google_cloud_storage::http::buckets::list::ListBucketsRequest;
use google_cloud_storage::http::objects::delete::DeleteObjectRequest;
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use google_cloud_storage::http::objects::list::ListObjectsRequest;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use google_cloud_storage::sign::SignedURLMethod;
use google_cloud_storage::sign::SignedURLOptions;
use std::sync::Arc;
use std::time::Duration;

/// Google Cloud Storage implementation.
pub struct GcsStorage {
    context: Arc<CloudContext>,
    client: Client,
    project_id: String,
}

impl GcsStorage {
    /// Create a new GCS storage client.
    pub fn new(context: Arc<CloudContext>, client: Client, project_id: String) -> Self {
        Self {
            context,
            client,
            project_id,
        }
    }

    fn map_err(e: google_cloud_storage::http::Error) -> CloudError {
        CloudError::Provider {
            provider: "gcp".to_string(),
            code: "GcsError".to_string(),
            message: e.to_string(),
        }
    }

    fn to_utc(ts: time::OffsetDateTime) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp(ts.unix_timestamp(), ts.nanosecond()).unwrap_or_default()
    }
}

#[async_trait]
impl ObjectStorage for GcsStorage {
    async fn list_buckets(&self) -> CloudResult<Vec<BucketMetadata>> {
        tracing::debug!(provider = "gcp", service = "gcs", "list_buckets called");
        let result = self
            .client
            .list_buckets(&ListBucketsRequest {
                project: self.project_id.clone(),
                ..Default::default()
            })
            .await
            .map_err(Self::map_err)?;

        let buckets = result
            .items
            .into_iter()
            .map(|b| BucketMetadata {
                name: b.name,
                created_at: Self::to_utc(b.time_created.unwrap_or(time::OffsetDateTime::now_utc())),
                region: b.location,
                versioning_enabled: b.versioning.map(|v| v.enabled).unwrap_or(false),
            })
            .collect();

        Ok(buckets)
    }

    async fn create_bucket(&self, bucket: &str) -> CloudResult<()> {
        let config = BucketCreationConfig {
            location: self.context.region().to_string(),
            ..Default::default()
        };

        self.client
            .insert_bucket(&InsertBucketRequest {
                name: bucket.to_string(),
                param: InsertBucketParam {
                    project: self.project_id.clone(),
                    ..Default::default()
                },
                bucket: config,
            })
            .await
            .map_err(Self::map_err)?;

        Ok(())
    }

    async fn delete_bucket(&self, bucket: &str) -> CloudResult<()> {
        self.client
            .delete_bucket(&DeleteBucketRequest {
                bucket: bucket.to_string(),
                ..Default::default()
            })
            .await
            .map_err(Self::map_err)
    }

    async fn bucket_exists(&self, bucket: &str) -> CloudResult<bool> {
        match self
            .client
            .get_bucket(
                &google_cloud_storage::http::buckets::get::GetBucketRequest {
                    bucket: bucket.to_string(),
                    ..Default::default()
                },
            )
            .await
        {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn put_object(&self, bucket: &str, key: &str, data: &[u8]) -> CloudResult<()> {
        self.put_object_with_options(bucket, key, data, PutOptions::default())
            .await
    }

    async fn put_object_with_options(
        &self,
        bucket: &str,
        key: &str,
        data: &[u8],
        _options: PutOptions,
    ) -> CloudResult<()> {
        let upload_type = UploadType::Simple(Media {
            name: key.to_string().into(),
            content_type: "application/octet-stream".into(),
            content_length: Some(data.len() as u64),
        });

        self.client
            .upload_object(
                &UploadObjectRequest {
                    bucket: bucket.to_string(),
                    ..Default::default()
                },
                data.to_vec(),
                &upload_type,
            )
            .await
            .map(|_| ())
            .map_err(Self::map_err)
    }

    async fn get_object(&self, bucket: &str, key: &str) -> CloudResult<Bytes> {
        self.get_object_with_options(bucket, key, GetOptions::default())
            .await
    }

    async fn get_object_with_options(
        &self,
        bucket: &str,
        key: &str,
        _options: GetOptions,
    ) -> CloudResult<Bytes> {
        let result = self
            .client
            .download_object(
                &GetObjectRequest {
                    bucket: bucket.to_string(),
                    object: key.to_string(),
                    ..Default::default()
                },
                &Range::default(),
            )
            .await
            .map_err(Self::map_err)?;

        Ok(Bytes::from(result))
    }

    async fn head_object(&self, bucket: &str, key: &str) -> CloudResult<ObjectMetadata> {
        let obj = self
            .client
            .get_object(&GetObjectRequest {
                bucket: bucket.to_string(),
                object: key.to_string(),
                ..Default::default()
            })
            .await
            .map_err(Self::map_err)?;

        Ok(ObjectMetadata {
            key: obj.name,
            size: obj.size as u64,
            last_modified: {
                let ts = obj.updated.unwrap_or(time::OffsetDateTime::now_utc());
                chrono::DateTime::from_timestamp(ts.unix_timestamp(), ts.nanosecond())
                    .unwrap_or_default()
            },
            etag: Some(obj.etag),
            content_type: obj.content_type,
            storage_class: obj.storage_class,
            metadata: std::collections::HashMap::new(),
        })
    }

    async fn delete_object(&self, bucket: &str, key: &str) -> CloudResult<()> {
        self.client
            .delete_object(&DeleteObjectRequest {
                bucket: bucket.to_string(),
                object: key.to_string(),
                ..Default::default()
            })
            .await
            .map_err(Self::map_err)
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
        self.client
            .rewrite_object(
                &google_cloud_storage::http::objects::rewrite::RewriteObjectRequest {
                    destination_bucket: dest_bucket.to_string(),
                    destination_object: dest_key.to_string(),
                    source_bucket: source_bucket.to_string(),
                    source_object: source_key.to_string(),
                    ..Default::default()
                },
            )
            .await
            .map(|_| ())
            .map_err(Self::map_err)
    }

    async fn object_exists(&self, bucket: &str, key: &str) -> CloudResult<bool> {
        match self.head_object(bucket, key).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn list_objects(
        &self,
        bucket: &str,
        options: ListOptions,
    ) -> CloudResult<ListResult<ObjectMetadata>> {
        let result = self
            .client
            .list_objects(&ListObjectsRequest {
                bucket: bucket.to_string(),
                prefix: options.prefix,
                page_token: options.continuation_token,
                max_results: options.max_results.map(|l| l as i32),
                ..Default::default()
            })
            .await
            .map_err(Self::map_err)?;

        let items = result
            .items
            .unwrap_or_default()
            .into_iter()
            .map(|o| ObjectMetadata {
                key: o.name,
                size: o.size as u64,
                last_modified: {
                    let ts = o.updated.unwrap_or(time::OffsetDateTime::now_utc());
                    chrono::DateTime::from_timestamp(ts.unix_timestamp(), ts.nanosecond())
                        .unwrap_or_default()
                },
                etag: Some(o.etag),
                content_type: o.content_type,
                storage_class: o.storage_class,
                metadata: std::collections::HashMap::new(),
            })
            .collect();

        Ok(ListResult::new(
            items,
            result
                .next_page_token
                .map(PaginationToken::some)
                .unwrap_or_else(PaginationToken::none),
        ))
    }

    async fn presigned_get_url(
        &self,
        bucket: &str,
        key: &str,
        expires_in: Duration,
    ) -> CloudResult<String> {
        let opts = SignedURLOptions {
            method: SignedURLMethod::GET,
            expires: expires_in,
            ..Default::default()
        };
        self.client
            .signed_url(bucket, key, None, None, opts)
            .await
            .map_err(|e| CloudError::Provider {
                provider: "gcp".to_string(),
                code: "SignedUrlError".to_string(),
                message: e.to_string(),
            })
    }

    async fn presigned_put_url(
        &self,
        bucket: &str,
        key: &str,
        expires_in: Duration,
    ) -> CloudResult<String> {
        let opts = SignedURLOptions {
            method: SignedURLMethod::PUT,
            expires: expires_in,
            ..Default::default()
        };
        self.client
            .signed_url(bucket, key, None, None, opts)
            .await
            .map_err(|e| CloudError::Provider {
                provider: "gcp".to_string(),
                code: "SignedUrlError".to_string(),
                message: e.to_string(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cloudkit_spi::core::ProviderType;

    // Helper to generic test client if credentials exist
    async fn create_client(context: Arc<CloudContext>) -> Option<Client> {
        use google_cloud_storage::client::ClientConfig;
        match ClientConfig::default().with_auth().await {
            Ok(config) => Some(Client::new(config)),
            Err(_) => None,
        }
    }

    #[tokio::test]
    #[ignore] // Integration test - requires creds
    async fn test_gcs_operations() {
        let context = Arc::new(
            CloudContext::builder(ProviderType::Gcp)
                .build()
                .await
                .unwrap(),
        );

        if let Some(client) = create_client(context.clone()).await {
            let storage = GcsStorage::new(context, client, "test-project".to_string());
            let _ = storage.list_buckets().await;
        }
    }
}

