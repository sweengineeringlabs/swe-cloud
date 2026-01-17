
use aws_data_core::StorageEngine;

pub mod handlers;

#[derive(Clone)]
pub struct ElastiCacheService {
    pub storage: StorageEngine,
}

impl ElastiCacheService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { storage }
    }
}
