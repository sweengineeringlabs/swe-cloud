use crate::storage::StorageEngine;

pub struct EventsService {
    _storage: StorageEngine,
}

impl EventsService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { _storage: storage }
    }
}
