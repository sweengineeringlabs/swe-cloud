use aws_data_core::storage::StorageEngine;

pub struct IdentityService {
    _storage: StorageEngine,
}

impl IdentityService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { _storage: storage }
    }
}
