
use aws_data_core::StorageEngine;

pub mod handlers;

#[derive(Clone)]
pub struct EcrService {
    pub storage: StorageEngine,
}

impl EcrService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { storage }
    }
}
