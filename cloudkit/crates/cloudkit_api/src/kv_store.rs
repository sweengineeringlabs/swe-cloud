//! Key-value store trait for NoSQL operations.

use cloudkit_spi::{CloudResult, ListResult};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

/// Condition for conditional operations.
#[derive(Debug, Clone)]
pub enum Condition {
    /// Attribute exists
    Exists(String),
    /// Attribute does not exist
    NotExists(String),
    /// Attribute equals value
    Equals(String, serde_json::Value),
    /// Attribute not equals value
    NotEquals(String, serde_json::Value),
    /// Attribute less than value
    LessThan(String, serde_json::Value),
    /// Attribute less than or equal to value
    LessThanOrEqual(String, serde_json::Value),
    /// Attribute greater than value
    GreaterThan(String, serde_json::Value),
    /// Attribute greater than or equal to value
    GreaterThanOrEqual(String, serde_json::Value),
    /// Attribute is in a list of values
    In(String, Vec<serde_json::Value>),
    /// Attribute is between two values
    Between(String, serde_json::Value, serde_json::Value),
    /// Attribute begins with prefix
    BeginsWith(String, String),
    /// Attribute contains substring
    Contains(String, String),
    /// Logical NOT of a condition
    Not(Box<Condition>),
    /// Logical AND of multiple conditions
    And(Vec<Condition>),
    /// Logical OR of multiple conditions
    Or(Vec<Condition>),
}

/// Options for get operations.
#[derive(Debug, Clone, Default)]
pub struct KvGetOptions {
    /// Consistent read
    pub consistent_read: bool,
    /// Projection (only return these attributes)
    pub projection: Option<Vec<String>>,
}

/// Options for put operations.
#[derive(Debug, Clone, Default)]
pub struct KvPutOptions {
    /// Condition for conditional put
    pub condition: Option<Condition>,
    /// Return old value
    pub return_old: bool,
}

/// Options for query operations.
#[derive(Debug, Clone, Default)]
pub struct KvQueryOptions {
    /// Filter expression
    pub filter: Option<Condition>,
    /// Maximum results
    pub limit: Option<u32>,
    /// Scan forward
    pub scan_forward: bool,
    /// Consistent read
    pub consistent_read: bool,
    /// Continuation token
    pub continuation_token: Option<String>,
}

/// Key-value store service trait.
///
/// This trait abstracts NoSQL key-value operations across cloud providers:
/// - AWS DynamoDB
/// - Azure Cosmos DB
/// - Google Firestore
/// - Oracle NoSQL
///
/// # Example
///
/// ```rust,ignore
/// use cloudkit::api::KeyValueStore;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct User {
///     id: String,
///     name: String,
/// }
///
/// async fn save_user<K: KeyValueStore>(store: &K, user: &User) -> CloudResult<()> {
///     store.put("users", &user.id, user).await
/// }
/// ```
#[async_trait]
pub trait KeyValueStore: Send + Sync {
    /// Get an item by key.
    async fn get<T: DeserializeOwned + Send>(
        &self,
        table: &str,
        key: &str,
    ) -> CloudResult<Option<T>>;

    /// Get an item with options.
    async fn get_with_options<T: DeserializeOwned + Send>(
        &self,
        table: &str,
        key: &str,
        options: KvGetOptions,
    ) -> CloudResult<Option<T>>;

    /// Put an item.
    async fn put<T: Serialize + Send + Sync>(
        &self,
        table: &str,
        key: &str,
        item: &T,
    ) -> CloudResult<()>;

    /// Put an item with options.
    async fn put_with_options<T: Serialize + Send + Sync>(
        &self,
        table: &str,
        key: &str,
        item: &T,
        options: KvPutOptions,
    ) -> CloudResult<Option<serde_json::Value>>;

    /// Delete an item.
    async fn delete(&self, table: &str, key: &str) -> CloudResult<()>;

    /// Delete an item with condition.
    async fn delete_with_condition(
        &self,
        table: &str,
        key: &str,
        condition: Condition,
    ) -> CloudResult<bool>;

    /// Check if an item exists.
    async fn exists(&self, table: &str, key: &str) -> CloudResult<bool>;

    /// Update specific attributes.
    async fn update(
        &self,
        table: &str,
        key: &str,
        updates: HashMap<String, serde_json::Value>,
    ) -> CloudResult<()>;

    /// Query items by partition key.
    async fn query<T: DeserializeOwned + Send>(
        &self,
        table: &str,
        partition_key: &str,
        options: KvQueryOptions,
    ) -> CloudResult<ListResult<T>>;

    /// Batch get items.
    async fn batch_get<T: DeserializeOwned + Send>(
        &self,
        table: &str,
        keys: &[&str],
    ) -> CloudResult<Vec<T>>;

    /// Batch write items.
    async fn batch_write<T: Serialize + Send + Sync>(
        &self,
        table: &str,
        items: &[(&str, &T)],
    ) -> CloudResult<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_equals() {
        let condition = Condition::Equals("status".to_string(), serde_json::json!("active"));
        match condition {
            Condition::Equals(attr, val) => {
                assert_eq!(attr, "status");
                assert_eq!(val, serde_json::json!("active"));
            }
            _ => panic!("Expected Equals condition"),
        }
    }
}

