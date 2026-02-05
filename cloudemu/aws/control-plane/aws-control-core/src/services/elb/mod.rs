
use aws_data_core::StorageEngine;

pub mod handlers;

#[derive(Clone)]
pub struct ElbService {
    pub storage: StorageEngine,
}

impl ElbService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { storage }
    }
}
