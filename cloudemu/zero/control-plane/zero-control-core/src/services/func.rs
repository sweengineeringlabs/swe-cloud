use zero_control_spi::{ZeroResult, ZeroError};
use zero_data_core::ZeroEngine;
use std::sync::Arc;
use serde_json::json;

pub struct FuncService {
    engine: Arc<ZeroEngine>,
}

impl FuncService {
    pub fn new(engine: Arc<ZeroEngine>) -> Self {
        Self { engine }
    }

    pub async fn create_function(&self, name: &str, handler: &str, code: &str) -> ZeroResult<()> {
        // Store function metadata in SQLite
        // Schema: name (PK), handler, code_zip_path (placeholder for now we just store raw string or path)
        let conn = self.engine.db.lock();
        
        let sql = "CREATE TABLE IF NOT EXISTS functions (
            name TEXT PRIMARY KEY,
            handler TEXT NOT NULL,
            code TEXT NOT NULL
        )";
        conn.execute(sql, []).map_err(|e| ZeroError::Internal(e.to_string()))?;

        let insert = "INSERT OR REPLACE INTO functions (name, handler, code) VALUES (?1, ?2, ?3)";
        conn.execute(insert, zero_data_core::rusqlite::params![name, handler, code])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
            
        Ok(())
    }

    pub async fn list_functions(&self) -> ZeroResult<Vec<String>> {
        let conn = self.engine.db.lock();
        // Check if table exists first? Or just try query
        let table_exists: bool = conn.query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='functions'",
            [],
            |row| row.get(0),
        ).unwrap_or(false);

        if !table_exists {
            return Ok(vec![]);
        }

        let mut stmt = conn.prepare("SELECT name FROM functions").map_err(|e| ZeroError::Internal(e.to_string()))?;
        let funcs = stmt.query_map([], |row| row.get(0)).map_err(|e| ZeroError::Internal(e.to_string()))?
            .collect::<Result<Vec<String>, _>>().map_err(|e| ZeroError::Internal(e.to_string()))?;
        Ok(funcs)
    }

    pub async fn invoke_function(&self, name: &str, payload: serde_json::Value) -> ZeroResult<serde_json::Value> {
        let conn = self.engine.db.lock();
        
        // 1. Fetch function code
        let mut stmt = conn.prepare("SELECT code, handler FROM functions WHERE name = ?1").map_err(|e| ZeroError::Internal(e.to_string()))?;
        let mut rows = stmt.query(zero_data_core::rusqlite::params![name]).map_err(|e| ZeroError::Internal(e.to_string()))?;
        
        let (code, handler) = if let Some(row) = rows.next().map_err(|e| ZeroError::Internal(e.to_string()))? {
            (
                row.get::<_, String>(0).map_err(|e| ZeroError::Internal(e.to_string()))?,
                row.get::<_, String>(1).map_err(|e| ZeroError::Internal(e.to_string()))?
            )
        } else {
             return Err(ZeroError::NotFound(format!("Function {} not found", name)));
        };

        // 2. Write to temp file
        let func_id = uuid::Uuid::new_v4().to_string();
        let tmp_dir = std::env::temp_dir().join("zero_funcs").join(name);
        std::fs::create_dir_all(&tmp_dir).map_err(|e| ZeroError::Internal(e.to_string()))?;
        
        let file_name = if handler.contains("py") { "main.py" } else { "index.js" };
        let file_path = tmp_dir.join(file_name);
        std::fs::write(&file_path, &code).map_err(|e| ZeroError::Internal(e.to_string()))?;

        // 3. Execute
        let output = if handler.contains("py") {
             std::process::Command::new("python3")
                .arg(&file_path)
                .arg(payload.to_string())
                .output()
        } else {
             // Default to Node
             std::process::Command::new("node")
                .arg(&file_path)
                .arg(payload.to_string())
                .output()
        };

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                Ok(json!({
                    "status": "Executed",
                    "function": name,
                    "stdout": stdout,
                    "stderr": stderr,
                    "exit_code": out.status.code()
                }))
            },
            Err(e) => {
                // If runtime not found, fallback to Mock for stability
                Ok(json!({
                    "status": "MockExecuted",
                    "function": name,
                    "warning": format!("Runtime execution failed: {}. Falling back to mock.", e),
                    "result": "Hello from ZeroFunc (Mock)"
                }))
            }
        }
    }
}
