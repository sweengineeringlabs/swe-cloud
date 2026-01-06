use crate::storage::StorageEngine;

pub struct DynamoDbService {
    storage: StorageEngine,
}

impl DynamoDbService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { storage }
    }
}
