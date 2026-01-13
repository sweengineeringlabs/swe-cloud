use super::engine::{StorageEngine, TableMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== DynamoDB Operations ====================

    // NOTE: This function will be refactored with a builder pattern in upcoming storage refactor
    #[allow(clippy::too_many_arguments)]
    pub fn create_table(&self, name: &str, attr_defs: &str, key_schema: &str, account_id: &str, region: &str) -> Result<TableMetadata> {
        let db = self.db.lock();
        let arn = format!("arn:aws:dynamodb:{}:{}:table/{}", region, account_id, name);
        let now = chrono::Utc::now().to_rfc3339();

        db.execute(
            "INSERT INTO ddb_tables (name, arn, attribute_definitions, key_schema, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![name, arn, attr_defs, key_schema, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Table {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(TableMetadata {
            name: name.to_string(),
            arn,
            status: "ACTIVE".to_string(),
            attribute_definitions: attr_defs.to_string(),
            key_schema: key_schema.to_string(),
            created_at: now,
        })
    }

    pub fn put_item(&self, table_name: &str, pk: &str, sk: Option<&str>, item_json: &str) -> Result<()> {
        let db = self.db.lock();
        db.execute(
            "INSERT OR REPLACE INTO ddb_items (table_name, partition_key, sort_key, item_json) VALUES (?1, ?2, ?3, ?4)",
            params![table_name, pk, sk, item_json],
        )?;
        Ok(())
    }

    pub fn get_item(&self, table_name: &str, pk: &str, sk: Option<&str>) -> Result<Option<String>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            "SELECT item_json FROM ddb_items WHERE table_name = ?1 AND partition_key = ?2 AND (sort_key = ?3 OR sort_key IS NULL)"
        )?;
        let result = stmt.query_row(params![table_name, pk, sk], |row| row.get(0)).ok();
        Ok(result)
    }

    pub fn query_items(&self, table_name: &str, pk: &str) -> Result<Vec<String>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            "SELECT item_json FROM ddb_items WHERE table_name = ?1 AND partition_key = ?2"
        )?;
        let rows = stmt.query_map(params![table_name, pk], |row| row.get(0))
            .map_err(|e| EmulatorError::Database(e.to_string()))?;
        
        let mut items = Vec::new();
        for item in rows {
            items.push(item.map_err(|e| EmulatorError::Database(e.to_string()))?);
        }
        Ok(items)
    }
}
