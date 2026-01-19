use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use rusqlite::Connection;
use crate::error::Result;

pub struct StorageEngine {
    pub db: Arc<Mutex<Connection>>,
    pub data_dir: PathBuf,
}

pub mod pricing;
pub mod compute;
pub mod database;
pub mod identity;
pub mod dns;
pub mod object_storage;
pub mod monitoring;
pub mod functions;
pub mod queue;
pub mod networking;
pub mod containers;
pub mod vault;
pub mod nosql;

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

        let engine = Self {
            db: Arc::new(Mutex::new(conn)),
            data_dir,
        };

        engine.init_compute_tables()?;
        engine.init_db_tables()?;
        engine.init_identity_tables()?;
        engine.init_dns_tables()?;
        engine.init_object_storage_tables()?;
        engine.init_monitoring_tables()?;
        engine.init_functions_tables()?;
        engine.init_queue_tables()?;
        engine.init_networking_tables()?;
        engine.init_containers_tables()?;
        engine.init_vault_tables()?;
        engine.init_nosql_tables()?;
        
        Ok(engine)
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let engine = Self {
            db: Arc::new(Mutex::new(conn)),
            data_dir: PathBuf::from(""), // In-memory
        };
        engine.init_compute_tables()?;
        engine.init_db_tables()?;
        engine.init_identity_tables()?;
        engine.init_dns_tables()?;
        engine.init_object_storage_tables()?;
        engine.init_monitoring_tables()?;
        engine.init_monitoring_tables()?;
        engine.init_functions_tables()?;
        engine.init_queue_tables()?;
        engine.init_networking_tables()?;
        engine.init_containers_tables()?;
        engine.init_vault_tables()?;
        engine.init_nosql_tables()?;
        Ok(engine)
    }
    pub fn get_connection(&self) -> Result<std::sync::MutexGuard<rusqlite::Connection>> {
        self.db.lock().map_err(|_| crate::error::Error::NotFound("Lock poisoned".into()))
    }
}
