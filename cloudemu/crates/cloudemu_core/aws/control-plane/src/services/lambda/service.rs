use data_plane::storage::StorageEngine;

pub struct LambdaService {
    _storage: StorageEngine,
}

impl LambdaService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { _storage: storage }
    }
}
