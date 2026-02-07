use gcp_control_spi::{Request, Response, CloudResult, CloudError};
use gcp_data_core::storage::StorageEngine;
use std::sync::Arc;

/// GCP Cloud Storage Handler
pub struct CloudStorageService {
    engine: Arc<StorageEngine>,
}

impl CloudStorageService {
    pub fn new(engine: Arc<StorageEngine>) -> Self {
        Self { engine }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        let path = req.path.trim_start_matches('/');
        let parts: Vec<&str> = path.split('/').collect();

        if parts.is_empty() {
            return Ok(Response::ok("Cloud Storage Emulator"));
        }

        // GCS paths: /{bucket} or /{bucket}/{object}
        let bucket = parts[0];

        if parts.len() == 1 {
            match req.method.as_str() {
                "PUT" => return self.create_bucket(bucket).await,
                "GET" => return self.get_bucket(bucket).await,
                "DELETE" => return self.delete_bucket(bucket).await,
                _ => {}
            }
        } else {
            // Object operations
            let object_key = parts[1..].join("/");
            match req.method.as_str() {
                "PUT" => return self.put_object(bucket, &object_key, &req.body).await,
                "GET" => return self.get_object(bucket, &object_key).await,
                "DELETE" => return self.delete_object(bucket, &object_key).await,
                _ => {}
            }
        }

        Err(CloudError::Validation(format!("Unsupported Cloud Storage operation: {} {}", req.method, req.path)))
    }

    async fn create_bucket(&self, name: &str) -> CloudResult<Response> {
        self.engine.create_gcs_bucket(name, "gcp", "local")
            .map_err(|e| CloudError::Internal(e.to_string()))?;
        Ok(Response::created(""))
    }

    async fn get_bucket(&self, name: &str) -> CloudResult<Response> {
        let _bucket = self.engine.get_gcs_bucket(name)
            .map_err(|_| CloudError::NotFound { resource_type: "Bucket".into(), resource_id: name.into() })?;
        Ok(Response::ok(format!(r#"{{"name":"{}"}}"#, name)))
    }

    async fn delete_bucket(&self, name: &str) -> CloudResult<Response> {
        self.engine.delete_gcs_bucket(name)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
        Ok(Response::no_content())
    }

    async fn put_object(&self, bucket: &str, key: &str, body: &[u8]) -> CloudResult<Response> {
        self.engine.insert_gcs_object(bucket, key, body, Some("application/octet-stream"))
            .map_err(|e| CloudError::Internal(e.to_string()))?;
        Ok(Response::created(""))
    }

    async fn get_object(&self, bucket: &str, key: &str) -> CloudResult<Response> {
        let (_metadata, data) = self.engine.get_gcs_object(bucket, key, None)
            .map_err(|_| CloudError::NotFound { resource_type: "Object".into(), resource_id: key.into() })?;
        Ok(Response::ok(data))
    }

    async fn delete_object(&self, bucket: &str, key: &str) -> CloudResult<Response> {
        self.engine.delete_gcs_object(bucket, key)
            .map_err(|e| CloudError::Internal(e.to_string()))?;
        Ok(Response::no_content())
    }
}
