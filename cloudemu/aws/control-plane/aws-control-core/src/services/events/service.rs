use aws_data_core::storage::StorageEngine;

pub struct EventsService {
    _storage: StorageEngine,
}

impl EventsService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { _storage: storage }
    }
}
