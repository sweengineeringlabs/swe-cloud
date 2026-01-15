use data_plane::storage::StorageEngine;

pub struct DynamoDbService {
    _storage: StorageEngine,
}

impl DynamoDbService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { _storage: storage }
    }
}
