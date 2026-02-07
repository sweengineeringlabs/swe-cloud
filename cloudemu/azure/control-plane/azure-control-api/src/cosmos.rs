//! Azure Cosmos DB service traits.

use async_trait::async_trait;
use azure_control_spi::CloudResult;
use serde_json::Value;

/// Azure Cosmos DB service trait.
#[async_trait]
pub trait CosmosService: Send + Sync {
    /// Create a database.
    async fn create_database(&self, database_id: &str) -> CloudResult<Value>;

    /// Create a container (collection).
    async fn create_container(&self, database_id: &str, container_id: &str, partition_key: &str) -> CloudResult<Value>;

    /// Create an item.
    async fn create_item(&self, database_id: &str, container_id: &str, item: Value) -> CloudResult<Value>;

    /// Read an item.
    async fn read_item(&self, database_id: &str, container_id: &str, item_id: &str) -> CloudResult<Option<Value>>;

    /// Query items.
    async fn query_items(&self, database_id: &str, container_id: &str, query: &str) -> CloudResult<Vec<Value>>;
}
