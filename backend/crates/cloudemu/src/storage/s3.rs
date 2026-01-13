use super::engine::{StorageEngine, BucketMetadata, ObjectMetadata, ListObjectsResult};
use crate::error::{EmulatorError, Result};
use rusqlite::params;
use std::fs;

impl StorageEngine {
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

    // ==================== Multipart Upload Operations ====================

    pub fn create_multipart_upload(&self, bucket: &str, key: &str) -> Result<String> {
        let upload_id = uuid::Uuid::new_v4().to_string();
        let initiated = chrono::Utc::now().to_rfc3339();
        
        let db = self.db.lock();
        db.execute(
            "INSERT INTO multipart_uploads (upload_id, bucket, key, initiated) VALUES (?, ?, ?, ?)",
            params![upload_id, bucket, key, initiated],
        )?;
        
        Ok(upload_id)
    }

    pub fn upload_part(&self, upload_id: &str, part_number: i32, data: &[u8]) -> Result<String> {
        let content_hash = self.store_object_data(data)?;
        let etag = format!("\"{}\"", &content_hash[..32]);
        let last_modified = chrono::Utc::now().to_rfc3339();
        
        let db = self.db.lock();
        db.execute(
            "INSERT OR REPLACE INTO multipart_parts (upload_id, part_number, content_hash, size, etag, last_modified) VALUES (?, ?, ?, ?, ?, ?)",
            params![upload_id, part_number, content_hash, data.len() as i64, etag, last_modified],
        )?;
        
        Ok(etag)
    }

    pub fn complete_multipart_upload(&self, bucket: &str, key: &str, upload_id: &str) -> Result<String> {
        let db = self.db.lock();
        
        // Get all parts in order
        let mut stmt = db.prepare(
            "SELECT content_hash FROM multipart_parts WHERE upload_id = ? ORDER BY part_number"
        )?;
        let parts: Vec<String> = stmt.query_map(params![upload_id], |row| row.get(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        
        if parts.is_empty() {
            return Err(EmulatorError::InvalidRequest("No parts uploaded".into()));
        }
        
        // Combine all parts
        let mut combined_data = Vec::new();
        for part_hash in &parts {
            let part_data = self.read_object_data(part_hash)?;
            combined_data.extend_from_slice(&part_data);
        }
        
        // Store the combined object
        let final_hash = self.store_object_data(&combined_data)?;
        let etag = format!("\"{}\"", &final_hash[..32]);
        
        // Create object metadata
        let last_modified = chrono::Utc::now().to_rfc3339();
        db.execute(
            "INSERT INTO objects (bucket, key, version_id, is_latest, content_hash, content_length, content_type, etag, last_modified, metadata) VALUES (?, ?, NULL, 1, ?, ?, 'application/octet-stream', ?, ?, NULL)",
            params![bucket, key, final_hash, combined_data.len() as i64, etag, last_modified],
        )?;
        
        // Clean up multipart data
        db.execute("DELETE FROM multipart_uploads WHERE upload_id = ?", params![upload_id])?;
        
        Ok(etag)
    }

    pub fn abort_multipart_upload(&self, upload_id: &str) -> Result<()> {
        let db = self.db.lock();
        db.execute("DELETE FROM multipart_uploads WHERE upload_id = ?", params![upload_id])?;
        Ok(())
    }
}
