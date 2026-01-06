use crate::storage::StorageEngine;

pub struct SqsService {
    storage: StorageEngine,
}

impl SqsService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { storage }
    }
}
