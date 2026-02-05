use gcp_control_spi::{CloudResult, Request, Response};
use gcp_data_core::storage::{StorageEngine};
use std::sync::Arc;
use serde_json::json;

pub struct CloudBillingService {
    storage: Arc<StorageEngine>,
}

impl CloudBillingService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        let path = req.path;
        match (req.method.as_str(), path.as_str()) {
            ("GET", p) if p.ends_with("/services") => self.list_services().await,
            // Add skus listing if needed: /v1/services/{service_id}/skus
            ("GET", p) if p.contains("/skus") => self.list_skus().await,
            _ => Ok(Response::not_found("Not Found")),
        }
    }

    async fn list_services(&self) -> CloudResult<Response> {
        // Seed on first access
        let _ = self.storage.seed_pricing_data().await;
        
        // Mock response for services listing
        // In reality, we should query DB, but our DB stores products (SKUs), not service definitions separately yet.
        // We'll return a static list or one derived from products.
        let body = json!({
            "services": [
                {
                    "name": "services/6F81-5844-456A",
                    "serviceId": "6F81-5844-456A",
                    "displayName": "Compute Engine",
                    "businessEntityName": "Google Cloud Platform"
                }
            ],
            "nextPageToken": ""
        });
        
        Ok(Response::ok(serde_json::to_vec(&body).unwrap()))
    }

    async fn list_skus(&self) -> CloudResult<Response> {
        // Seed on first access (important for tests jumping straight to skus)
        let _ = self.storage.seed_pricing_data().await;

        // Assume filtering for Compute Engine for now
         let products = self.storage.get_products("Compute Engine", |_| true).await.unwrap_or_default();
         
         let mut skus = Vec::new();
         for (product, terms) in products {
             // Map to GCP Billing API SKU format
             let mut sku_json = product.attributes.clone();
             
             // Add pricing info
             if let Some(term) = terms.first() {
                 if let Some(obj) = sku_json.as_object_mut() {
                     obj.insert("skuId".to_string(), json!(product.sku)); // Matches seeding "C028-2F74-78E6"
                     obj.insert("description".to_string(), json!(term.description));
                     
                     // Pricing Info
                     let unit_price = term.price_dimensions["units"].as_f64().unwrap_or(0.0);
                     let nanos = term.price_dimensions["nanos"].as_i64().unwrap_or(0);
                     let currency = term.price_dimensions["currencyCode"].as_str().unwrap_or("USD");
                     
                     obj.insert("pricingInfo".to_string(), json!([{
                         "summary": "",
                         "pricingExpression": {
                             "usageUnit": term.price_dimensions["usageUnit"],
                             "displayQuantity": 1,
                             "tieredRates": [
                                 {
                                     "startUsageAmount": 0,
                                     "unitPrice": {
                                         "currencyCode": currency,
                                         "units": unit_price as i64, 
                                         "nanos": nanos 
                                     }
                                 }
                             ]
                         },
                         "currencyConversionRate": 1,
                         "effectiveTime": term.effective_date
                     }]));
                 }
             }
             skus.push(sku_json);
         }

        let body = json!({
            "skus": skus,
            "nextPageToken": ""
        });
        
        Ok(Response::ok(serde_json::to_vec(&body).unwrap()))
    }
}
