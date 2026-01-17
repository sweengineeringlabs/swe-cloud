use zero_control_spi::{ZeroResult, ZeroError};
use zero_data_core::ZeroEngine;
use std::sync::Arc;
use serde_json::json;

pub struct StoreService {
    engine: Arc<ZeroEngine>,
}

impl StoreService {
    pub fn new(engine: Arc<ZeroEngine>) -> Self {
        Self { engine }
    }

    pub async fn list_buckets(&self) -> ZeroResult<Vec<String>> {
        // We map ZeroCloud "Volumes" to S3 Buckets for this native implementation
        let volumes = self.engine.storage.list_volumes().await?;
        let bucket_names = volumes.into_iter()
            .map(|v| v.id)
            .collect();
        Ok(bucket_names)
    }

    pub async fn create_bucket(&self, name: &str) -> ZeroResult<()> {
        // Map bucket to a Volume (Directory)
        // We use a small default size since it's just a folder
        self.engine.storage.create_volume(name, 1).await?;
        Ok(())
    }
}
