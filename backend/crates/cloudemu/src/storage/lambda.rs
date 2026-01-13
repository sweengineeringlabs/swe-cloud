use super::engine::{StorageEngine, LambdaMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== Lambda Operations ====================

    // NOTE: This function will be refactored with a builder pattern in upcoming storage refactor
    #[allow(clippy::too_many_arguments)]
    pub fn create_function(
        &self,
        name: &str,
        runtime: &str,
        role: &str,
        handler: &str,
        code_hash: &str,
        account_id: &str,
        region: &str
    ) -> Result<LambdaMetadata> {
        let arn = format!("arn:aws:lambda:{}:{}:function:{}", region, account_id, name);
        let last_modified = chrono::Utc::now().to_rfc3339();
        
        let db = self.db.lock();
        db.execute(
            "INSERT INTO lambda_functions (name, arn, runtime, role, handler, code_hash, last_modified) VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![name, arn, runtime, role, handler, code_hash, last_modified],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Function {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;
        
        Ok(LambdaMetadata {
            name: name.to_string(),
            arn,
            runtime: runtime.to_string(),
            handler: handler.to_string(),
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
