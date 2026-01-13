use data_plane::storage::StorageEngine;

pub struct SqsService {
    _storage: StorageEngine,
}

impl SqsService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { _storage: storage }
    }
}
