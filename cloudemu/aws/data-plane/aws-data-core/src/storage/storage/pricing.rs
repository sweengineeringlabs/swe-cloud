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

    /// Implement a seeding function for mock data
    pub async fn seed_pricing_data(&self) -> Result<()> {
        let conn = self.db.lock();
        let sku = "ABC-123";
        // Check if exists
        let exists: Option<String> = conn.query_row(
            "SELECT sku FROM pricing_products WHERE sku = ?",
            params![sku],
            |row| row.get(0)
        ).optional()?;

        if exists.is_none() {
            let attributes = serde_json::json!({
                "location": "US East (N. Virginia)",
                "instanceType": "t3.micro",
                "operatingSystem": "Linux"
            });
            
            conn.execute(
                r#"
                INSERT INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
                params![sku, "AmazonEC2", "Compute Instance", attributes.to_string(), "20240101", "2024-01-01T00:00:00Z"]
            )?;

            let dimensions = serde_json::json!({
                "ABC-123.JRTCKXETXF.6YS6EN2CT7": {
                    "unit": "Hrs",
                    "pricePerUnit": { "USD": "0.0104" },
                    "description": "$0.0104 per On Demand Linux t3.micro Instance Hour"
                }
            });

            conn.execute(
                 r#"
                INSERT INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
                params![
                    "ABC-123.JRTCKXETXF",
                    sku,
                    "JRTCKXETXF",
                    "OnDemand",
                    "2024-01-01T00:00:00Z",
                    dimensions.to_string()
                ]
            )?;

            // Seed S3
            let s3_sku = "S3-STD-STORAGE";
            conn.execute(
                "INSERT OR IGNORE INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date) VALUES (?, ?, ?, ?, ?, ?)",
                params![s3_sku, "AmazonS3", "Storage", serde_json::json!({"location": "US East (N. Virginia)", "storageClass": "General Purpose"}).to_string(), "20240101", "2024-01-01T00:00:00Z"]
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    "S3-STD.OnDemand", s3_sku, "OnDemand", "S3 Standard Storage", "2024-01-01T00:00:00Z",
                    serde_json::json!({"S3-STD.OnDemand.Price": {"unit": "GB-Mo", "pricePerUnit": {"USD": "0.023"}, "description": "$0.023 per GB"}}).to_string()
                ]
            )?;

            // Seed DynamoDB
            let ddb_sku = "DDB-IER";
            conn.execute(
                "INSERT OR IGNORE INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date) VALUES (?, ?, ?, ?, ?, ?)",
                params![ddb_sku, "AmazonDynamoDB", "Database", serde_json::json!({"location": "US East (N. Virginia)", "group": "DDB-WriteUnits"}).to_string(), "20240101", "2024-01-01T00:00:00Z"]
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    "DDB-IER.OnDemand", ddb_sku, "OnDemand", "DynamoDB Write Units", "2024-01-01T00:00:00Z",
                    serde_json::json!({"DDB-IER.OnDemand.Price": {"unit": "WriteCapacityUnit-Hrs", "pricePerUnit": {"USD": "0.00065"}, "description": "$0.00065 per WCU"}}).to_string()
                ]
            )?;

            // Seed SQS
            let sqs_sku = "SQS-REQ";
            conn.execute(
                "INSERT OR IGNORE INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date) VALUES (?, ?, ?, ?, ?, ?)",
                params![sqs_sku, "AmazonSQS", "Queue", serde_json::json!({"location": "US East (N. Virginia)", "group": "SQS-Requests"}).to_string(), "20240101", "2024-01-01T00:00:00Z"]
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    "SQS-REQ.OnDemand", sqs_sku, "OnDemand", "SQS Requests", "2024-01-01T00:00:00Z",
                    serde_json::json!({"SQS-REQ.OnDemand.Price": {"unit": "Requests", "pricePerUnit": {"USD": "0.0000004"}, "description": "$0.40 per Million Requests"}}).to_string()
                ]
            )?;

            // Seed Lambda
            let lambda_sku = "LAMBDA-REQ";
            conn.execute(
                "INSERT OR IGNORE INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date) VALUES (?, ?, ?, ?, ?, ?)",
                params![lambda_sku, "AWSLambda", "Serverless", serde_json::json!({"location": "US East (N. Virginia)", "group": "Lambda-Requests"}).to_string(), "20240101", "2024-01-01T00:00:00Z"]
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    "LAMBDA-REQ.OnDemand", lambda_sku, "OnDemand", "Lambda Requests", "2024-01-01T00:00:00Z",
                    serde_json::json!({"LAMBDA-REQ.OnDemand.Price": {"unit": "Requests", "pricePerUnit": {"USD": "0.0000002"}, "description": "$0.20 per Million Requests"}}).to_string()
                ]
            )?;

            // Seed Secrets Manager
            let secrets_sku = "SECRETS-STORE";
            conn.execute(
                "INSERT OR IGNORE INTO pricing_products (sku, service_code, product_family, attributes, version, publication_date) VALUES (?, ?, ?, ?, ?, ?)",
                params![secrets_sku, "AWSSecretsManager", "Security", serde_json::json!({"location": "US East (N. Virginia)", "group": "Secrets-Storage"}).to_string(), "20240101", "2024-01-01T00:00:00Z"]
            )?;
            conn.execute(
                "INSERT OR IGNORE INTO pricing_terms (id, sku, offer_term_code, description, effective_date, price_dimensions) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    "SECRETS-STORE.OnDemand", secrets_sku, "OnDemand", "Secrets Storage", "2024-01-01T00:00:00Z",
                    serde_json::json!({"SECRETS-STORE.OnDemand.Price": {"unit": "Secret-Mo", "pricePerUnit": {"USD": "0.40"}, "description": "$0.40 per Secret per Month"}}).to_string()
                ]
            )?;
        }
        
        Ok(())
    }
}
