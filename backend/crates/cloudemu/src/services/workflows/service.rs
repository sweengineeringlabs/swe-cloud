use crate::storage::StorageEngine;

pub struct WorkflowsService {
    storage: StorageEngine,
}

impl WorkflowsService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { storage }
    }
}
