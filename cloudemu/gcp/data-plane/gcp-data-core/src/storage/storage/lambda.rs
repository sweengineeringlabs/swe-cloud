use super::engine::{StorageEngine, LambdaMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

pub struct CreateFunctionParams<'a> {
    pub name: &'a str,
    pub runtime: &'a str,
    pub role: &'a str,
    pub handler: &'a str,
    pub code_hash: &'a str,
    pub account_id: &'a str,
    pub region: &'a str,
}

impl StorageEngine {
    // ==================== Lambda Operations ====================

    pub fn create_function(&self, params: CreateFunctionParams) -> Result<LambdaMetadata> {
        let arn = format!("arn:aws:lambda:{}:{}:function:{}", params.region, params.account_id, params.name);
        let last_modified = chrono::Utc::now().to_rfc3339();
        
        let db = self.db.lock();
        db.execute(
            "INSERT INTO lambda_functions (name, arn, runtime, role, handler, code_hash, last_modified) VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![params.name, arn, params.runtime, params.role, params.handler, params.code_hash, last_modified],
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
}
