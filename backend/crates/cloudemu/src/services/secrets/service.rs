use crate::storage::StorageEngine;

pub struct SecretsService {
    _storage: StorageEngine,
}

impl SecretsService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { _storage: storage }
    }
}
