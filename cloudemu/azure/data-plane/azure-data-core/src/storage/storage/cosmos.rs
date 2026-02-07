use uuid::Uuid;
use chrono::Utc;
use super::engine::{StorageEngine, CosmosAccountMetadata, CosmosDatabaseMetadata, CosmosContainerMetadata, CosmosItemMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== Cosmos Account Operations ====================

    pub fn create_cosmos_account(&self, name: &str, location: &str, resource_group: &str) -> Result<()> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();

        db.execute(
            "INSERT INTO az_cosmos_accounts (name, location, resource_group, created_at) VALUES (?, ?, ?, ?)",
            params![name, location, resource_group, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Cosmos account {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(())
    }

    pub fn get_cosmos_account(&self, name: &str) -> Result<CosmosAccountMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT name, location, resource_group, kind, created_at FROM az_cosmos_accounts WHERE name = ?",
            params![name],
            |row| {
                Ok(CosmosAccountMetadata {
                    name: row.get(0)?,
                    location: row.get(1)?,
                    resource_group: row.get(2)?,
                    kind: row.get(3)?,
                    created_at: row.get(4)?,
                })
            },
        ).map_err(|_| EmulatorError::NotFound("CosmosAccount".into(), name.into()))
    }

    // ==================== Database Operations ====================

    pub fn create_cosmos_database(&self, account_name: &str, name: &str) -> Result<()> {
        if let Err(EmulatorError::NotFound(_, _)) = self.get_cosmos_account(account_name) {
             self.create_cosmos_account(account_name, "local", "default")?;
        }

        let db = self.db.lock();
        let etag = format!("\"{}\"", uuid::Uuid::new_v4());

        db.execute(
            "INSERT INTO az_cosmos_databases (name, account_name, etag) VALUES (?, ?, ?)",
            params![name, account_name, etag],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Database {}/{} already exists", account_name, name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(())
    }

    pub fn get_cosmos_database(&self, account_name: &str, name: &str) -> Result<CosmosDatabaseMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT name, account_name, throughput, etag FROM az_cosmos_databases WHERE account_name = ? AND name = ?",
            params![account_name, name],
            |row| {
                Ok(CosmosDatabaseMetadata {
                    name: row.get(0)?,
                    account_name: row.get(1)?,
                    throughput: row.get(2)?,
                    etag: row.get(3)?,
                })
            },
        ).map_err(|_| EmulatorError::NotFound("CosmosDatabase".into(), format!("{} / {}", account_name, name)))
    }

    // ==================== Container Operations ====================

    pub fn create_cosmos_container(&self, account_name: &str, database_name: &str, name: &str, partition_key_path: &str) -> Result<()> {
        self.get_cosmos_database(account_name, database_name)?;

        let db = self.db.lock();
        let etag = format!("\"{}\"", uuid::Uuid::new_v4());

        db.execute(
            "INSERT INTO az_cosmos_containers (name, database_name, account_name, partition_key_path, etag) VALUES (?, ?, ?, ?, ?)",
            params![name, database_name, account_name, partition_key_path, etag],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Container {}/{}/{} already exists", account_name, database_name, name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(())
    }

    pub fn get_cosmos_container(&self, account_name: &str, database_name: &str, name: &str) -> Result<CosmosContainerMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT name, database_name, account_name, partition_key_path, throughput, etag FROM az_cosmos_containers WHERE account_name = ? AND database_name = ? AND name = ?",
            params![account_name, database_name, name],
            |row| {
                Ok(CosmosContainerMetadata {
                    name: row.get(0)?,
                    database_name: row.get(1)?,
                    account_name: row.get(2)?,
                    partition_key_path: row.get(3)?,
                    throughput: row.get(4)?,
                    etag: row.get(5)?,
                })
            },
        ).map_err(|_| EmulatorError::NotFound("CosmosContainer".into(), format!("{} / {} / {}", account_name, database_name, name)))
    }

    // ==================== Item Operations ====================

    pub fn create_cosmos_item(&self, account_name: &str, database_name: &str, container_name: &str, item_json: &serde_json::Value) -> Result<CosmosItemMetadata> {
        // Validate container exists and get partition key path
        let container = self.get_cosmos_container(account_name, database_name, container_name)?;
        
        // Extract ID and partition key
        let id = item_json["id"].as_str()
            .ok_or_else(|| EmulatorError::InvalidRequest("Item must have an 'id' field".to_string()))?;
            
        // Clean partition key path (remove leading /)
        let pk_path = container.partition_key_path.trim_start_matches('/');
        let pk_value = item_json[pk_path].as_str()
            .ok_or_else(|| EmulatorError::InvalidRequest(format!("Item must have partition key field '{}'", pk_path)))?;

        let db = self.db.lock();
        let now = chrono::Utc::now().timestamp();
        let etag = format!("\"{}\"", uuid::Uuid::new_v4());
        let json_str = item_json.to_string();

        db.execute(
            r#"INSERT OR REPLACE INTO az_cosmos_items 
               (id, container_name, database_name, account_name, partition_key_value, item_json, last_modified, etag)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            params![
                id,
                container_name,
                database_name,
                account_name,
                pk_value,
                json_str,
                now,
                etag,
            ],
        )?;

        Ok(CosmosItemMetadata {
            id: id.to_string(),
            container_name: container_name.to_string(),
            database_name: database_name.to_string(),
            account_name: account_name.to_string(),
            partition_key_value: pk_value.to_string(),
            item_json: json_str,
            last_modified: now,
            etag,
        })
    }

    pub fn get_cosmos_item(&self, account_name: &str, database_name: &str, container_name: &str, id: &str, partition_key: &str) -> Result<CosmosItemMetadata> {
        let db = self.db.lock();
        db.query_row(
            r#"SELECT id, container_name, database_name, account_name, partition_key_value, item_json, last_modified, etag 
               FROM az_cosmos_items 
               WHERE account_name = ? AND database_name = ? AND container_name = ? AND id = ? AND partition_key_value = ?"#,
            params![account_name, database_name, container_name, id, partition_key],
            |row| {
                Ok(CosmosItemMetadata {
                    id: row.get(0)?,
                    container_name: row.get(1)?,
                    database_name: row.get(2)?,
                    account_name: row.get(3)?,
                    partition_key_value: row.get(4)?,
                    item_json: row.get(5)?,
                    last_modified: row.get(6)?,
                    etag: row.get(7)?,
                })
            },
        ).map_err(|_| EmulatorError::NotFound("CosmosItem".into(), format!("{} (pk: {})", id, partition_key)))
    }

    pub fn query_cosmos_items(&self, account_name: &str, database_name: &str, container_name: &str, query: &str) -> Result<Vec<CosmosItemMetadata>> {
        // Simple scan for now - real SQL parsing is complex
        let db = self.db.lock();
        let mut stmt = db.prepare(
            r#"SELECT id, container_name, database_name, account_name, partition_key_value, item_json, last_modified, etag 
               FROM az_cosmos_items 
               WHERE account_name = ? AND database_name = ? AND container_name = ?"#
        )?;

        let items = stmt.query_map(params![account_name, database_name, container_name], |row| {
            Ok(CosmosItemMetadata {
                id: row.get(0)?,
                container_name: row.get(1)?,
                database_name: row.get(2)?,
                account_name: row.get(3)?,
                partition_key_value: row.get(4)?,
                item_json: row.get(5)?,
                last_modified: row.get(6)?,
                etag: row.get(7)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

        Ok(items)
    }
}
