use super::engine::{StorageEngine, LambdaMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

pub struct CreateFunctionParams<'a> {
    pub name: &'a str,
    pub runtime: &'a str,
    pub role: &'a str,
    pub handler: &'a str,
    pub code_bytes: &'a [u8],
    pub account_id: &'a str,
    pub region: &'a str,
}

impl StorageEngine {
    // ==================== Lambda Operations ====================

    pub fn create_function(&self, params: CreateFunctionParams) -> Result<LambdaMetadata> {
        let arn = format!("arn:aws:lambda:{}:{}:function:{}", params.region, params.account_id, params.name);
        let last_modified = chrono::Utc::now().to_rfc3339();
        
        let code_hash = self.store_object_data(params.code_bytes)?;
        
        let db = self.db.lock();
        db.execute(
            "INSERT INTO lambda_functions (name, arn, runtime, role, handler, code_hash, last_modified) VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![params.name, arn, params.runtime, params.role, params.handler, code_hash, last_modified],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Function {} already exists", params.name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;
        
        Ok(LambdaMetadata {
            name: params.name.to_string(),
            arn,
            runtime: params.runtime.to_string(),
            handler: params.handler.to_string(),
            last_modified,
        })
    }

    pub fn get_function(&self, name: &str) -> Result<LambdaMetadata> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT name, arn, runtime, handler, last_modified FROM lambda_functions WHERE name = ? OR arn = ?")?;
        let row = stmt.query_row(params![name, name], |row| {
            Ok(LambdaMetadata {
                name: row.get(0)?,
                arn: row.get(1)?,
                runtime: row.get(2)?,
                handler: row.get(3)?,
                last_modified: row.get(4)?,
            })
        })?;
        
        Ok(row)
    }

    pub fn get_function_code(&self, name: &str) -> Result<Vec<u8>> {
        let db = self.db.lock();
        let code_hash: String = db.query_row(
            "SELECT code_hash FROM lambda_functions WHERE name = ? OR arn = ?",
            params![name, name],
            |row| row.get(0),
        ).map_err(|_| EmulatorError::NotFound("Function".into(), name.into()))?;
        
        drop(db);
        self.read_object_data(&code_hash)
    }

    pub fn list_functions(&self) -> Result<Vec<LambdaMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare(
            "SELECT name, arn, runtime, handler, last_modified FROM lambda_functions ORDER BY name"
        )?;
        let functions = stmt.query_map([], |row| {
            Ok(LambdaMetadata {
                name: row.get(0)?,
                arn: row.get(1)?,
                runtime: row.get(2)?,
                handler: row.get(3)?,
                last_modified: row.get(4)?,
            })
        })?.filter_map(|r| r.ok()).collect();
        Ok(functions)
    }
}
