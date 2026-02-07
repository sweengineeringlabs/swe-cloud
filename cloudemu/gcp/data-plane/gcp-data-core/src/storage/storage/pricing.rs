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

    /// Implement a seeding function for mock GCP billing data
    pub async fn seed_pricing_data(&self) -> Result<()> {
        let conn = self.db.lock();
        let sku = "C028-2F74-78E6"; // Example GCP SKU for e2-micro
        // Check if exists
        let exists: Option<String> = conn.query_row(
            "SELECT sku FROM pricing_products WHERE sku = ?",
            params![sku],
            |row| row.get(0)
        ).optional()?;

        if exists.is_none() {
            let attributes = serde_json::json!({
                "serviceDisplayName": "Compute Engine",
                "resourceFamily": "Compute",
                "resourceGroup": "N1Standard",
                "usageType": "OnDemand",
                "serviceRegions": ["us-central1"],
                "description": "E2 Instance Core running in Americas"
            });
            
            conn.execute(
                r#"
                INSERT INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
                params![sku, "Compute Engine", "Compute", attributes.to_string(), "v1", "2024-01-01T00:00:00Z"]
            )?;

            let dimensions = serde_json::json!({
                "usageUnit": "h",
                "nanos": 8000000,
                "units": 0, 
                "currencyCode": "USD",
                "description": "$0.008 per hour"
            });

            conn.execute(
                 r#"
                INSERT INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
                params![
                    "C028-2F74-78E6.OnDemand",
                    sku,
                    "OnDemand",
                    "E2 Instance Core",
                    "2024-01-01T00:00:00Z",
                    dimensions.to_string()
                ]
            )?;

            // Seed Cloud Storage
            let gcs_sku = "GCS-STD";
            conn.execute(
                "INSERT OR IGNORE INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date) VALUES (?, ?, ?, ?, ?, ?)",
                params![gcs_sku, "Cloud Storage", "Storage", serde_json::json!({"serviceDisplayName": "Cloud Storage", "resourceFamily": "Storage", "resourceGroup": "Standard"}).to_string(), "v1", "2024-01-01T00:00:00Z"]
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    "GCS-STD.OnDemand", gcs_sku, "OnDemand", "Standard Storage", "2024-01-01T00:00:00Z",
                    serde_json::json!({"usageUnit": "GiB-mo", "nanos": 20000000, "units": 0, "currencyCode": "USD", "description": "$0.020 per GiB"}).to_string()
                ]
            )?;

            // Seed Firestore
            let firestore_sku = "FS-READS";
            conn.execute(
                "INSERT OR IGNORE INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date) VALUES (?, ?, ?, ?, ?, ?)",
                params![firestore_sku, "Firestore", "Database", serde_json::json!({"serviceDisplayName": "Firestore", "resourceFamily": "Database", "resourceGroup": "Operations"}).to_string(), "v1", "2024-01-01T00:00:00Z"]
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    "FS-READS.OnDemand", firestore_sku, "OnDemand", "Document Reads", "2024-01-01T00:00:00Z",
                    serde_json::json!({"usageUnit": "100k-ops", "nanos": 60000000, "units": 0, "currencyCode": "USD", "description": "$0.06 per 100k reads"}).to_string()
                ]
            )?;

            // Seed Pub/Sub
            let pubsub_sku = "PS-THROUGHPUT";
            conn.execute(
                "INSERT OR IGNORE INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date) VALUES (?, ?, ?, ?, ?, ?)",
                params![pubsub_sku, "Cloud Pub/Sub", "Messaging", serde_json::json!({"serviceDisplayName": "Cloud Pub/Sub", "resourceFamily": "ApplicationServices", "resourceGroup": "Throughput"}).to_string(), "v1", "2024-01-01T00:00:00Z"]
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    "PS-THROUGHPUT.OnDemand", pubsub_sku, "OnDemand", "Message Throughput", "2024-01-01T00:00:00Z",
                    serde_json::json!({"usageUnit": "GiB", "nanos": 40000000, "units": 0, "currencyCode": "USD", "description": "$0.040 per GiB"}).to_string()
                ]
            )?;

            // Seed Cloud Functions
            let cf_sku = "CF-INVOCATIONS";
            conn.execute(
                "INSERT OR IGNORE INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date) VALUES (?, ?, ?, ?, ?, ?)",
                params![cf_sku, "Cloud Functions", "Compute", serde_json::json!({"serviceDisplayName": "Cloud Functions", "resourceFamily": "Compute", "resourceGroup": "Invocations"}).to_string(), "v1", "2024-01-01T00:00:00Z"]
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    "CF-INVOCATIONS.OnDemand", cf_sku, "OnDemand", "Function Invocations", "2024-01-01T00:00:00Z",
                    serde_json::json!({"usageUnit": "million-ops", "nanos": 400000000, "units": 0, "currencyCode": "USD", "description": "$0.40 per million"}).to_string()
                ]
            )?;

            // Seed Secret Manager
            let sm_sku = "SM-ACCESS";
            conn.execute(
                "INSERT OR IGNORE INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date) VALUES (?, ?, ?, ?, ?, ?)",
                params![sm_sku, "Secret Manager", "Security", serde_json::json!({"serviceDisplayName": "Secret Manager", "resourceFamily": "Security", "resourceGroup": "Access"}).to_string(), "v1", "2024-01-01T00:00:00Z"]
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    "SM-ACCESS.OnDemand", sm_sku, "OnDemand", "Secret Access", "2024-01-01T00:00:00Z",
                    serde_json::json!({"usageUnit": "10k-ops", "nanos": 30000000, "units": 0, "currencyCode": "USD", "description": "$0.03 per 10k ops"}).to_string()
                ]
            )?;
        }
        
        Ok(())
    }
}
