use chrono::Utc;
use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureSqlDatabase {
    pub name: String,
    pub server_name: String,
    pub resource_group: String,
    pub location: String,
    pub sku: String,
    pub status: String,
}

impl StorageEngine {
    const TABLE_AZURE_SQL: &'static str = "azure_sql_databases";

    pub fn init_sql_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                server_name TEXT NOT NULL,
                resource_group TEXT NOT NULL,
                location TEXT NOT NULL,
                sku TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at INTEGER,
                UNIQUE(name, server_name, resource_group)
            )", 
            Self::TABLE_AZURE_SQL
        ), [])?;

        Ok(())
    }

    pub fn create_sql_db(&self, db: AzureSqlDatabase) -> Result<AzureSqlDatabase> {
        let conn = self.get_connection()?;
        let id = format!("/subscriptions/sub-1/resourceGroups/{}/providers/Microsoft.Sql/servers/{}/databases/{}", db.resource_group, db.server_name, db.name);

        conn.execute(
            &format!("INSERT INTO {} (
                id, name, server_name, resource_group, location, sku, status, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", Self::TABLE_AZURE_SQL),
            params![
                id, db.name, db.server_name, db.resource_group, db.location, db.sku, "Online", 
                chrono::Utc::now().timestamp()
            ],
        )?;

        Ok(db)
    }
}
