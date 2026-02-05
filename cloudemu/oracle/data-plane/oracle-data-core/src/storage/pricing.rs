use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::{params, OptionalExtension};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub sku: String,
    pub service_code: String,
    pub product_family: Option<String>,
    pub attributes: serde_json::Value,
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
    ) -> Result<Vec<(Product, Vec<OfferTerm>)>> {
        let conn = self.db.lock().map_err(|_| crate::error::Error::NotFound("Lock poisoned".into()))?;
        let mut stmt = conn.prepare(
            r#"
            SELECT 
                sku, service_code, product_family, attributes
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
            })
        })?;

        let mut results = Vec::new();
        let products: Vec<Product> = products_iter.filter_map(std::result::Result::ok).collect();

        for product in products {
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

        Ok(results)
    }

    /// Implement a seeding function for mock Oracle OCI pricing data
    pub async fn seed_pricing_data(&self) -> Result<()> {
        let conn = self.db.lock().map_err(|_| crate::error::Error::NotFound("Lock poisoned".into()))?;
        
        // Compute (VM.Standard2.1)
        let sku = "B9F0-5A32-9D1C"; 
        let exists: Option<String> = conn.query_row(
            "SELECT sku FROM pricing_products WHERE sku = ?",
            params![sku],
            |row| row.get(0)
        ).optional()?;

        if exists.is_none() {
            let attributes = serde_json::json!({
                "serviceName": "Compute",
                "serviceFamily": "Compute",
                "productName": "VM.Standard2.1",
                "region": "us-ashburn-1"
            });
            
            conn.execute(
                r#"
                INSERT INTO pricing_products (sku, service_code, product_family, attributes)
                VALUES (?, ?, ?, ?)
                "#,
                params![sku, "Compute", "Compute", attributes.to_string()]
            )?;

            let dimensions = serde_json::json!({
                "unitPrice": 0.0638,
                "currency": "USD",
                "unit": "OCPU-Hour",
                "description": "$0.0638 per OCPU Hour"
            });

            conn.execute(
                 r#"
                INSERT INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
                params![
                    "B9F0-5A32-9D1C.OnDemand",
                    sku,
                    "OnDemand",
                    "VM.Standard2.1 OCPU",
                    "2024-01-01T00:00:00Z",
                    dimensions.to_string()
                ]
            )?;
            
             // Object Storage (Standard)
            let storage_sku = "OCI-OBJ-STD";
            conn.execute(
                "INSERT INTO pricing_products (sku, service_code, product_family, attributes) VALUES (?, ?, ?, ?)",
                params![storage_sku, "Object Storage", "Storage", serde_json::json!({"serviceName": "Object Storage", "storageTier": "Standard"}).to_string()]
            )?;
            
            conn.execute(
                "INSERT INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    "OCI-OBJ-STD.OnDemand", 
                    storage_sku, 
                    "OnDemand", 
                    "Standard Object Storage", 
                    "2024-01-01T00:00:00Z",
                    serde_json::json!({"unitPrice": 0.0255, "currency": "USD", "unit": "GB-Month", "description": "$0.0255 per GB Month"}).to_string()
                ]
            )?;
        }
        
        Ok(())
    }
}
