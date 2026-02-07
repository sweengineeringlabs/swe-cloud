use aws_data_core::storage::StorageEngine;

pub struct WorkflowsService {
    _storage: StorageEngine,
}

impl WorkflowsService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { _storage: storage }
    }
}
