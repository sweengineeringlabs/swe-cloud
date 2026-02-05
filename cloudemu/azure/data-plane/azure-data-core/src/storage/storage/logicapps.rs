use chrono::Utc;
use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicApp {
    pub name: String,
    pub resource_group: String,
    pub location: String,
    pub definition: String,
    pub state: String,
    pub created_at: String,
}

impl StorageEngine {
    pub fn init_logicapps_tables(&self) -> Result<()> {
        let conn = self.db.lock();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS azure_logic_apps (
                name TEXT NOT NULL,
                resource_group TEXT NOT NULL,
                location TEXT NOT NULL,
                definition TEXT NOT NULL,
                state TEXT NOT NULL,
                created_at TEXT NOT NULL,
                PRIMARY KEY(resource_group, name)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_logic_app(&self, name: &str, rg: &str, location: &str, definition: &str) -> Result<LogicApp> {
        let conn = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO azure_logic_apps (name, resource_group, location, definition, state, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![name, rg, location, definition, "Enabled", now],
        )?;

        Ok(LogicApp {
            name: name.to_string(),
            resource_group: rg.to_string(),
            location: location.to_string(),
            definition: definition.to_string(),
            state: "Enabled".to_string(),
            created_at: now,
        })
    }
}
