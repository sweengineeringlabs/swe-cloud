use crate::storage::StorageEngine;

pub struct IdentityService {
    storage: StorageEngine,
}

impl IdentityService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { storage }
    }
}
