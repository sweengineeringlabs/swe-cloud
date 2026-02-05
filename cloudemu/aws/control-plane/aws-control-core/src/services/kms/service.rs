use aws_data_core::storage::StorageEngine;

pub struct KmsService {
    _storage: StorageEngine,
}

impl KmsService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { _storage: storage }
    }
}
