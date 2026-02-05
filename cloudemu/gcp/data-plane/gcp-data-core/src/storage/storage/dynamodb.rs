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

    pub fn get_table(&self, name: &str) -> Result<TableMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT name, arn, status, attribute_definitions, key_schema, created_at FROM ddb_tables WHERE name = ?1",
            params![name],
            |row| Ok(TableMetadata {
                name: row.get(0)?,
                arn: row.get(1)?,
                status: row.get(2)?,
                attribute_definitions: row.get(3)?,
                key_schema: row.get(4)?,
                created_at: row.get(5)?,
            })
        ).map_err(|_| EmulatorError::NotFound("Table".into(), name.into()))
    }

    pub fn put_item(&self, table_name: &str, pk: &str, sk: Option<&str>, item_json: &str) -> Result<()> {
        let db = self.db.lock();
        
        // SQLite treats NULLs as distinct in UNIQUE/PK constraints, so INSERT OR REPLACE doesn't work for NULL sort_keys.
        // We manually delete conflict if it exists.
        if sk.is_none() {
            db.execute(
                "DELETE FROM ddb_items WHERE table_name = ?1 AND partition_key = ?2 AND sort_key IS NULL",
                params![table_name, pk],
            )?;
        }
        
        db.execute(
            "INSERT OR REPLACE INTO ddb_items (table_name, partition_key, sort_key, item_json) VALUES (?1, ?2, ?3, ?4)",
            params![table_name, pk, sk, item_json],
        )?;
        Ok(())
    }

    pub fn get_item(&self, table_name: &str, pk: &str, sk: Option<&str>) -> Result<Option<String>> {
        let db = self.db.lock();
        
        if let Some(s) = sk {
            let mut stmt = db.prepare(
                "SELECT item_json FROM ddb_items WHERE table_name = ?1 AND partition_key = ?2 AND sort_key = ?3"
            )?;
            let result = stmt.query_row(params![table_name, pk, s], |row| row.get(0)).ok();
            Ok(result)
        } else {
            let mut stmt = db.prepare(
                "SELECT item_json FROM ddb_items WHERE table_name = ?1 AND partition_key = ?2 AND sort_key IS NULL"
            )?;
            let result = stmt.query_row(params![table_name, pk], |row| row.get(0)).ok();
            Ok(result)
        }
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
    pub fn scan_items(&self, table_name: &str) -> Result<Vec<String>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            "SELECT item_json FROM ddb_items WHERE table_name = ?1"
        )?;
        let rows = stmt.query_map(params![table_name], |row| row.get(0))
            .map_err(|e| EmulatorError::Database(e.to_string()))?;
        
        let mut items = Vec::new();
        for item in rows {
            items.push(item.map_err(|e| EmulatorError::Database(e.to_string()))?);
        }
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_dynamodb_query_and_scan() {
        let engine = StorageEngine::in_memory().unwrap();
        
        // Create table
        engine.create_table("users", "{}", "{}", "000000000000", "us-east-1").unwrap();
        
        // Put multiple items
        let items = [
            json!({"userId": {"S": "user1"}, "name": {"S": "Alice"}}),
            json!({"userId": {"S": "user1"}, "name": {"S": "Alice Updated"}}),
            json!({"userId": {"S": "user2"}, "name": {"S": "Bob"}}),
        ];
        
        for item in items.iter() {
            let pk = item["userId"]["S"].as_str().unwrap();
            engine.put_item("users", pk, None, &item.to_string()).unwrap();
        }
        
        // Query by partition key
        let query_results = engine.query_items("users", "user1").unwrap();
        assert_eq!(query_results.len(), 1);
        
        // Scan entire table
        let scan_results = engine.scan_items("users").unwrap();
        assert_eq!(scan_results.len(), 2);
    }
    
    #[test]
    fn test_dynamodb_get_put() {
        let engine = StorageEngine::in_memory().unwrap();
        engine.create_table("test", "{}", "{}", "000000000000", "us-east-1").unwrap();
        
        let item = json!({"id": {"S": "1"}, "data": {"S": "value"}}).to_string();
        engine.put_item("test", "1", None, &item).unwrap();
        
        let retrieved = engine.get_item("test", "1", None).unwrap();
        assert_eq!(retrieved, Some(item));
    }
}
