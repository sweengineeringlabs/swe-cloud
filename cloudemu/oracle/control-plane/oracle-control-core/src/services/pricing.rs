use std::sync::Arc;
use oracle_data_core::StorageEngine;
use oracle_control_spi::{CloudResult, Request, Response};
use serde_json::json;

pub struct PricingService {
    storage: Arc<StorageEngine>,
}

impl PricingService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        match req.method.as_str() {
            "GET" => self.list_prices(req).await,
            _ => Ok(Response::not_found("Not Found")),
        }
    }

    async fn list_prices(&self, _req: Request) -> CloudResult<Response> {
        // Seed first
        let _ = self.storage.seed_pricing_data().await;

        // Fetch all Compute prices for now (simulating a list call)
        // In a real implementation we would parse query params for filters
        let products = self.storage.get_products("Compute").await.unwrap_or_default();
        let storage_products = self.storage.get_products("Object Storage").await.unwrap_or_default();
        
        // Combine results
        let all_products = products.into_iter().chain(storage_products.into_iter());

        let mut items = Vec::new();

        for (product, terms) in all_products {
             for term in terms {
                 // Map to a generic OCI-like Structure
                 let mut item = json!({
                     "partNumber": product.sku,
                     "service": product.service_code,
                     "attributes": product.attributes,
                     "price": term.price_dimensions
                 });
                 items.push(item);
             }
        }

        let body = json!({
            "items": items
        });

        Ok(Response::ok(serde_json::to_vec(&body).unwrap()))
    }
}
