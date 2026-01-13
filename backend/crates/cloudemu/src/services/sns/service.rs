use crate::storage::StorageEngine;

pub struct SnsService {
    _storage: StorageEngine,
}

impl SnsService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { _storage: storage }
    }
}
