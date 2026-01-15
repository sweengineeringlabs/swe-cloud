use super::engine::{StorageEngine, GcsBucketMetadata, GcsObjectMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== Bucket Operations ====================

    pub fn create_gcs_bucket(&self, name: &str, project_id: &str, location: &str) -> Result<()> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();

        db.execute(
            "INSERT INTO gcs_buckets (name, project_id, location, created_at) VALUES (?, ?, ?, ?)",
            params![name, project_id, location, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Bucket {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(())
    }

    pub fn get_gcs_bucket(&self, name: &str) -> Result<GcsBucketMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT name, project_id, location, storage_class, versioning_enabled, created_at FROM gcs_buckets WHERE name = ?",
            params![name],
            |row| {
                Ok(GcsBucketMetadata {
                    name: row.get(0)?,
                    project_id: row.get(1)?,
                    location: row.get(2)?,
                    storage_class: row.get(3)?,
                    versioning_enabled: row.get::<_, i32>(4)? != 0,
                    created_at: row.get(5)?,
                })
            },
        ).map_err(|_| EmulatorError::NotFound(format!("Bucket {} not found", name)))
    }

    pub fn list_gcs_buckets(&self, project_id: &str) -> Result<Vec<GcsBucketMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            "SELECT name, project_id, location, storage_class, versioning_enabled, created_at FROM gcs_buckets WHERE project_id = ? ORDER BY name"
        )?;

        let buckets = stmt.query_map(params![project_id], |row| {
            Ok(GcsBucketMetadata {
                name: row.get(0)?,
                project_id: row.get(1)?,
                location: row.get(2)?,
                storage_class: row.get(3)?,
                versioning_enabled: row.get::<_, i32>(4)? != 0,
                created_at: row.get(5)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

        Ok(buckets)
    }

    // ==================== Object Operations ====================

    pub fn insert_gcs_object(&self, bucket: &str, name: &str, data: &[u8], content_type: Option<&str>) -> Result<GcsObjectMetadata> {
        // Verify bucket exists
        self.get_gcs_bucket(bucket)?;

        // Calculate hash and use common storage engine helper
        let content_hash = self.store_object_data(data)?;
        let etag = format!("\"{}\"", &content_hash[..32]);
        let now = chrono::Utc::now().to_rfc3339();
        
        // Simple generation logic (timestamp based for now)
        let generation = chrono::Utc::now().timestamp_micros();

        let db = self.db.lock();
        
        // Insert object
        db.execute(
            r#"INSERT INTO gcs_objects 
               (name, bucket, generation, content_hash, size, content_type, etag, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            params![
                name,
                bucket,
                generation,
                content_hash,
                data.len() as i64,
                content_type.unwrap_or("application/octet-stream"),
                etag,
                now,
                now,
            ],
        )?;

        Ok(GcsObjectMetadata {
            name: name.to_string(),
            bucket: bucket.to_string(),
            generation,
            size: data.len() as u64,
            content_type: content_type.map(|s| s.to_string()),
            etag,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn get_gcs_object(&self, bucket: &str, name: &str, generation: Option<i64>) -> Result<(GcsObjectMetadata, Vec<u8>)> {
        let db = self.db.lock();
        
        let query_result = if let Some(gen) = generation {
             db.query_row(
                r#"SELECT name, bucket, generation, size, content_type, etag, created_at, updated_at, content_hash
                   FROM gcs_objects WHERE bucket = ? AND name = ? AND generation = ?"#,
                params![bucket, name, gen],
                |row| Ok((
                    GcsObjectMetadata {
                        name: row.get(0)?,
                        bucket: row.get(1)?,
                        generation: row.get(2)?,
                        size: row.get::<_, i64>(3)? as u64,
                        content_type: row.get(4)?,
                        etag: row.get(5)?,
                        created_at: row.get(6)?,
                        updated_at: row.get(7)?,
                    },
                    row.get::<_, String>(8)?,
                ))
            )
        } else {
            // Get latest generation
            db.query_row(
                r#"SELECT name, bucket, generation, size, content_type, etag, created_at, updated_at, content_hash
                   FROM gcs_objects WHERE bucket = ? AND name = ? ORDER BY generation DESC LIMIT 1"#,
                params![bucket, name],
                |row| Ok((
                    GcsObjectMetadata {
                        name: row.get(0)?,
                        bucket: row.get(1)?,
                        generation: row.get(2)?,
                        size: row.get::<_, i64>(3)? as u64,
                        content_type: row.get(4)?,
                        etag: row.get(5)?,
                        created_at: row.get(6)?,
                        updated_at: row.get(7)?,
                    },
                    row.get::<_, String>(8)?,
                ))
            )
        };

        let (metadata, content_hash) = query_result.map_err(|_| EmulatorError::NotFound(format!("Object {}/{} not found", bucket, name)))?;

        drop(db); // Release lock

        let data = self.read_object_data(&content_hash)?;
        Ok((metadata, data))
    }

    pub fn list_gcs_objects(&self, bucket: &str, prefix: Option<&str>) -> Result<Vec<GcsObjectMetadata>> {
        let db = self.db.lock();
        let prefix = prefix.unwrap_or("");
        let pattern = format!("{}%", prefix);

        let mut stmt = db.prepare(
            r#"SELECT name, bucket, generation, size, content_type, etag, created_at, updated_at
               FROM gcs_objects 
               WHERE bucket = ? AND name LIKE ?
               GROUP BY name -- Only show latest version per name for simple list (simplified logic)
               HAVING generation = MAX(generation)"#
        )?;
        
        // Note: The GROUP BY logic above is a simplification; real GCS list behavior is complex with versions.

        let objects = stmt.query_map(params![bucket, pattern], |row| {
             Ok(GcsObjectMetadata {
                name: row.get(0)?,
                bucket: row.get(1)?,
                generation: row.get(2)?,
                size: row.get::<_, i64>(3)? as u64,
                content_type: row.get(4)?,
                etag: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

        Ok(objects)
    }
}
