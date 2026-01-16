use super::engine::{StorageEngine, StorageAccountMetadata, BlobContainerMetadata, BlobMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== Storage Account Operations ====================

    pub fn create_storage_account(&self, name: &str, location: &str, resource_group: &str) -> Result<()> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();

        db.execute(
            "INSERT INTO az_storage_accounts (name, location, resource_group, sku_name, kind, access_tier, created_at) VALUES (?, ?, ?, 'Standard_LRS', 'StorageV2', 'Hot', ?)",
            params![name, location, resource_group, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Storage account {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(())
    }

    pub fn get_storage_account(&self, name: &str) -> Result<StorageAccountMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT name, location, resource_group, sku_name, kind, access_tier, created_at FROM az_storage_accounts WHERE name = ?",
            params![name],
            |row| {
                Ok(StorageAccountMetadata {
                    name: row.get(0)?,
                    location: row.get(1)?,
                    resource_group: row.get(2)?,
                    sku_name: row.get(3)?,
                    kind: row.get(4)?,
                    access_tier: row.get(5)?,
                    created_at: row.get(6)?,
                })
            },
        ).map_err(|_| EmulatorError::NotFound("StorageAccount".into(), name.into()))
    }

    // ==================== Container Operations ====================

    pub fn create_container(&self, account_name: &str, name: &str) -> Result<()> {
        // Verify account exists or create it
        if let Err(EmulatorError::NotFound(_, _)) = self.get_storage_account(account_name) {
            self.create_storage_account(account_name, "local", "default")?;
        }

        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        let etag = format!("\"{}\"", uuid::Uuid::new_v4());

        db.execute(
            "INSERT INTO az_storage_containers (name, account_name, etag, last_modified) VALUES (?, ?, ?, ?)",
            params![name, account_name, etag, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Container {}/{} already exists", account_name, name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(())
    }

    pub fn get_container(&self, account_name: &str, name: &str) -> Result<BlobContainerMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT name, account_name, public_access, etag, last_modified FROM az_storage_containers WHERE account_name = ? AND name = ?",
            params![account_name, name],
            |row| {
                Ok(BlobContainerMetadata {
                    name: row.get(0)?,
                    account_name: row.get(1)?,
                    public_access: row.get(2)?,
                    etag: row.get(3)?,
                    last_modified: row.get(4)?,
                })
            },
        ).map_err(|_| EmulatorError::NotFound("BlobContainer".into(), format!("Container {}/{} not found", account_name, name)))
    }

    pub fn list_containers(&self, account_name: &str) -> Result<Vec<BlobContainerMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            "SELECT name, account_name, public_access, etag, last_modified FROM az_storage_containers WHERE account_name = ?"
        )?;
        
        let containers = stmt.query_map(params![account_name], |row| {
            Ok(BlobContainerMetadata {
                name: row.get(0)?,
                account_name: row.get(1)?,
                public_access: row.get(2)?,
                etag: row.get(3)?,
                last_modified: row.get(4)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

        Ok(containers)
    }

    // ==================== Blob Operations ====================

    pub fn put_blob(&self, account_name: &str, container_name: &str, blob_name: &str, data: &[u8], content_type: Option<&str>) -> Result<BlobMetadata> {
        // Verify container exists
        self.get_container(account_name, container_name)?;

        // Calculate hash and use common storage engine helper
        let content_hash = self.store_object_data(data)?;
        let etag = format!("\"{}\"", &content_hash[..32]);
        let now = chrono::Utc::now().to_rfc3339();

        let db = self.db.lock();
        
        // Upsert blob
        db.execute(
            r#"INSERT OR REPLACE INTO az_blobs 
               (name, container_name, account_name, content_hash, content_length, content_type, etag, last_modified)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            params![
                blob_name,
                container_name,
                account_name,
                content_hash,
                data.len() as i64,
                content_type.unwrap_or("application/octet-stream"),
                etag,
                now,
            ],
        )?;

        Ok(BlobMetadata {
            name: blob_name.to_string(),
            container_name: container_name.to_string(),
            account_name: account_name.to_string(),
            blob_type: "BlockBlob".to_string(),
            access_tier: "Hot".to_string(),
            size: data.len() as u64,
            content_type: content_type.map(|s| s.to_string()),
            etag,
            last_modified: now,
        })
    }

    pub fn get_blob(&self, account_name: &str, container_name: &str, blob_name: &str) -> Result<(BlobMetadata, Vec<u8>)> {
        let db = self.db.lock();
        
        let (metadata, content_hash) = db.query_row(
            r#"SELECT name, container_name, account_name, blob_type, access_tier, content_length, content_type, etag, last_modified, content_hash
               FROM az_blobs WHERE account_name = ? AND container_name = ? AND name = ?"#,
            params![account_name, container_name, blob_name],
            |row| {
                Ok((
                    BlobMetadata {
                        name: row.get(0)?,
                        container_name: row.get(1)?,
                        account_name: row.get(2)?,
                        blob_type: row.get(3)?,
                        access_tier: row.get(4)?,
                        size: row.get::<_, i64>(5)? as u64,
                        content_type: row.get(6)?,
                        etag: row.get(7)?,
                        last_modified: row.get(8)?,
                    },
                    row.get::<_, String>(9)?,
                ))
            },
        ).map_err(|_| EmulatorError::NotFound("Blob".into(), format!("Blob {}/{}/{} not found", account_name, container_name, blob_name)))?;

        drop(db); // Release lock

        let data = self.read_object_data(&content_hash)?;
        Ok((metadata, data))
    }

    pub fn list_blobs(&self, account_name: &str, container_name: &str) -> Result<Vec<BlobMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            r#"SELECT name, container_name, account_name, blob_type, access_tier, content_length, content_type, etag, last_modified
               FROM az_blobs WHERE account_name = ? AND container_name = ?"#
        )?;

        let blobs = stmt.query_map(params![account_name, container_name], |row| {
            Ok(BlobMetadata {
                name: row.get(0)?,
                container_name: row.get(1)?,
                account_name: row.get(2)?,
                blob_type: row.get(3)?,
                access_tier: row.get(4)?,
                size: row.get::<_, i64>(5)? as u64,
                content_type: row.get(6)?,
                etag: row.get(7)?,
                last_modified: row.get(8)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

        Ok(blobs)
    }
}
