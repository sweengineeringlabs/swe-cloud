//! AWS data storage traits.

use async_trait::async_trait;
use aws_data_spi::CloudResult;
use serde_json::Value;

/// AWS data storage operations.
#[async_trait]
pub trait DataStorageService: Send + Sync {
    /// Store a service-specific record.
    async fn store_record(&self, service: &str, id: &str, data: Value) -> CloudResult<()>;

    /// Retrieve a service-specific record.
    async fn get_record(&self, service: &str, id: &str) -> CloudResult<Option<Value>>;

    /// Delete a service-specific record.
    async fn delete_record(&self, service: &str, id: &str) -> CloudResult<()>;

    /// List records for a service.
    async fn list_records(&self, service: &str, prefix: Option<&str>) -> CloudResult<Vec<Value>>;
}
