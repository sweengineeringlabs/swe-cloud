use crate::storage::StorageEngine;

pub struct SecretsService {
    storage: StorageEngine,
}

impl SecretsService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { storage }
    }
}
