use crate::storage::StorageEngine;

pub struct MonitoringService {
    storage: StorageEngine,
}

impl MonitoringService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { storage }
    }
}
