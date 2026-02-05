use aws_data_core::storage::StorageEngine;

pub struct SnsService {
    _storage: StorageEngine,
}

impl SnsService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { _storage: storage }
    }
}
