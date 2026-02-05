use aws_data_core::storage::StorageEngine;

pub struct MonitoringService {
    _storage: StorageEngine,
}

impl MonitoringService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { _storage: storage }
    }
}
