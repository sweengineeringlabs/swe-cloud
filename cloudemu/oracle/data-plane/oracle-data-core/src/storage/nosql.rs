use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoSqlTable {
    pub name: String,
    pub compartment_id: String,
    pub ddl: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoSqlRow {
    pub table_name: String,
    pub key: String,
    pub value: String, // JSON
}

impl StorageEngine {
    pub fn init_nosql_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS oci_nosql_tables (
                name TEXT NOT NULL,
                compartment_id TEXT NOT NULL,
                ddl TEXT NOT NULL,
                state TEXT NOT NULL,
                PRIMARY KEY(compartment_id, name)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS oci_nosql_rows (
                table_name TEXT NOT NULL,
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                PRIMARY KEY(table_name, key)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_nosql_table(&self, name: &str, compartment: &str, ddl: &str) -> Result<NoSqlTable> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "INSERT INTO oci_nosql_tables (name, compartment_id, ddl, state)
             VALUES (?1, ?2, ?3, ?4)",
            params![name, compartment, ddl, "ACTIVE"],
        )?;

        Ok(NoSqlTable {
            name: name.to_string(),
            compartment_id: compartment.to_string(),
            ddl: ddl.to_string(),
            state: "ACTIVE".to_string(),
        })
    }

    pub fn put_nosql_row(&self, table: &str, key: &str, value: &str) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "INSERT OR REPLACE INTO oci_nosql_rows (table_name, key, value)
             VALUES (?1, ?2, ?3)",
            params![table, key, value],
        )?;

        Ok(())
    }
}
