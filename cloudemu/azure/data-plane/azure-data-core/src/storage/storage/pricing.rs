use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::{params, OptionalExtension};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub sku: String,
    pub service_code: String,
    pub product_family: Option<String>,
    pub attributes: serde_json::Value,
    pub version: Option<String>,
    pub publication_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfferTerm {
    pub id: String,
    pub sku: String,
    pub offer_term_code: String,
    pub description: Option<String>,
    pub effective_date: Option<String>,
    pub price_dimensions: serde_json::Value,
}

impl super::StorageEngine {
    pub async fn get_products(
        &self,
        service_code: &str,
        filter_fn: impl Fn(&Product) -> bool,
    ) -> Result<Vec<(Product, Vec<OfferTerm>)>> {
        let conn = self.db.lock();
        let mut stmt = conn.prepare(
            r#"
            SELECT 
                sku, service_code, product_family, 
                attributes, version, publication_date
            FROM pricing_products
            WHERE service_code = ?
            "#,
        )?;

        let products_iter = stmt.query_map(params![service_code], |row| {
            let attributes_str: String = row.get(3)?;
            let attributes: serde_json::Value = serde_json::from_str(&attributes_str).unwrap_or(serde_json::json!({}));
            
            Ok(Product {
                sku: row.get(0)?,
                service_code: row.get(1)?,
                product_family: row.get(2)?,
                attributes,
                version: row.get(4)?,
                publication_date: row.get(5)?,
            })
        })?;

        let mut results = Vec::new();
        let products: Vec<Product> = products_iter.filter_map(std::result::Result::ok).collect();

        for product in products {
             if filter_fn(&product) {
                 let mut stmt_terms = conn.prepare(
                    r#"
                    SELECT 
                        id, sku, offer_term_code, description, 
                        effective_date, price_dimensions
                    FROM pricing_terms
                    WHERE sku = ?
                    "#,
                )?;

                let terms_iter = stmt_terms.query_map(params![product.sku], |row| {
                    let dims_str: String = row.get(5)?;
                    let price_dimensions: serde_json::Value = serde_json::from_str(&dims_str).unwrap_or(serde_json::json!({}));
                    
                    Ok(OfferTerm {
                        id: row.get(0)?,
                        sku: row.get(1)?,
                        offer_term_code: row.get(2)?,
                        description: row.get(3)?,
                        effective_date: row.get(4)?,
                        price_dimensions,
                    })
                })?;
                
                let terms: Vec<OfferTerm> = terms_iter.filter_map(std::result::Result::ok).collect();
                results.push((product, terms));
             }
        }

        Ok(results)
    }

    /// Implement a seeding function for mock Azure retail prices
    pub async fn seed_pricing_data(&self) -> Result<()> {
        let conn = self.db.lock();
        let sku = "DZH318Z0BQPS"; // Example Azure SKU
        // Check if exists
        let exists: Option<String> = conn.query_row(
            "SELECT sku FROM pricing_products WHERE sku = ?",
            params![sku],
            |row| row.get(0)
        ).optional()?;

        if exists.is_none() {
            let attributes = serde_json::json!({
                "serviceName": "Virtual Machines",
                "serviceFamily": "Compute",
                "priceType": "Consumption",
                "armRegionName": "eastus",
                "skuName": "Standard_D2s_v3",
                "productName": "Virtual Machines D2s v3 Series"
            });
            
            conn.execute(
                r#"
                INSERT INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
                params![sku, "Virtual Machines", "Compute", attributes.to_string(), "20240101", "2024-01-01T00:00:00Z"]
            )?;

            let dimensions = serde_json::json!({
                "retailPrice": 0.096,
                "currencyCode": "USD",
                "unitOfMeasure": "1 Hour",
                "type": "Consumption"
            });

            conn.execute(
                 r#"
                INSERT INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
                params![
                    "DZH318Z0BQPS.Consumption",
                    sku,
                    "Consumption",
                    "$0.096/Hour",
                    "2024-01-01T00:00:00Z",
                    dimensions.to_string()
                ]
            )?;

            // Seed Storage (Blob)
            let blob_sku = "AZ-BLOB-HOT";
            conn.execute(
                "INSERT OR IGNORE INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date) VALUES (?, ?, ?, ?, ?, ?)",
                params![blob_sku, "Storage", "Storage", serde_json::json!({"serviceName": "Storage", "skuName": "Hot LRS", "productName": "Blob Storage Hot LRS"}).to_string(), "20240101", "2024-01-01T00:00:00Z"]
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    "AZ-BLOB-HOT.Consumption", blob_sku, "Consumption", "Blob Hot LRS", "2024-01-01T00:00:00Z",
                    serde_json::json!({"retailPrice": 0.0184, "currencyCode": "USD", "unitOfMeasure": "1 GB/Month", "type": "Consumption"}).to_string()
                ]
            )?;

            // Seed Cosmos DB
            let cosmos_sku = "AZ-COSMOS-RU";
            conn.execute(
                "INSERT OR IGNORE INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date) VALUES (?, ?, ?, ?, ?, ?)",
                params![cosmos_sku, "Azure Cosmos DB", "Database", serde_json::json!({"serviceName": "Azure Cosmos DB", "skuName": "Standard", "productName": "Cosmos DB RUs"}).to_string(), "20240101", "2024-01-01T00:00:00Z"]
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    "AZ-COSMOS-RU.Consumption", cosmos_sku, "Consumption", "Cosmos DB RUs", "2024-01-01T00:00:00Z",
                    serde_json::json!({"retailPrice": 0.00008, "currencyCode": "USD", "unitOfMeasure": "100 RU/s/Hour", "type": "Consumption"}).to_string()
                ]
            )?;

            // Seed Service Bus
            let sb_sku = "AZ-SB-STD";
            conn.execute(
                "INSERT OR IGNORE INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date) VALUES (?, ?, ?, ?, ?, ?)",
                params![sb_sku, "Service Bus", "Messaging", serde_json::json!({"serviceName": "Service Bus", "skuName": "Standard", "productName": "Service Bus Standard Messaging"}).to_string(), "20240101", "2024-01-01T00:00:00Z"]
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    "AZ-SB-STD.Consumption", sb_sku, "Consumption", "Service Bus Standard", "2024-01-01T00:00:00Z",
                    serde_json::json!({"retailPrice": 0.0135, "currencyCode": "USD", "unitOfMeasure": "1 Hour", "type": "Consumption"}).to_string()
                ]
            )?;

            // Seed Functions
            let func_sku = "AZ-FUNC-CONS";
            conn.execute(
                "INSERT OR IGNORE INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date) VALUES (?, ?, ?, ?, ?, ?)",
                params![func_sku, "Azure Functions", "Compute", serde_json::json!({"serviceName": "Azure Functions", "skuName": "Consumption", "productName": "Functions Consumption"}).to_string(), "20240101", "2024-01-01T00:00:00Z"]
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    "AZ-FUNC-CONS.Consumption", func_sku, "Consumption", "Functions Execution", "2024-01-01T00:00:00Z",
                    serde_json::json!({"retailPrice": 0.000016, "currencyCode": "USD", "unitOfMeasure": "GB-s", "type": "Consumption"}).to_string()
                ]
            )?;

            // Seed Key Vault
            let kv_sku = "AZ-KV-STD";
            conn.execute(
                "INSERT OR IGNORE INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date) VALUES (?, ?, ?, ?, ?, ?)",
                params![kv_sku, "Key Vault", "Security", serde_json::json!({"serviceName": "Key Vault", "skuName": "Standard", "productName": "Key Vault Secrets"}).to_string(), "20240101", "2024-01-01T00:00:00Z"]
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    "AZ-KV-STD.Consumption", kv_sku, "Consumption", "Key Vault Operations", "2024-01-01T00:00:00Z",
                    serde_json::json!({"retailPrice": 0.03, "currencyCode": "USD", "unitOfMeasure": "10K Operations", "type": "Consumption"}).to_string()
                ]
            )?;
        }
        
        Ok(())
    }
}
