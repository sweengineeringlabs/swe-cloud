use aws_data_core::storage::StorageEngine;

pub struct LambdaService {
    _storage: StorageEngine,
}

impl LambdaService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { _storage: storage }
    }
}
