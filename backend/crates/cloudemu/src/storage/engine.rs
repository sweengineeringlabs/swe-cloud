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

    // ==================== Step Functions Operations ====================
    
    pub fn create_state_machine(&self, name: &str, definition: &str, role_arn: &str, machine_type: &str, account_id: &str, region: &str) -> Result<StateMachineMetadata> {
        let db = self.db.lock();
        let arn = format!("arn:aws:states:{}:{}:stateMachine:{}", region, account_id, name);
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            "INSERT INTO sf_state_machines (arn, name, definition, role_arn, type, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![arn, name, definition, role_arn, machine_type, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("State machine {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;
        
        Ok(StateMachineMetadata {
            arn,
            name: name.to_string(),
            definition: definition.to_string(),
            role_arn: role_arn.to_string(),
            machine_type: machine_type.to_string(),
            created_at: now,
        })
    }

    pub fn list_state_machines(&self) -> Result<Vec<StateMachineMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT arn, name, definition, role_arn, type, created_at FROM sf_state_machines")?;
        let machines = stmt.query_map([], |row| Ok(StateMachineMetadata {
            arn: row.get(0)?,
            name: row.get(1)?,
            definition: row.get(2)?,
            role_arn: row.get(3)?,
            machine_type: row.get(4)?,
            created_at: row.get(5)?,
        }))?
        .filter_map(|r| r.ok())
        .collect();
        Ok(machines)
    }

    pub fn delete_state_machine(&self, arn: &str) -> Result<()> {
        let db = self.db.lock();
        let rows = db.execute("DELETE FROM sf_state_machines WHERE arn = ?1", params![arn])?;
        if rows == 0 {
            return Err(EmulatorError::NotFound("StateMachine".into(), arn.into()));
        }
        Ok(())
    }

    pub fn start_execution(&self, state_machine_arn: &str, name: Option<&str>, input: Option<&str>, account_id: &str, region: &str) -> Result<ExecutionMetadata> {
        let db = self.db.lock();
        let exec_name = name.map(|s| s.to_string()).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = format!("arn:aws:states:{}:{}:execution:{}:{}", region, account_id, state_machine_arn.split(':').last().unwrap_or("unknown"), exec_name);
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            "INSERT INTO sf_executions (arn, state_machine_arn, name, input, start_date) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![arn, state_machine_arn, exec_name, input, now],
        )?;
        
        Ok(ExecutionMetadata {
            arn,
            state_machine_arn: state_machine_arn.to_string(),
            name: exec_name,
            status: "RUNNING".to_string(),
            input: input.map(|s| s.to_string()),
            output: None,
            start_date: now,
            stop_date: None,
        })
    }

    pub fn describe_execution(&self, arn: &str) -> Result<ExecutionMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT arn, state_machine_arn, name, status, input, output, start_date, stop_date FROM sf_executions WHERE arn = ?1",
            params![arn],
            |row| Ok(ExecutionMetadata {
                arn: row.get(0)?,
                state_machine_arn: row.get(1)?,
                name: row.get(2)?,
                status: row.get(3)?,
                input: row.get(4)?,
                output: row.get(5)?,
                start_date: row.get(6)?,
                stop_date: row.get(7)?,
            })
        ).map_err(|_| EmulatorError::NotFound("Execution".into(), arn.into()))
    }

    // ==================== Cognito Operations ====================
    
    pub fn create_user_pool(&self, name: &str, account_id: &str, region: &str) -> Result<UserPoolMetadata> {
        let db = self.db.lock();
        let pool_id = format!("{}_{}", region, uuid::Uuid::new_v4().to_string().replace("-", ""));
        let arn = format!("arn:aws:cognito-idp:{}:{}:userpool/{}", region, account_id, pool_id);
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            "INSERT INTO cognito_user_pools (id, name, arn, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![pool_id, name, arn, now],
        )?;
        
        Ok(UserPoolMetadata {
            id: pool_id,
            name: name.to_string(),
            arn,
            created_at: now,
        })
    }

    pub fn list_user_pools(&self) -> Result<Vec<UserPoolMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT id, name, arn, created_at FROM cognito_user_pools")?;
        let pools = stmt.query_map([], |row| Ok(UserPoolMetadata {
            id: row.get(0)?,
            name: row.get(1)?,
            arn: row.get(2)?,
            created_at: row.get(3)?,
        }))?
        .filter_map(|r| r.ok())
        .collect();
        Ok(pools)
    }

    pub fn admin_create_user(&self, pool_id: &str, username: &str, attributes: Vec<(String, String)>) -> Result<UserMetadata> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        
        let email = attributes.iter().find(|(n, _)| n == "email").map(|(_, v)| v.to_string());
        
        db.execute(
            "INSERT INTO cognito_users (user_pool_id, username, email, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![pool_id, username, email, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("User {} already exists in pool {}", username, pool_id))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;
        
        for (name, value) in attributes {
            db.execute(
                "INSERT INTO cognito_user_attributes (user_pool_id, username, name, value) VALUES (?1, ?2, ?3, ?4)",
                params![pool_id, username, name, value],
            )?;
        }
        
        Ok(UserMetadata {
            user_pool_id: pool_id.to_string(),
            username: username.to_string(),
            email,
            status: "CONFIRMED".to_string(),
            enabled: true,
            created_at: now,
        })
    }

    pub fn admin_get_user(&self, pool_id: &str, username: &str) -> Result<(UserMetadata, Vec<(String, String)>)> {
        let db = self.db.lock();
        let user = db.query_row(
            "SELECT user_pool_id, username, email, status, enabled, created_at FROM cognito_users WHERE user_pool_id = ?1 AND username = ?2",
            params![pool_id, username],
            |row| Ok(UserMetadata {
                user_pool_id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                status: row.get(3)?,
                enabled: row.get(4)?,
                created_at: row.get(5)?,
            })
        ).map_err(|_| EmulatorError::NotFound("User".into(), username.into()))?;
        
        let mut stmt = db.prepare("SELECT name, value FROM cognito_user_attributes WHERE user_pool_id = ?1 AND username = ?2")?;
        let attrs = stmt.query_map(params![pool_id, username], |row| Ok((row.get(0)?, row.get(1)?)))?
            .filter_map(|r| r.ok())
            .collect();
            
        Ok((user, attrs))
    }

    pub fn create_group(&self, pool_id: &str, group_name: &str, description: Option<&str>, precedence: Option<i32>) -> Result<UserGroupMetadata> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            "INSERT INTO cognito_groups (user_pool_id, group_name, description, precedence, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![pool_id, group_name, description, precedence, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Group {} already exists in pool {}", group_name, pool_id))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;
        
        Ok(UserGroupMetadata {
            user_pool_id: pool_id.to_string(),
            group_name: group_name.to_string(),
            description: description.map(|s| s.to_string()),
            precedence,
            created_at: now,
        })
    }

    pub fn list_groups(&self, pool_id: &str) -> Result<Vec<UserGroupMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT user_pool_id, group_name, description, precedence, created_at FROM cognito_groups WHERE user_pool_id = ?1")?;
        let groups = stmt.query_map(params![pool_id], |row| Ok(UserGroupMetadata {
            user_pool_id: row.get(0)?,
            group_name: row.get(1)?,
            description: row.get(2)?,
            precedence: row.get(3)?,
            created_at: row.get(4)?,
        }))?
        .filter_map(|r| r.ok())
        .collect();
        Ok(groups)
    }

    // ==================== CloudWatch Operations ====================
    
    pub fn put_metric_data(&self, namespace: &str, metrics: Vec<MetricMetadata>) -> Result<()> {
        let db = self.db.lock();
        for m in metrics {
            db.execute(
                "INSERT INTO cw_metrics (namespace, metric_name, dimensions, value, unit, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![namespace, m.metric_name, m.dimensions, m.value, m.unit, m.timestamp],
            )?;
        }
        Ok(())
    }

    pub fn list_metrics(&self, namespace: Option<&str>, metric_name: Option<&str>) -> Result<Vec<MetricMetadata>> {
        let db = self.db.lock();
        let mut query = "SELECT namespace, metric_name, dimensions, value, unit, timestamp FROM cw_metrics WHERE 1=1".to_string();
        let mut args: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(ns) = namespace {
            query.push_str(" AND namespace = ?");
            args.push(Box::new(ns.to_string()));
        }
        if let Some(name) = metric_name {
            query.push_str(" AND metric_name = ?");
            args.push(Box::new(name.to_string()));
        }

        let mut stmt = db.prepare(&query)?;
        
        // Convert Vec<Box<dyn ToSql>> to a slice of &dyn ToSql
        let params_refs: Vec<&dyn rusqlite::ToSql> = args.iter().map(|b| b.as_ref()).collect();

        let metrics = stmt.query_map(rusqlite::params_from_iter(params_refs), |row| Ok(MetricMetadata {
            namespace: row.get(0)?,
            metric_name: row.get(1)?,
            dimensions: row.get(2)?,
            value: row.get(3)?,
            unit: row.get(4)?,
            timestamp: row.get(5)?,
        }))?
        .filter_map(|r| r.ok())
        .collect();

        Ok(metrics)
    }

    pub fn create_log_group(&self, name: &str, account_id: &str, region: &str) -> Result<LogGroupMetadata> {
        let db = self.db.lock();
        let arn = format!("arn:aws:logs:{}:{}:log-group:{}", region, account_id, name);
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            "INSERT INTO cw_log_groups (name, arn, created_at) VALUES (?1, ?2, ?3)",
            params![name, arn, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Log group {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;
        
        Ok(LogGroupMetadata {
            name: name.to_string(),
            arn,
            retention_days: None,
            created_at: now,
        })
    }

    pub fn delete_log_group(&self, name: &str) -> Result<()> {
        let db = self.db.lock();
        let rows = db.execute("DELETE FROM cw_log_groups WHERE name = ?1", params![name])?;
        if rows == 0 {
            return Err(EmulatorError::NotFound("LogGroup".into(), name.into()));
        }
        Ok(())
    }

    pub fn create_log_stream(&self, group_name: &str, stream_name: &str, account_id: &str, region: &str) -> Result<LogStreamMetadata> {
        let db = self.db.lock();
        let arn = format!("arn:aws:logs:{}:{}:log-group:{}:log-stream:{}", region, account_id, group_name, stream_name);
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            "INSERT INTO cw_log_streams (name, log_group_name, arn, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![stream_name, group_name, arn, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Log stream {} already exists in group {}", stream_name, group_name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;
        
        Ok(LogStreamMetadata {
            name: stream_name.to_string(),
            log_group_name: group_name.to_string(),
            arn,
            created_at: now,
        })
    }

    pub fn put_log_events(&self, group_name: &str, stream_name: &str, events: Vec<LogEventMetadata>) -> Result<()> {
        let db = self.db.lock();
        for e in events {
            db.execute(
                "INSERT INTO cw_log_events (log_group_name, log_stream_name, timestamp, message) VALUES (?1, ?2, ?3, ?4)",
                params![group_name, stream_name, e.timestamp, e.message],
            )?;
        }
        Ok(())
    }

    pub fn get_log_events(&self, group_name: &str, stream_name: &str) -> Result<Vec<LogEventMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            "SELECT timestamp, message FROM cw_log_events WHERE log_group_name = ?1 AND log_stream_name = ?2 ORDER BY timestamp"
        )?;
        let events = stmt.query_map(params![group_name, stream_name], |row| Ok(LogEventMetadata {
            timestamp: row.get(0)?,
            message: row.get(1)?,
        }))?
        .filter_map(|r| r.ok())
        .collect();
        Ok(events)
    }

    // ==================== EventBridge Operations ====================
    
    pub fn create_event_bus(&self, name: &str, account_id: &str, region: &str) -> Result<EventBusMetadata> {
        let db = self.db.lock();
        let arn = format!("arn:aws:events:{}:{}:event-bus/{}", region, account_id, name);
        
        db.execute(
            "INSERT INTO event_buses (name, arn) VALUES (?1, ?2)",
            params![name, arn],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Event bus {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;
        
        Ok(EventBusMetadata {
            name: name.to_string(),
            arn,
            policy: None,
        })
    }
    
    pub fn get_event_bus(&self, name: &str) -> Result<EventBusMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT name, arn, policy FROM event_buses WHERE name = ?1",
            params![name],
            |row| Ok(EventBusMetadata {
                name: row.get(0)?,
                arn: row.get(1)?,
                policy: row.get(2)?,
            })
        ).map_err(|_| EmulatorError::NotFound("EventBus".into(), name.into()))
    }
    
    pub fn list_event_buses(&self) -> Result<Vec<EventBusMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT name, arn, policy FROM event_buses")?;
        let buses = stmt.query_map([], |row| Ok(EventBusMetadata {
            name: row.get(0)?,
            arn: row.get(1)?,
            policy: row.get(2)?,
        }))?
        .filter_map(|r| r.ok())
        .collect();
        Ok(buses)
    }
    
    pub fn delete_event_bus(&self, name: &str) -> Result<()> {
        let db = self.db.lock();
        let rows = db.execute("DELETE FROM event_buses WHERE name = ?1", params![name])?;
        if rows == 0 {
            return Err(EmulatorError::NotFound("EventBus".into(), name.into()));
        }
        Ok(())
    }
    
    pub fn put_rule(&self, name: &str, bus_name: &str, pattern: Option<&str>, state: &str, description: Option<&str>, schedule: Option<&str>, account_id: &str, region: &str) -> Result<String> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        let arn = format!("arn:aws:events:{}:{}:rule/{}/{}", region, account_id, bus_name, name);
        
        db.execute(
            r#"INSERT INTO event_rules 
               (name, event_bus_name, arn, event_pattern, state, description, schedule_expression, created_at)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
               ON CONFLICT(event_bus_name, name) DO UPDATE SET
               event_pattern = excluded.event_pattern,
               state = excluded.state,
               description = excluded.description,
               schedule_expression = excluded.schedule_expression"#,
            params![name, bus_name, arn, pattern, state, description, schedule, now],
        )?;
        
        Ok(arn)
    }
    
    pub fn list_rules(&self, bus_name: &str) -> Result<Vec<EventRuleMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            "SELECT name, event_bus_name, arn, event_pattern, state, description, schedule_expression, created_at 
             FROM event_rules WHERE event_bus_name = ?1"
        )?;
        let rules = stmt.query_map(params![bus_name], |row| Ok(EventRuleMetadata {
            name: row.get(0)?,
            event_bus_name: row.get(1)?,
            arn: row.get(2)?,
            event_pattern: row.get(3)?,
            state: row.get(4)?,
            description: row.get(5)?,
            schedule_expression: row.get(6)?,
            created_at: row.get(7)?,
        }))?
        .filter_map(|r| r.ok())
        .collect();
        Ok(rules)
    }
    
    pub fn put_targets(&self, bus_name: &str, rule_name: &str, targets: Vec<EventTargetMetadata>) -> Result<()> {
        let db = self.db.lock();
        for target in targets {
            db.execute(
                r#"INSERT INTO event_targets (id, rule_name, event_bus_name, arn, input, input_path)
                   VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                   ON CONFLICT(event_bus_name, rule_name, id) DO UPDATE SET
                   arn = excluded.arn,
                   input = excluded.input,
                   input_path = excluded.input_path"#,
                params![target.id, rule_name, bus_name, target.arn, target.input, target.input_path],
            )?;
        }
        Ok(())
    }
    
    pub fn list_targets(&self, bus_name: &str, rule_name: &str) -> Result<Vec<EventTargetMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            "SELECT id, rule_name, event_bus_name, arn, input, input_path 
             FROM event_targets WHERE event_bus_name = ?1 AND rule_name = ?2"
        )?;
        let targets = stmt.query_map(params![bus_name, rule_name], |row| Ok(EventTargetMetadata {
            id: row.get(0)?,
            rule_name: row.get(1)?,
            event_bus_name: row.get(2)?,
            arn: row.get(3)?,
            input: row.get(4)?,
            input_path: row.get(5)?,
        }))?
        .filter_map(|r| r.ok())
        .collect();
        Ok(targets)
    }

    pub fn record_event(&self, bus_name: &str, source: &str, detail_type: &str, detail: &str, resources: Option<&str>) -> Result<String> {
        let db = self.db.lock();
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        
        db.execute(
            r#"INSERT INTO event_history (id, event_bus_name, source, detail_type, detail, time, resources)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"#,
            params![id, bus_name, source, detail_type, detail, now, resources],
        )?;
        
        Ok(id)
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

    // ==================== SQS Operations ====================

    pub fn create_queue(&self, name: &str, account_id: &str, region: &str) -> Result<QueueMetadata> {
        let db = self.db.lock();
        let arn = format!("arn:aws:sqs:{}:{}:{}", region, account_id, name);
        let url = format!("http://localhost:4566/{}/{}", account_id, name);
        let now = chrono::Utc::now().to_rfc3339();

        db.execute(
            "INSERT INTO sqs_queues (name, url, arn, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![name, url, arn, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Queue {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(QueueMetadata {
            name: name.to_string(),
            url,
            arn,
            created_at: now,
            visibility_timeout: 30,
            message_retention_period: 345600,
            delay_seconds: 0,
            receive_message_wait_time_seconds: 0,
        })
    }

    pub fn send_message(&self, queue_name: &str, body: &str) -> Result<String> {
        let db = self.db.lock();
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        // Check if queue exists
        let exists: bool = db.query_row(
            "SELECT 1 FROM sqs_queues WHERE name = ?1",
            params![queue_name],
            |_| Ok(true),
        ).unwrap_or(false);

        if !exists {
            return Err(EmulatorError::NotFound("Queue".into(), queue_name.into()));
        }

        db.execute(
            "INSERT INTO sqs_messages (id, queue_name, body, sent_at, visible_at) VALUES (?1, ?2, ?3, ?4, ?4)",
            params![id, queue_name, body, now],
        )?;

        Ok(id)
    }

    pub fn receive_message(&self, queue_name: &str, max_count: i32) -> Result<Vec<MessageMetadata>> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();

        let mut stmt = db.prepare(
            "SELECT id, body, md5_body, sent_at, visible_at, receive_count FROM sqs_messages 
             WHERE queue_name = ?1 AND visible_at <= ?2 
             LIMIT ?3"
        )?;

        let mut messages: Vec<MessageMetadata> = stmt.query_map(params![queue_name, now, max_count], |row| {
            Ok(MessageMetadata {
                id: row.get(0)?,
                queue_name: queue_name.to_string(),
                body: row.get(1)?,
                md5_body: row.get(2)?,
                sent_at: row.get(3)?,
                visible_at: row.get(4)?,
                receipt_handle: None,
                receive_count: row.get(5)?,
            })
        })?.filter_map(|r| r.ok()).collect();

        // Update visibility and receipt handles for received messages
        for msg in &mut messages {
            let handle = uuid::Uuid::new_v4().to_string();
            let new_visible_at = (chrono::Utc::now() + chrono::Duration::seconds(30)).to_rfc3339();
            
            db.execute(
                "UPDATE sqs_messages SET receipt_handle = ?1, visible_at = ?2, receive_count = receive_count + 1 WHERE id = ?3",
                params![handle, new_visible_at, msg.id],
            )?;
            
            msg.receipt_handle = Some(handle);
            msg.visible_at = new_visible_at;
        }

        Ok(messages)
    }

    pub fn delete_message(&self, queue_name: &str, receipt_handle: &str) -> Result<()> {
        let db = self.db.lock();
        let rows = db.execute(
            "DELETE FROM sqs_messages WHERE queue_name = ?1 AND receipt_handle = ?2",
            params![queue_name, receipt_handle],
        )?;

        if rows == 0 {
            return Err(EmulatorError::NotFound("Message".into(), receipt_handle.into()));
        }

        Ok(())
    }

    // ==================== DynamoDB Operations ====================

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

/// Event Bus metadata
#[derive(Debug, Clone)]
pub struct EventBusMetadata {
    pub name: String,
    pub arn: String,
    pub policy: Option<String>,
}

/// Event Rule metadata
#[derive(Debug, Clone)]
pub struct EventRuleMetadata {
    pub name: String,
    pub event_bus_name: String,
    pub arn: String,
    pub event_pattern: Option<String>,
    pub state: String,
    pub description: Option<String>,
    pub schedule_expression: Option<String>,
    pub created_at: String,
}

/// Event Target metadata
#[derive(Debug, Clone)]
pub struct EventTargetMetadata {
    pub id: String,
    pub rule_name: String,
    pub event_bus_name: String,
    pub arn: String,
    pub input: Option<String>,
    pub input_path: Option<String>,
}

/// CloudWatch Metric metadata
#[derive(Debug, Clone)]
pub struct MetricMetadata {
    pub namespace: String,
    pub metric_name: String,
    pub dimensions: Option<String>,
    pub value: f64,
    pub unit: Option<String>,
    pub timestamp: String,
}

/// CloudWatch Log Group metadata
#[derive(Debug, Clone)]
pub struct LogGroupMetadata {
    pub name: String,
    pub arn: String,
    pub retention_days: Option<i32>,
    pub created_at: String,
}

/// CloudWatch Log Stream metadata
#[derive(Debug, Clone)]
pub struct LogStreamMetadata {
    pub name: String,
    pub log_group_name: String,
    pub arn: String,
    pub created_at: String,
}

/// CloudWatch Log Event metadata
#[derive(Debug, Clone)]
pub struct LogEventMetadata {
    pub timestamp: String,
    pub message: String,
}

/// Cognito User Pool metadata
#[derive(Debug, Clone)]
pub struct UserPoolMetadata {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub created_at: String,
}

/// Cognito Group metadata
#[derive(Debug, Clone)]
pub struct UserGroupMetadata {
    pub user_pool_id: String,
    pub group_name: String,
    pub description: Option<String>,
    pub precedence: Option<i32>,
    pub created_at: String,
}

/// Cognito User metadata
#[derive(Debug, Clone)]
pub struct UserMetadata {
    pub user_pool_id: String,
    pub username: String,
    pub email: Option<String>,
    pub status: String,
    pub enabled: bool,
    pub created_at: String,
}

/// Step Functions State Machine metadata
#[derive(Debug, Clone)]
pub struct StateMachineMetadata {
    pub arn: String,
    pub name: String,
    pub definition: String,
    pub role_arn: String,
    pub machine_type: String,
    pub created_at: String,
}

/// Step Functions Execution metadata
#[derive(Debug, Clone)]
pub struct ExecutionMetadata {
    pub arn: String,
    pub state_machine_arn: String,
    pub name: String,
    pub status: String,
    pub input: Option<String>,
    pub output: Option<String>,
    pub start_date: String,
    pub stop_date: Option<String>,
}

/// SQS Queue metadata
#[derive(Debug, Clone)]
pub struct QueueMetadata {
    pub name: String,
    pub url: String,
    pub arn: String,
    pub created_at: String,
    pub visibility_timeout: i32,
    pub message_retention_period: i32,
    pub delay_seconds: i32,
    pub receive_message_wait_time_seconds: i32,
}

/// SQS Message metadata
#[derive(Debug, Clone)]
pub struct MessageMetadata {
    pub id: String,
    pub queue_name: String,
    pub body: String,
    pub md5_body: Option<String>,
    pub sent_at: String,
    pub visible_at: String,
    pub receipt_handle: Option<String>,
    pub receive_count: i32,
}

/// DynamoDB Table metadata
#[derive(Debug, Clone)]
pub struct TableMetadata {
    pub name: String,
    pub arn: String,
    pub status: String,
    pub attribute_definitions: String,
    pub key_schema: String,
    pub created_at: String,
}

/// DynamoDB Item metadata
#[derive(Debug, Clone)]
pub struct ItemMetadata {
    pub table_name: String,
    pub partition_key: String,
    pub sort_key: Option<String>,
    pub item_json: String,
}
