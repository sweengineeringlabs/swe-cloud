use cloudkit_api::{KeyValueStore, KvGetOptions, KvPutOptions, KvQueryOptions, Condition};
use cloudkit_spi::{CloudResult, ListResult, CloudError};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use zero_sdk::ZeroClient;

pub struct ZeroDb {
    client: ZeroClient,
}

impl ZeroDb {
    pub fn new(client: ZeroClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl KeyValueStore for ZeroDb {
    async fn get<T: DeserializeOwned + Send>(
        &self,
        table: &str,
        key: &str,
    ) -> CloudResult<Option<T>> {
        self.get_with_options(table, key, KvGetOptions::default()).await
    }

    async fn get_with_options<T: DeserializeOwned + Send>(
        &self,
        _table: &str,
        _key: &str,
        _options: KvGetOptions,
    ) -> CloudResult<Option<T>> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn put<T: Serialize + Send + Sync>(
        &self,
        table: &str,
        key: &str,
        item: &T,
    ) -> CloudResult<()> {
        let value = serde_json::to_value(item).map_err(|e| CloudError::Internal(e.to_string()))?;
        self.client.db().put_item(table, key, value).await
            .map_err(|e| CloudError::Internal(e.to_string()))?;
        Ok(())
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

    async fn delete(&self, _table: &str, _key: &str) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn delete_with_condition(
        &self,
        _table: &str,
        _key: &str,
        _condition: Condition,
    ) -> CloudResult<bool> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn exists(&self, _table: &str, _key: &str) -> CloudResult<bool> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn update(
        &self,
        _table: &str,
        _key: &str,
        _updates: HashMap<String, serde_json::Value>,
    ) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn query<T: DeserializeOwned + Send>(
        &self,
        _table: &str,
        _partition_key: &str,
        _options: KvQueryOptions,
    ) -> CloudResult<ListResult<T>> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn batch_get<T: DeserializeOwned + Send>(
        &self,
        _table: &str,
        _keys: &[&str],
    ) -> CloudResult<Vec<T>> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn batch_write<T: Serialize + Send + Sync>(
        &self,
        _table: &str,
        _items: &[(&str, &T)],
    ) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }
}
