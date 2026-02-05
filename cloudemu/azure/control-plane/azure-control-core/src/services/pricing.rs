use azure_control_spi::{CloudResult, Request, Response};
use azure_data_core::storage::{StorageEngine, Product, OfferTerm};
use std::sync::Arc;
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
            "GET" => self.get_prices(req).await,
            _ => Ok(Response::not_found("Not Found")),
        }
    }

    async fn get_prices(&self, req: Request) -> CloudResult<Response> {
        // Seed data on first access
        let _ = self.storage.seed_pricing_data().await;

        // ... (existing logic)

        let products = self.storage.get_products("Virtual Machines", |_| true).await.unwrap_or_default();
        
        let mut items = Vec::new();
        for (product, terms) in products {
             for term in terms {
                 let retail_price = term.price_dimensions["retailPrice"].as_f64().unwrap_or(0.0);
                 let currency = term.price_dimensions["currencyCode"].as_str().unwrap_or("USD");
                 let unit = term.price_dimensions["unitOfMeasure"].as_str().unwrap_or("1 Hour");
                 
                 let mut item = product.attributes.clone();
                 if let Some(obj) = item.as_object_mut() {
                     obj.insert("retailPrice".to_string(), json!(retail_price));
                     obj.insert("currencyCode".to_string(), json!(currency));
                     obj.insert("unitOfMeasure".to_string(), json!(unit));
                     obj.insert("offerTermCode".to_string(), json!(term.offer_term_code));
                     obj.insert("effectiveDate".to_string(), json!(term.effective_date));
                 }
                 items.push(item);
             }
        }

        let body = json!({
            "Items": items,
            "NextPageLink": null,
            "Count": items.len()
        });

        // Use standard Response::ok
        Ok(Response::ok(serde_json::to_vec(&body).unwrap()))
    }
}
