use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub project_id: String,
    pub region: String,
    pub description: String,
    pub state: String,
    pub created_at: String,
}

impl StorageEngine {
    pub fn init_workflows_tables(&self) -> Result<()> {
        let conn = self.db.lock();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS gcp_workflows (
                name TEXT NOT NULL,
                project_id TEXT NOT NULL,
                region TEXT NOT NULL,
                description TEXT NOT NULL,
                state TEXT NOT NULL,
                created_at TEXT NOT NULL,
                PRIMARY KEY(project_id, region, name)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_workflow(&self, name: &str, project: &str, region: &str, desc: &str) -> Result<Workflow> {
        let conn = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO gcp_workflows (name, project_id, region, description, state, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![name, project, region, desc, "ACTIVE", now],
        )?;

        Ok(Workflow {
            name: name.to_string(),
            project_id: project.to_string(),
            region: region.to_string(),
            description: desc.to_string(),
            state: "ACTIVE".to_string(),
            created_at: now,
        })
    }
}
