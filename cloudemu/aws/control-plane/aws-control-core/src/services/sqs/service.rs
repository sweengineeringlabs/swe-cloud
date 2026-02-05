use aws_data_core::storage::StorageEngine;

pub struct SqsService {
    _storage: StorageEngine,
}

impl SqsService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { _storage: storage }
    }
}
