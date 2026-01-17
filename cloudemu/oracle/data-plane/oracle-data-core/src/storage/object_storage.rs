use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bucket {
    pub name: String,
    pub namespace: String,
    pub compartment_id: String,
    pub created_by: String,
    pub time_created: i64,
    pub etag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    pub name: String,
    pub bucket_name: String,
    pub namespace: String,
    pub size: u64,
    pub md5: String,
    pub time_created: i64,
    pub etag: String,
    pub content_type: Option<String>,
}

impl StorageEngine {
    const TABLE_OCI_BUCKETS: &'static str = "oci_object_storage_buckets";
    const TABLE_OCI_OBJECTS: &'static str = "oci_object_storage_objects";

    pub fn init_object_storage_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                name TEXT NOT NULL,
                namespace TEXT NOT NULL,
                compartment_id TEXT NOT NULL,
                created_by TEXT NOT NULL,
                time_created INTEGER,
                etag TEXT NOT NULL,
                PRIMARY KEY(namespace, name)
            )", 
            Self::TABLE_OCI_BUCKETS
        ), [])?;

        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                name TEXT NOT NULL,
                bucket_name TEXT NOT NULL,
                namespace TEXT NOT NULL,
                size INTEGER NOT NULL,
                md5 TEXT NOT NULL,
                time_created INTEGER,
                etag TEXT NOT NULL,
                content_type TEXT,
                PRIMARY KEY(namespace, bucket_name, name)
            )", 
            Self::TABLE_OCI_OBJECTS
        ), [])?;

        Ok(())
    }

    pub fn create_bucket(&self, namespace: &str, name: &str, compartment_id: &str, user: &str) -> Result<Bucket> {
        let conn = self.get_connection()?;
        let etag = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        conn.execute(
            &format!("INSERT INTO {} (
                name, namespace, compartment_id, created_by, time_created, etag
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", Self::TABLE_OCI_BUCKETS),
            params![name, namespace, compartment_id, user, now, etag],
        )?;

        // Create directory for bucket objects
        let bucket_path = self.data_dir.join("objects").join(namespace).join(name);
        std::fs::create_dir_all(&bucket_path)?;

        Ok(Bucket {
            name: name.to_string(),
            namespace: namespace.to_string(),
            compartment_id: compartment_id.to_string(),
            created_by: user.to_string(),
            time_created: now,
            etag,
        })
    }

    pub fn get_bucket(&self, namespace: &str, name: &str) -> Result<Bucket> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(&format!("SELECT compartment_id, created_by, time_created, etag FROM {} WHERE namespace = ?1 AND name = ?2", Self::TABLE_OCI_BUCKETS))?;
        
        let bucket = stmt.query_row(params![namespace, name], |row| {
            Ok(Bucket {
                name: name.to_string(),
                namespace: namespace.to_string(),
                compartment_id: row.get(0)?,
                created_by: row.get(1)?,
                time_created: row.get(2)?,
                etag: row.get(3)?,
            })
        })?;

        Ok(bucket)
    }

    pub fn put_object(&self, namespace: &str, bucket: &str, name: &str, data: &[u8], content_type: Option<&str>) -> Result<Object> {
        let conn = self.get_connection()?;
        let etag = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        let size = data.len() as u64;
        let md5 = format!("{:x}", md5::compute(data)); // simple md5

        conn.execute(
            &format!("INSERT INTO {} (
                name, bucket_name, namespace, size, md5, time_created, etag, content_type
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
              ON CONFLICT(namespace, bucket_name, name) DO UPDATE SET
                size=excluded.size, md5=excluded.md5, time_created=excluded.time_created, etag=excluded.etag, content_type=excluded.content_type
            ", Self::TABLE_OCI_OBJECTS),
            params![name, bucket, namespace, size, md5, now, etag, content_type],
        )?;

        // Write content to file
        let file_path = self.data_dir.join("objects").join(namespace).join(bucket).join(name);
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(file_path, data)?;

        Ok(Object {
            name: name.to_string(),
            bucket_name: bucket.to_string(),
            namespace: namespace.to_string(),
            size,
            md5,
            time_created: now,
            etag,
            content_type: content_type.map(|s| s.to_string()),
        })
    }
}
