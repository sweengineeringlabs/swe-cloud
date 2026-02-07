//! DynamoDB service traits.

use async_trait::async_trait;
use aws_control_spi::CloudResult;
use serde_json::Value;

/// DynamoDB table service trait.
#[async_trait]
pub trait DynamoDbService: Send + Sync {
    /// Create a table.
    async fn create_table(&self, input: Value) -> CloudResult<Value>;

    /// Describe a table.
    async fn describe_table(&self, table_name: &str) -> CloudResult<Value>;

    /// List tables.
    async fn list_tables(&self) -> CloudResult<Value>;

    /// Put an item.
    async fn put_item(&self, table_name: &str, item: Value) -> CloudResult<()>;

    /// Get an item.
    async fn get_item(&self, table_name: &str, key: Value) -> CloudResult<Option<Value>>;

    /// Query items.
    async fn query(&self, input: Value) -> CloudResult<Value>;

    /// Scan items.
    async fn scan(&self, input: Value) -> CloudResult<Value>;
}
