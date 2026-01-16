use std::sync::Arc;
use oracle_data_core::StorageEngine;
use oracle_control_spi::{CloudProviderTrait, Request, Response, CloudResult};
use async_trait::async_trait;

pub mod services;
use services::pricing::PricingService;

pub struct OracleProvider {
    storage: Arc<StorageEngine>,
    pricing: PricingService,
}

impl OracleProvider {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { 
            storage: storage.clone(),
            pricing: PricingService::new(storage),
        }
    }
}

#[async_trait]
impl CloudProviderTrait for OracleProvider {
    async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // Pricing/Billing API
        if req.path.contains("/metering/api/v1/prices") {
            return self.pricing.handle_request(req).await;
        }

        // Basic routing logic
        if req.path.starts_with("/v1") {
             return Ok(Response::ok(b"Oracle API Root".to_vec()));
        }
        Ok(Response::not_found("Not Found"))
    }
}
