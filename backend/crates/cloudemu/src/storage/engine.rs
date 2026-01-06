//! Storage engine implementation

use crate::config::Config;
use crate::error::{EmulatorError, Result};
use rusqlite::{Connection, params};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::Mutex;
use super::schema::SCHEMA;
use serde_json;

/// Storage engine with SQLite for metadata and filesystem for objects
#[derive(Clone)]
pub struct StorageEngine {
    /// SQLite connection (wrapped for thread safety)
    db: Arc<Mutex<Connection>>,
    /// Directory for object data
    objects_dir: PathBuf,
}

impl StorageEngine {
    /// Create a new storage engine
    pub fn new(config: &Config) -> Result<Self> {
        // Create data directory
        fs::create_dir_all(&config.data_dir)?;
        
        // Create objects directory
        let objects_dir = config.data_dir.join("objects");
        fs::create_dir_all(&objects_dir)?;
        
        // Open SQLite database
        let db_path = config.data_dir.join("metadata.db");
        let conn = Connection::open(&db_path)?;
        
        // Enable WAL mode for better concurrency
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")?;
        
        // Create schema
        conn.execute_batch(SCHEMA)?;
        
        Ok(Self {
            db: Arc::new(Mutex::new(conn)),
            objects_dir,
        })
    }
    
    /// Create a new in-memory storage engine (for testing)
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(SCHEMA)?;
        
        let temp_dir = std::env::temp_dir().join(format!("cloudemu-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir)?;
        
        Ok(Self {
            db: Arc::new(Mutex::new(conn)),
            objects_dir: temp_dir,
        })
    }
    
    // ==================== Bucket Operations ====================
    
    /// Create a bucket
    pub fn create_bucket(&self, name: &str, region: &str) -> Result<()> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            "INSERT INTO buckets (name, region, created_at) VALUES (?1, ?2, ?3)",
            params![name, region, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::BucketAlreadyExists(name.to_string())
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;
        
        Ok(())
    }
    
    /// Delete a bucket
    pub fn delete_bucket(&self, name: &str) -> Result<()> {
        let db = self.db.lock();
        
        // Check if bucket exists
        let exists: bool = db.query_row(
            "SELECT 1 FROM buckets WHERE name = ?1",
            params![name],
            |_| Ok(true),
        ).unwrap_or(false);
        
        if !exists {
            return Err(EmulatorError::NoSuchBucket(name.to_string()));
        }
        
        // Check if bucket is empty
        let count: i64 = db.query_row(
            "SELECT COUNT(*) FROM objects WHERE bucket = ?1",
            params![name],
            |row| row.get(0),
        )?;
        
        if count > 0 {
            return Err(EmulatorError::BucketNotEmpty(name.to_string()));
        }
        
        db.execute("DELETE FROM buckets WHERE name = ?1", params![name])?;
        
        Ok(())
    }
    
    /// Check if bucket exists
    pub fn bucket_exists(&self, name: &str) -> Result<bool> {
        let db = self.db.lock();
        let exists: bool = db.query_row(
            "SELECT 1 FROM buckets WHERE name = ?1",
            params![name],
            |_| Ok(true),
        ).unwrap_or(false);
        Ok(exists)
    }
    
    /// Get bucket metadata
    pub fn get_bucket(&self, name: &str) -> Result<BucketMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT name, region, created_at, versioning, policy, acl FROM buckets WHERE name = ?1",
            params![name],
            |row| {
                Ok(BucketMetadata {
                    name: row.get(0)?,
                    region: row.get(1)?,
                    created_at: row.get(2)?,
                    versioning: row.get(3)?,
                    policy: row.get(4)?,
                    acl: row.get(5)?,
                })
            },
        ).map_err(|_| EmulatorError::NoSuchBucket(name.to_string()))
    }
    
    /// List all buckets
    pub fn list_buckets(&self) -> Result<Vec<BucketMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            "SELECT name, region, created_at, versioning, policy, acl FROM buckets ORDER BY name"
        )?;
        
        let buckets = stmt.query_map([], |row| {
            Ok(BucketMetadata {
                name: row.get(0)?,
                region: row.get(1)?,
                created_at: row.get(2)?,
                versioning: row.get(3)?,
                policy: row.get(4)?,
                acl: row.get(5)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
        
        Ok(buckets)
    }
    
    /// Set bucket versioning
    pub fn set_bucket_versioning(&self, name: &str, status: &str) -> Result<()> {
        let db = self.db.lock();
        let rows = db.execute(
            "UPDATE buckets SET versioning = ?1 WHERE name = ?2",
            params![status, name],
        )?;
        
        if rows == 0 {
            return Err(EmulatorError::NoSuchBucket(name.to_string()));
        }
        
        Ok(())
    }
    
    /// Get bucket versioning status
    pub fn get_bucket_versioning(&self, name: &str) -> Result<String> {
        let db = self.db.lock();
        db.query_row(
            "SELECT versioning FROM buckets WHERE name = ?1",
            params![name],
            |row| row.get(0),
        ).map_err(|_| EmulatorError::NoSuchBucket(name.to_string()))
    }
    
    /// Set bucket policy
    pub fn set_bucket_policy(&self, name: &str, policy: &str) -> Result<()> {
        let db = self.db.lock();
        let rows = db.execute(
            "UPDATE buckets SET policy = ?1 WHERE name = ?2",
            params![policy, name],
        )?;
        
        if rows == 0 {
            return Err(EmulatorError::NoSuchBucket(name.to_string()));
        }
        
        Ok(())
    }
    
    /// Get bucket policy
    pub fn get_bucket_policy(&self, name: &str) -> Result<Option<String>> {
        let db = self.db.lock();
        db.query_row(
            "SELECT policy FROM buckets WHERE name = ?1",
            params![name],
            |row| row.get(0),
        ).map_err(|_| EmulatorError::NoSuchBucket(name.to_string()))
    }
    
    /// Delete bucket policy
    pub fn delete_bucket_policy(&self, name: &str) -> Result<()> {
        let db = self.db.lock();
        let rows = db.execute(
            "UPDATE buckets SET policy = NULL WHERE name = ?1",
            params![name],
        )?;
        
        if rows == 0 {
            return Err(EmulatorError::NoSuchBucket(name.to_string()));
        }
        
        Ok(())
    }
    
    // ==================== Object Operations ====================
    
    /// Put an object
    pub fn put_object(&self, bucket: &str, key: &str, data: &[u8], content_type: Option<&str>, metadata: Option<&str>) -> Result<ObjectMetadata> {
        // Check bucket exists
        if !self.bucket_exists(bucket)? {
            return Err(EmulatorError::NoSuchBucket(bucket.to_string()));
        }
        
        // Calculate hash and ETag
        let content_hash = self.store_object_data(data)?;
        let etag = format!("\"{}\"", &content_hash[..32]);
        let now = chrono::Utc::now().to_rfc3339();
        
        // Get versioning status
        let versioning = self.get_bucket_versioning(bucket)?;
        let version_id = if versioning == "Enabled" {
            Some(uuid::Uuid::new_v4().to_string())
        } else {
            None
        };
        
        let db = self.db.lock();
        
        // If versioning is not enabled, delete existing object
        if versioning != "Enabled" {
            db.execute(
                "DELETE FROM objects WHERE bucket = ?1 AND key = ?2",
                params![bucket, key],
            )?;
        } else {
            // Mark previous version as not latest
            db.execute(
                "UPDATE objects SET is_latest = 0 WHERE bucket = ?1 AND key = ?2 AND is_latest = 1",
                params![bucket, key],
            )?;
        }
        
        // Insert new object
        db.execute(
            r#"INSERT INTO objects 
               (bucket, key, version_id, is_latest, content_hash, content_length, content_type, etag, last_modified, metadata)
               VALUES (?1, ?2, ?3, 1, ?4, ?5, ?6, ?7, ?8, ?9)"#,
            params![
                bucket,
                key,
                version_id,
                content_hash,
                data.len() as i64,
                content_type.unwrap_or("application/octet-stream"),
                etag,
                now,
                metadata,
            ],
        )?;
        
        Ok(ObjectMetadata {
            key: key.to_string(),
            version_id,
            etag,
            size: data.len() as u64,
            last_modified: now,
            content_type: content_type.unwrap_or("application/octet-stream").to_string(),
            storage_class: "STANDARD".to_string(),
            is_delete_marker: false,
        })
    }
    
    /// Get an object
    pub fn get_object(&self, bucket: &str, key: &str, version_id: Option<&str>) -> Result<(ObjectMetadata, Vec<u8>)> {
        let db = self.db.lock();
        
        let query = if let Some(vid) = version_id {
            db.query_row(
                r#"SELECT key, version_id, etag, content_length, last_modified, content_type, storage_class, is_delete_marker, content_hash
                   FROM objects WHERE bucket = ?1 AND key = ?2 AND version_id = ?3"#,
                params![bucket, key, vid],
                |row| Ok((
                    ObjectMetadata {
                        key: row.get(0)?,
                        version_id: row.get(1)?,
                        etag: row.get(2)?,
                        size: row.get::<_, i64>(3)? as u64,
                        last_modified: row.get(4)?,
                        content_type: row.get(5)?,
                        storage_class: row.get(6)?,
                        is_delete_marker: row.get::<_, i64>(7)? != 0,
                    },
                    row.get::<_, String>(8)?,
                ))
            )
        } else {
            db.query_row(
                r#"SELECT key, version_id, etag, content_length, last_modified, content_type, storage_class, is_delete_marker, content_hash
                   FROM objects WHERE bucket = ?1 AND key = ?2 AND is_latest = 1"#,
                params![bucket, key],
                |row| Ok((
                    ObjectMetadata {
                        key: row.get(0)?,
                        version_id: row.get(1)?,
                        etag: row.get(2)?,
                        size: row.get::<_, i64>(3)? as u64,
                        last_modified: row.get(4)?,
                        content_type: row.get(5)?,
                        storage_class: row.get(6)?,
                        is_delete_marker: row.get::<_, i64>(7)? != 0,
                    },
                    row.get::<_, String>(8)?,
                ))
            )
        };
        
        let (metadata, content_hash) = query.map_err(|_| EmulatorError::NoSuchKey(key.to_string()))?;
        
        if metadata.is_delete_marker {
            return Err(EmulatorError::NoSuchKey(key.to_string()));
        }
        
        drop(db); // Release lock before reading file
        
        let data = self.read_object_data(&content_hash)?;
        
        Ok((metadata, data))
    }
    
    /// Delete an object
    pub fn delete_object(&self, bucket: &str, key: &str, version_id: Option<&str>) -> Result<Option<String>> {
        if !self.bucket_exists(bucket)? {
            return Err(EmulatorError::NoSuchBucket(bucket.to_string()));
        }
        
        let versioning = self.get_bucket_versioning(bucket)?;
        let db = self.db.lock();
        
        if versioning == "Enabled" && version_id.is_none() {
            // Insert delete marker
            let delete_marker_version = uuid::Uuid::new_v4().to_string();
            let now = chrono::Utc::now().to_rfc3339();
            
            // Mark previous as not latest
            db.execute(
                "UPDATE objects SET is_latest = 0 WHERE bucket = ?1 AND key = ?2 AND is_latest = 1",
                params![bucket, key],
            )?;
            
            // Insert delete marker
            db.execute(
                r#"INSERT INTO objects 
                   (bucket, key, version_id, is_latest, is_delete_marker, content_hash, content_length, etag, last_modified)
                   VALUES (?1, ?2, ?3, 1, 1, '', 0, '', ?4)"#,
                params![bucket, key, delete_marker_version, now],
            )?;
            
            Ok(Some(delete_marker_version))
        } else if let Some(vid) = version_id {
            // Delete specific version
            db.execute(
                "DELETE FROM objects WHERE bucket = ?1 AND key = ?2 AND version_id = ?3",
                params![bucket, key, vid],
            )?;
            Ok(None)
        } else {
            // Delete object (no versioning)
            db.execute(
                "DELETE FROM objects WHERE bucket = ?1 AND key = ?2",
                params![bucket, key],
            )?;
            Ok(None)
        }
    }
    
    /// List objects in a bucket
    pub fn list_objects(&self, bucket: &str, prefix: Option<&str>, delimiter: Option<&str>, max_keys: u32, continuation_token: Option<&str>) -> Result<ListObjectsResult> {
        if !self.bucket_exists(bucket)? {
            return Err(EmulatorError::NoSuchBucket(bucket.to_string()));
        }
        
        let db = self.db.lock();
        let prefix_str = prefix.unwrap_or("");
        let start_after = continuation_token.unwrap_or("");
        
        // Query objects
        let mut stmt = db.prepare(
            r#"SELECT key, version_id, etag, content_length, last_modified, content_type, storage_class
               FROM objects 
               WHERE bucket = ?1 AND is_latest = 1 AND is_delete_marker = 0 AND key LIKE ?2 AND key > ?3
               ORDER BY key
               LIMIT ?4"#
        )?;
        
        let like_pattern = format!("{}%", prefix_str);
        let objects: Vec<ObjectMetadata> = stmt.query_map(
            params![bucket, like_pattern, start_after, max_keys + 1],
            |row| Ok(ObjectMetadata {
                key: row.get(0)?,
                version_id: row.get(1)?,
                etag: row.get(2)?,
                size: row.get::<_, i64>(3)? as u64,
                last_modified: row.get(4)?,
                content_type: row.get(5)?,
                storage_class: row.get(6)?,
                is_delete_marker: false,
            })
        )?
        .filter_map(|r| r.ok())
        .collect();
        
        let is_truncated = objects.len() > max_keys as usize;
        let mut contents = objects;
        if is_truncated {
            contents.truncate(max_keys as usize);
        }
        
        // Handle delimiter for common prefixes
        let mut common_prefixes = Vec::new();
        if let Some(delim) = delimiter {
            let mut seen_prefixes = std::collections::HashSet::new();
            contents.retain(|obj| {
                let after_prefix = &obj.key[prefix_str.len()..];
                if let Some(pos) = after_prefix.find(delim) {
                    let common_prefix = format!("{}{}{}", prefix_str, &after_prefix[..pos], delim);
                    if seen_prefixes.insert(common_prefix.clone()) {
                        common_prefixes.push(common_prefix);
                    }
                    false
                } else {
                    true
                }
            });
        }
        
        let next_token = if is_truncated {
            contents.last().map(|o| o.key.clone())
        } else {
            None
        };
        
        Ok(ListObjectsResult {
            name: bucket.to_string(),
            prefix: prefix.map(|s| s.to_string()),
            delimiter: delimiter.map(|s| s.to_string()),
            max_keys,
            is_truncated,
            contents,
            common_prefixes,
            continuation_token: continuation_token.map(|s| s.to_string()),
            next_continuation_token: next_token,
        })
    }
    
    // ==================== KMS Operations ====================

    pub fn create_key(&self, description: Option<&str>, key_usage: &str, tags: Option<&str>, account_id: &str, region: &str) -> Result<KmsKeyMetadata> {
        let db = self.db.lock();
        let key_id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:kms:{}:{}:key/{}", region, account_id, key_id);
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            "INSERT INTO kms_keys (id, arn, description, key_usage, created_at, tags) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![key_id, arn, description, key_usage, now, tags],
        )?;
        
        Ok(KmsKeyMetadata {
            id: key_id,
            arn,
            description: description.map(|s| s.to_string()),
            key_usage: key_usage.to_string(),
            key_state: "Enabled".to_string(),
            created_at: now,
            tags: tags.map(|s| s.to_string()),
        })
    }
    
    pub fn get_key(&self, key_id: &str) -> Result<KmsKeyMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT id, arn, description, key_usage, key_state, created_at, tags FROM kms_keys WHERE id = ?1 OR arn = ?1",
            params![key_id],
            |row| Ok(KmsKeyMetadata {
                id: row.get(0)?,
                arn: row.get(1)?,
                description: row.get(2)?,
                key_usage: row.get(3)?,
                key_state: row.get(4)?,
                created_at: row.get(5)?,
                tags: row.get(6)?,
            })
        ).map_err(|_| EmulatorError::NotFound("Key".into(), key_id.into()))
    }
    
    pub fn enable_key(&self, key_id: &str) -> Result<()> {
         self.set_key_state(key_id, "Enabled")
    }
    
    pub fn disable_key(&self, key_id: &str) -> Result<()> {
         self.set_key_state(key_id, "Disabled")
    }
    
    fn set_key_state(&self, key_id: &str, state: &str) -> Result<()> {
        let db = self.db.lock();
        let rows = db.execute(
             "UPDATE kms_keys SET key_state = ?1 WHERE id = ?2 OR arn = ?2",
             params![state, key_id]
        )?;
        if rows == 0 {
             return Err(EmulatorError::NotFound("Key".into(), key_id.into()));
        }
        Ok(())
    }

    pub fn list_keys(&self) -> Result<Vec<KmsKeyMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT id, arn, description, key_usage, key_state, created_at, tags FROM kms_keys")
            .map_err(|e| EmulatorError::Database(e.to_string()))?;
            
        let keys = stmt.query_map([], |row| Ok(KmsKeyMetadata {
            id: row.get(0)?,
            arn: row.get(1)?,
            description: row.get(2)?,
            key_usage: row.get(3)?,
            key_state: row.get(4)?,
            created_at: row.get(5)?,
            tags: row.get(6)?,
        }))
        .map_err(|e| EmulatorError::Database(e.to_string()))?
        .filter_map(|r| r.ok())
        .collect();
        
        Ok(keys)
    }

    // ==================== Secrets Operations ====================

    /// Create a secret
    pub fn create_secret(&self, name: &str, description: Option<&str>, tags: Option<&str>, account_id: &str, region: &str) -> Result<SecretMetadata> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        let arn = format!("arn:aws:secretsmanager:{}:{}:secret:{}", region, account_id, name);

        db.execute(
            "INSERT INTO secrets (arn, name, description, created_at, last_changed_date, tags) VALUES (?1, ?2, ?3, ?4, ?4, ?5)",
            params![arn, name, description, now, tags],
        ).map_err(|e| {
             if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Secret {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(SecretMetadata {
            arn,
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            created_at: now.clone(),
            last_changed_date: Some(now),
            tags: tags.map(|s| s.to_string()),
        })
    }
    
    /// Put secret value
    pub fn put_secret_value(&self, secret_id: &str, secret_string: Option<&str>, secret_binary: Option<&[u8]>) -> Result<(String, String)> {
        let db = self.db.lock();
        
        let arn: String = db.query_row(
            "SELECT arn FROM secrets WHERE name = ?1 OR arn = ?1",
            params![secret_id],
            |row| row.get(0),
        ).map_err(|_| EmulatorError::NotFound("Secret".into(), secret_id.into()))?;
        
        let version_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let stages = "[\"AWSCURRENT\"]";
        
        db.execute(
            "UPDATE secret_versions SET version_stages = '[]' WHERE secret_arn = ?1",
            params![arn],
        )?;

        db.execute(
            "INSERT INTO secret_versions (secret_arn, version_id, version_stages, secret_string, secret_binary, created_date) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![arn, version_id, stages, secret_string, secret_binary, now],
        )?;
        
        Ok((arn, version_id))
    }

    /// Get secret value
    pub fn get_secret_value(&self, secret_id: &str, version_id: Option<&str>, _version_stage: Option<&str>) -> Result<SecretValue> {
        let db = self.db.lock();
        let (arn, name): (String, String) = db.query_row(
            "SELECT arn, name FROM secrets WHERE name = ?1 OR arn = ?1",
            params![secret_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ).map_err(|_| EmulatorError::NotFound("Secret".into(), secret_id.into()))?;

        let map_row = |row: &rusqlite::Row| -> rusqlite::Result<SecretValue> {
             let stages_str: String = row.get(3)?;
             let stages: Vec<String> = serde_json::from_str(&stages_str).unwrap_or_default();
             Ok(SecretValue {
                 arn: arn.clone(),
                 name: name.clone(),
                 version_id: row.get(0)?,
                 secret_string: row.get(1)?,
                 secret_binary: row.get(2)?,
                 version_stages: stages,
                 created_date: row.get(4)?,
             }) 
        };

        if let Some(vid) = version_id {
             db.query_row(
                "SELECT version_id, secret_string, secret_binary, version_stages, created_date FROM secret_versions WHERE secret_arn = ?1 AND version_id = ?2", 
                params![arn, vid], 
                map_row
            )
        } else {
             db.query_row(
                "SELECT version_id, secret_string, secret_binary, version_stages, created_date FROM secret_versions WHERE secret_arn = ?1 AND version_stages LIKE '%AWSCURRENT%'", 
                params![arn], 
                map_row
            )
        }.map_err(|_| EmulatorError::NotFound("SecretVersion".into(), "current".into()))
    }

    // ==================== Object Data Storage ====================
    
    /// Store object data to filesystem, returns content hash
    fn store_object_data(&self, data: &[u8]) -> Result<String> {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hex::encode(hasher.finalize());
        
        // Content-addressed storage: first 2 chars as directory
        let dir = self.objects_dir.join(&hash[..2]);
        fs::create_dir_all(&dir)?;
        
        let file_path = dir.join(&hash);
        if !file_path.exists() {
            fs::write(&file_path, data)?;
        }
        
        Ok(hash)
    }
    
    /// Read object data from filesystem
    fn read_object_data(&self, content_hash: &str) -> Result<Vec<u8>> {
        if content_hash.is_empty() {
            return Ok(Vec::new());
        }
        
        let file_path = self.objects_dir.join(&content_hash[..2]).join(content_hash);
        fs::read(&file_path).map_err(|e| EmulatorError::Internal(e.to_string()))
    }
}

/// Bucket metadata
#[derive(Debug, Clone)]
pub struct BucketMetadata {
    pub name: String,
    pub region: String,
    pub created_at: String,
    pub versioning: String,
    pub policy: Option<String>,
    pub acl: Option<String>,
}

/// Object metadata
#[derive(Debug, Clone)]
pub struct ObjectMetadata {
    pub key: String,
    pub version_id: Option<String>,
    pub etag: String,
    pub size: u64,
    pub last_modified: String,
    pub content_type: String,
    pub storage_class: String,
    pub is_delete_marker: bool,
}

/// List objects result
#[derive(Debug, Clone)]
pub struct ListObjectsResult {
    pub name: String,
    pub prefix: Option<String>,
    pub delimiter: Option<String>,
    pub max_keys: u32,
    pub is_truncated: bool,
    pub contents: Vec<ObjectMetadata>,
    pub common_prefixes: Vec<String>,
    pub continuation_token: Option<String>,
    pub next_continuation_token: Option<String>,
}

/// Secret metadata
#[derive(Debug, Clone)]
pub struct SecretMetadata {
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub last_changed_date: Option<String>,
    pub tags: Option<String>,
}

/// Secret value
#[derive(Debug, Clone)]
pub struct SecretValue {
    pub arn: String,
    pub name: String,
    pub version_id: String,
    pub secret_string: Option<String>,
    pub secret_binary: Option<Vec<u8>>,
    pub version_stages: Vec<String>,
    pub created_date: String,
}

/// KMS Key metadata
#[derive(Debug, Clone)]
pub struct KmsKeyMetadata {
    pub id: String,
    pub arn: String,
    pub description: Option<String>,
    pub key_usage: String,
    pub key_state: String,
    pub created_at: String,
    pub tags: Option<String>,
}
