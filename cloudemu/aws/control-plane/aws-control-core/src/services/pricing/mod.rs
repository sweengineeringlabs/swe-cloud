use aws_data_core::StorageEngine;

pub mod handlers;

/// Pricing Service implementation
#[derive(Clone)]
pub struct PricingService {
    storage: StorageEngine,
}

impl PricingService {
    /// Create a new PricingService
    pub fn new(storage: StorageEngine) -> Self {
        Self { storage }
    }
}
