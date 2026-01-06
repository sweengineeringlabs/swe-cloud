use crate::storage::StorageEngine;

pub struct EventsService {
    storage: StorageEngine,
}

impl EventsService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { storage }
    }
}
