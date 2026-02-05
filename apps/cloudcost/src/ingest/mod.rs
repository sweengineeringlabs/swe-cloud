use crate::core::FocusItem;
use reqwest::Client;
use serde_json::Value;

pub struct Ingestor {
    client: Client,
    base_urls: BaseUrls,
}

struct BaseUrls {
    aws: String,
    azure: String,
    gcp: String,
    oracle: String,
}

impl Ingestor {
    pub async fn new() -> Self {
        Self {
            client: Client::new(),
            base_urls: BaseUrls {
                aws: "http://localhost:4566".to_string(), // AWS Pricing is usually on us-east-1 region URL, but we simplify
                azure: "http://localhost:10000".to_string(),
                gcp: "http://localhost:4567".to_string(),
                oracle: "http://localhost:4568".to_string(),
            },
        }
    }
    
    pub async fn fetch_prices(&self, provider: &str) -> Result<Vec<FocusItem>, Box<dyn std::error::Error>> {
        match provider.to_lowercase().as_str() {
            "aws" => self.fetch_aws().await,
            "azure" => self.fetch_azure().await,
            "gcp" => self.fetch_gcp().await,
            "oracle" => self.fetch_oracle().await,
            _ => Err("Unknown provider".into()),
        }
    }

    async fn fetch_aws(&self) -> Result<Vec<FocusItem>, Box<dyn std::error::Error>> {
        // Mock query for EC2
        let url = format!("{}/pricing/v1/services/AmazonEC2/products", self.base_urls.aws);
        // Note: Real AWS Pricing is complex. We assume CloudEmu returns our normalized simple list for now,
        // or we adapt the response.
        // Let's assume CloudEmu returns the Price List JSON structure.
        // For CLI demo purposes, we might just hit a simplified endpoint if we built one, 
        // but let's try to parse the structure we stored in SQLite.
        
        // Actually, we implemented `GetProducts` in AWS pricing service.
        // Let's call that.
        // CLI: aws pricing get-products --service-code AmazonEC2
        // API: POST / with Header X-Amz-Target: AWSPriceListService.GetProducts
        
        let res = self.client.post(&self.base_urls.aws)
            .header("X-Amz-Target", "AWSPriceListService.GetProducts")
            .header("Content-Type", "application/x-amz-json-1.1")
            .body(r#"{"ServiceCode":"AmazonEC2"}"#)
            .send()
            .await?;
            
        let body: Value = res.json().await?;
        let price_list = body["PriceList"].as_array().ok_or("No PriceList found")?;
        
        let mut items = Vec::new();
        for price_str_val in price_list {
            // AWS returns JSON embedded in strings sometimes, or pure JSON? 
            // In our impl we returned pure JSON objects in the list provided by `aws-control-core`?
            // Checking `aws-control-core`: It returns a list of strings (JSON seralized).
            
            let price_json: Value = if price_str_val.is_string() {
                serde_json::from_str(price_str_val.as_str().unwrap())?
            } else {
                price_str_val.clone()
            };

            let sku = price_json["product"]["sku"].as_str().unwrap_or("unknown").to_string();
            // Simplify extraction of OnDemand price
            let terms = &price_json["terms"]["OnDemand"];
            // Deep nested traversing simplified for demo
            if let Some(term_obj) = terms.as_object() {
                 if let Some((_, term_details)) = term_obj.iter().next() {
                     if let Some(dim_obj) = term_details["priceDimensions"].as_object() {
                         if let Some((_, dim_details)) = dim_obj.iter().next() {
                             let price_per_unit = dim_details["pricePerUnit"]["USD"].as_str().unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
                             let unit = dim_details["unit"].as_str().unwrap_or("Unit").to_string();
                             
                             items.push(FocusItem {
                                 provider: "AWS".to_string(),
                                 service_category: "Compute".to_string(),
                                 resource_id: None,
                                 sku: sku.clone(),
                                 billed_cost: price_per_unit,
                                 currency: "USD".to_string(),
                                 usage_quantity: 1.0,
                                 usage_unit: unit,
                                 effective_cost: price_per_unit, 
                             });
                         }
                     }
                 }
            }
        }
        
        Ok(items)
    }

    async fn fetch_azure(&self) -> Result<Vec<FocusItem>, Box<dyn std::error::Error>> {
        let url = format!("{}/api/retail/prices?api-version=2023-01-01-preview", self.base_urls.azure);
        let res = self.client.get(&url).send().await?;
        let body: Value = res.json().await?;
        
        let mut items = Vec::new();
        if let Some(vals) = body["Items"].as_array() {
            for v in vals {
                let sku = v["skuId"].as_str().unwrap_or("unknown").to_string();
                let price = v["retailPrice"].as_f64().unwrap_or(0.0);
                let unit = v["unitOfMeasure"].as_str().unwrap_or("Unit").to_string();
                let service = v["serviceName"].as_str().unwrap_or("Unknown").to_string();

                items.push(FocusItem {
                    provider: "Azure".to_string(),
                    service_category: service,
                    resource_id: None,
                    sku,
                    billed_cost: price,
                    currency: v["currencyCode"].as_str().unwrap_or("USD").to_string(),
                    usage_quantity: 1.0,
                    usage_unit: unit,
                    effective_cost: price,
                });
            }
        }
        Ok(items)
    }

    async fn fetch_gcp(&self) -> Result<Vec<FocusItem>, Box<dyn std::error::Error>> {
        // Mock: list_skus for Compute
        let url = format!("{}/v1/services/6F81-5844-456A/skus", self.base_urls.gcp);
        let res = self.client.get(&url).send().await?;
        let body: Value = res.json().await?;
        
        let mut items = Vec::new();
        if let Some(skus) = body["skus"].as_array() {
            for sku in skus {
                let id = sku["skuId"].as_str().unwrap_or("unknown").to_string();
                let cat = sku["category"]["resourceFamily"].as_str().unwrap_or("Unknown").to_string();
                
                // Simplified price extraction (GCP pricing is complex nanos/units)
                let price_info = &sku["pricingInfo"][0]["pricingExpression"]["tieredRates"][0]["unitPrice"];
                let units = price_info["units"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                let nanos = price_info["nanos"].as_f64().unwrap_or(0.0);
                let price = units + (nanos / 1_000_000_000.0);
                
                items.push(FocusItem {
                     provider: "Google Cloud".to_string(),
                     service_category: cat,
                     resource_id: None,
                     sku: id,
                     billed_cost: price,
                     currency: "USD".to_string(), 
                     usage_quantity: 1.0,
                     usage_unit: "Unit".to_string(),
                     effective_cost: price,
                });
            }
        }
        Ok(items)
    }

    async fn fetch_oracle(&self) -> Result<Vec<FocusItem>, Box<dyn std::error::Error>> {
         let url = format!("{}/metering/api/v1/prices", self.base_urls.oracle);
         let res = self.client.get(&url).send().await?;
         let body: Value = res.json().await?;
         
         let mut items = Vec::new();
         if let Some(list) = body["items"].as_array() {
             for item in list {
                 let sku = item["partNumber"].as_str().unwrap_or("unknown").to_string();
                 let service = item["service"].as_str().unwrap_or("Unknown").to_string();
                 let price = item["price"]["unitPrice"].as_f64().unwrap_or(0.0);
                 let unit = item["price"]["unit"].as_str().unwrap_or("Unit").to_string();
                 
                  items.push(FocusItem {
                     provider: "Oracle".to_string(),
                     service_category: service,
                     resource_id: None,
                     sku,
                     billed_cost: price,
                     currency: item["price"]["currency"].as_str().unwrap_or("USD").to_string(),
                     usage_quantity: 1.0,
                     usage_unit: unit,
                     effective_cost: price,
                });
             }
         }
         Ok(items)
    }
}
