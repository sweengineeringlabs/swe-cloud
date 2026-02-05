use zero_control_spi::{ZeroResult, ZeroError};
use zero_data_core::ZeroEngine;
use std::sync::Arc;

pub struct DbService {
    engine: Arc<ZeroEngine>,
}

impl DbService {
    pub fn new(engine: Arc<ZeroEngine>) -> Self {
        Self { engine }
    }

    pub async fn create_table(&self, name: &str, _pk: &str) -> ZeroResult<()> {
        let conn = self.engine.db.lock();
        // Create a table with dynamic columns? No, simplified: PK + JSON Body
        let sql = format!("CREATE TABLE IF NOT EXISTS {} (
            pk TEXT PRIMARY KEY,
            item_json TEXT NOT NULL
        )", name);
        
        conn.execute(&sql, [])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
        Ok(())
    }

    pub async fn list_tables(&self) -> ZeroResult<Vec<String>> {
        let conn = self.engine.db.lock();
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name != 'nodes'").map_err(|e| ZeroError::Internal(e.to_string()))?;
        let tables = stmt.query_map([], |row| row.get(0)).map_err(|e| ZeroError::Internal(e.to_string()))?
            .collect::<Result<Vec<String>, _>>().map_err(|e| ZeroError::Internal(e.to_string()))?;
        Ok(tables)
    }

    pub async fn put_item(&self, table: &str, pk_value: &str, item: serde_json::Value) -> ZeroResult<()> {
        let conn = self.engine.db.lock();
        let query = format!("INSERT OR REPLACE INTO {} (pk, item_json) VALUES (?1, ?2)", table);
        conn.execute(&query, zero_data_core::rusqlite::params![pk_value, item.to_string()])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
        Ok(())
    }
}
