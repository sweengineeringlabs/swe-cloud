//! S3 service traits

use async_trait::async_trait;

/// S3 storage service
#[async_trait]
pub trait S3Service {
    /// Put object
    async fn put_object(&self, bucket: &str, key: &str, data: Vec<u8>) -> Result<(), String>;
    
    /// Get object
    async fn get_object(&self, bucket: &str, key: &str) -> Result<Vec<u8>, String>;
}
