use crate::storage::StorageEngine;

pub struct KmsService {
    storage: StorageEngine,
}

impl KmsService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { storage }
    }
}
