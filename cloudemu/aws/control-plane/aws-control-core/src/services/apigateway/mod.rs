
use aws_data_core::StorageEngine;

pub mod handlers;

#[derive(Clone)]
pub struct ApiGatewayService {
    pub storage: StorageEngine,
}

impl ApiGatewayService {
    pub fn new(storage: StorageEngine) -> Self {
        Self { storage }
    }
}
