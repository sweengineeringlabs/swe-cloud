use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use rusqlite::Connection;
use crate::error::Result;

pub struct StorageEngine {
    pub db: Arc<Mutex<Connection>>,
    pub data_dir: PathBuf,
}

pub mod pricing;

impl StorageEngine {
    pub fn new(data_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&data_dir)?;
        let db_path = data_dir.join("metadata.db");
        let conn = Connection::open(&db_path)?;
        
        // Init tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS oci_resources (
                id TEXT PRIMARY KEY,
                ocid TEXT NOT NULL,
                resource_type TEXT NOT NULL,
                data JSON NOT NULL
            )",
            [],
        )?;
        
        // Pricing Tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS pricing_products (
                sku TEXT PRIMARY KEY,
                service_code TEXT,
                product_family TEXT,
                attributes JSON
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS pricing_terms (
                id TEXT PRIMARY KEY,
                sku TEXT,
                offer_term_code TEXT,
                description TEXT,
                effective_date TEXT,
                price_dimensions JSON
            )",
            [],
        )?;

        Ok(Self {
            db: Arc::new(Mutex::new(conn)),
            data_dir,
        })
    }
}
