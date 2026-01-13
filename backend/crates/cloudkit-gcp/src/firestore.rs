//! Google Cloud Firestore implementation.

use async_trait::async_trait;
use cloudkit_spi::api::{Condition, KeyValueStore, KvGetOptions, KvPutOptions, KvQueryOptions};
use cloudkit_spi::common::{CloudError, CloudResult, ListResult, PaginationToken};
use cloudkit_spi::core::CloudContext;
use firestore::{FirestoreDb, FirestoreResult};
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;
use std::sync::Arc;

/// Google Cloud Firestore implementation.
pub struct GcpFirestore {
    context: Arc<CloudContext>,
    client: FirestoreDb,
}

impl GcpFirestore {
    /// Create a new Firestore client.
    pub fn new(context: Arc<CloudContext>, client: FirestoreDb) -> Self {
        Self { context, client }
    }

    fn map_err(e: firestore::errors::FirestoreError) -> CloudError {
        CloudError::Provider {
            provider: "gcp".to_string(),
            code: "FirestoreError".to_string(),
            message: e.to_string(),
        }
    }
}

#[async_trait]
impl KeyValueStore for GcpFirestore {
    async fn get<T: DeserializeOwned + Send>(
        &self,
        table: &str,
        key: &str,
    ) -> CloudResult<Option<T>> {
        self.client
            .fluent()
            .select()
            .by_id_in(table)
            .obj()
            .one(key)
            .await
            .map_err(Self::map_err)
    }

    async fn get_with_options<T: DeserializeOwned + Send>(
        &self,
        table: &str,
        key: &str,
        _options: KvGetOptions,
    ) -> CloudResult<Option<T>> {
        // Options ignored for now
        self.get(table, key).await
    }

    async fn put<T: Serialize + Send + Sync>(
        &self,
        table: &str,
        key: &str,
        item: &T,
    ) -> CloudResult<()> {
        let item_value =
            serde_json::to_value(item).map_err(|e| CloudError::Serialization(e.to_string()))?;
        self.client
            .fluent()
            .insert()
            .into(table)
            .document_id(key)
            .object(&item_value)
            .execute()
            .await
            .map_err(Self::map_err)
    }

    async fn put_with_options<T: Serialize + Send + Sync>(
        &self,
        table: &str,
        key: &str,
        item: &T,
        _options: KvPutOptions,
    ) -> CloudResult<Option<serde_json::Value>> {
        self.put(table, key, item).await?;
        Ok(None)
    }

    async fn delete(&self, table: &str, key: &str) -> CloudResult<()> {
        self.client
            .fluent()
            .delete()
            .from(table)
            .document_id(key)
            .execute()
            .await
            .map_err(Self::map_err)
    }

    async fn delete_with_condition(
        &self,
        table: &str,
        key: &str,
        _condition: Condition,
    ) -> CloudResult<bool> {
        // Conditions not fully supported yet in this binding
        self.delete(table, key).await?;
        Ok(true)
    }

    async fn exists(&self, table: &str, key: &str) -> CloudResult<bool> {
        let result: Option<HashMap<String, serde_json::Value>> = self
            .client
            .fluent()
            .select()
            .by_id_in(table)
            .obj()
            .one(key)
            .await
            .map_err(Self::map_err)?;
        Ok(result.is_some())
    }

    async fn update(
        &self,
        table: &str,
        key: &str,
        updates: HashMap<String, serde_json::Value>,
    ) -> CloudResult<()> {
        // Update specific fields
        // Firestore supports update with merge
        self.client
            .fluent()
            .update()
            .in_col(table)
            .document_id(key)
            .object(&updates)
            .execute()
            .await
            .map_err(Self::map_err)
    }

    async fn query<T: DeserializeOwned + Send>(
        &self,
        table: &str,
        _partition_key: &str,
        _options: KvQueryOptions,
    ) -> CloudResult<ListResult<T>> {
        // Basic scan for now
        let items: Vec<T> = self
            .client
            .fluent()
            .select()
            .from(table)
            .obj()
            .query()
            .await
            .map_err(Self::map_err)?;

        Ok(ListResult::new(items, PaginationToken::none()))
    }

    async fn batch_get<T: DeserializeOwned + Send>(
        &self,
        table: &str,
        keys: &[&str],
    ) -> CloudResult<Vec<T>> {
        // Firestore doesn't have direct batch get in fluent API?
        // It mostly likely does, or we execute in parallel.
        // For now, serial fetch (suboptimal but correct).
        let mut results = Vec::new();
        for key in keys {
            if let Some(item) = self.get(table, key).await? {
                results.push(item);
            }
        }
        Ok(results)
    }

    async fn batch_write<T: Serialize + Send + Sync>(
        &self,
        table: &str,
        items: &[(&str, &T)],
    ) -> CloudResult<()> {
        // Support batch write via loop for v1
        // Firestore has batch API but fluent needs to be checked.
        for (key, item) in items {
            self.put(table, key, *item).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cloudkit_spi::core::ProviderType;
    use serde::{Deserialize, Serialize};

    async fn create_test_context() -> Arc<CloudContext> {
        Arc::new(
            CloudContext::builder(ProviderType::Gcp)
                .build()
                .await
                .unwrap(),
        )
    }

    #[tokio::test]
    #[ignore]
    async fn test_firestore_operations() {
        let context = create_test_context().await;
        let db_client = FirestoreDb::new("test-project").await.unwrap();
        let db = GcpFirestore::new(context, db_client);

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct TestItem {
            id: String,
            value: String,
        }

        let item = TestItem {
            id: "1".to_string(),
            value: "test".to_string(),
        };

        // Basic CRUD
        assert!(db.put("users", "1", &item).await.is_ok());
        assert!(db.exists("users", "1").await.unwrap());
        let fetched = db.get::<TestItem>("users", "1").await.unwrap();
        assert_eq!(fetched.unwrap(), item);
        assert!(db.delete("users", "1").await.is_ok());
    }
}

