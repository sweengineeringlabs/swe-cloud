use super::engine::{StorageEngine, FirestoreDatabaseMetadata, FirestoreDocumentMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== Database Operations ====================

    pub fn create_firestore_database(&self, name: &str, project_id: &str, location_id: &str) -> Result<()> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();

        db.execute(
            "INSERT INTO fs_databases (name, project_id, location_id, created_at) VALUES (?, ?, ?, ?)",
            params![name, project_id, location_id, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Database {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(())
    }

    pub fn get_firestore_database(&self, name: &str) -> Result<FirestoreDatabaseMetadata> {
        let db = self.db.lock();
        db.query_row(
            "SELECT name, project_id, location_id, created_at FROM fs_databases WHERE name = ?",
            params![name],
            |row| {
                Ok(FirestoreDatabaseMetadata {
                    name: row.get(0)?,
                    project_id: row.get(1)?,
                    location_id: row.get(2)?,
                    created_at: row.get(3)?,
                })
            },
        ).map_err(|_| EmulatorError::NotFound("FirestoreDatabase".into(), name.into()))
    }

    // ==================== Document Operations ====================

    pub fn create_document(&self, database_name: &str, collection_id: &str, document_id: &str, fields_json: &serde_json::Value) -> Result<FirestoreDocumentMetadata> {
        // Auto-create database if not exists (Lazy provisioning)
        if self.get_firestore_database(database_name).is_err() {
             self.create_firestore_database(database_name, "auto-project", "global")?;
        }

        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        
        // Construct full path: projects/{p}/databases/{d}/documents/{c}/{doc}
        // Simplifying to just relative path for now or strictly following structure? 
        // Let's store consistent path.
        let path = format!("{}/documents/{}/{}", database_name, collection_id, document_id);
        let json_str = fields_json.to_string();

        db.execute(
            r#"INSERT OR REPLACE INTO fs_documents 
               (path, database_name, collection_id, document_id, fields_json, create_time, update_time)
               VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            params![
                path,
                database_name,
                collection_id,
                document_id,
                json_str,
                now,
                now,
            ],
        )?;

        Ok(FirestoreDocumentMetadata {
            path,
            database_name: database_name.to_string(),
            collection_id: collection_id.to_string(),
            document_id: document_id.to_string(),
            fields_json: json_str,
            create_time: now.clone(),
            update_time: now,
        })
    }

    pub fn get_document(&self, database_name: &str, collection_id: &str, document_id: &str) -> Result<FirestoreDocumentMetadata> {
        let db = self.db.lock();
        let path = format!("{}/documents/{}/{}", database_name, collection_id, document_id);

        db.query_row(
            r#"SELECT path, database_name, collection_id, document_id, fields_json, create_time, update_time 
               FROM fs_documents WHERE database_name = ? AND path = ?"#,
            params![database_name, path],
            |row| {
                Ok(FirestoreDocumentMetadata {
                    path: row.get(0)?,
                    database_name: row.get(1)?,
                    collection_id: row.get(2)?,
                    document_id: row.get(3)?,
                    fields_json: row.get(4)?,
                    create_time: row.get(5)?,
                    update_time: row.get(6)?,
                })
            },
        ).map_err(|_| EmulatorError::NotFound("FirestoreDocument".into(), path))
    }

    pub fn list_documents(&self, database_name: &str, collection_id: &str) -> Result<Vec<FirestoreDocumentMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            r#"SELECT path, database_name, collection_id, document_id, fields_json, create_time, update_time 
               FROM fs_documents WHERE database_name = ? AND collection_id = ?"#
        )?;

        let docs = stmt.query_map(params![database_name, collection_id], |row| {
             Ok(FirestoreDocumentMetadata {
                path: row.get(0)?,
                database_name: row.get(1)?,
                collection_id: row.get(2)?,
                document_id: row.get(3)?,
                fields_json: row.get(4)?,
                create_time: row.get(5)?,
                update_time: row.get(6)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

        Ok(docs)
    }
}
