use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpSqlInstance {
    pub name: String,
    pub project: String,
    pub region: String,
    pub tier: String,
    pub state: String,
}

impl StorageEngine {
    const TABLE_GCP_SQL: &'static str = "gcp_sql_instances";

    pub fn init_sql_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                self_link TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                project TEXT NOT NULL,
                region TEXT NOT NULL,
                tier TEXT NOT NULL,
                state TEXT NOT NULL,
                created_at INTEGER,
                UNIQUE(name, project)
            )", 
            Self::TABLE_GCP_SQL
        ), [])?;

        Ok(())
    }

    pub fn insert_sql_instance(&self, db: GcpSqlInstance) -> Result<GcpSqlInstance> {
        let conn = self.get_connection()?;
        let self_link = format!("https://sqladmin.googleapis.com/sql/v1beta4/projects/{}/instances/{}", db.project, db.name);

        conn.execute(
            &format!("INSERT INTO {} (
                self_link, name, project, region, tier, state, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", Self::TABLE_GCP_SQL),
            params![
                self_link, db.name, db.project, db.region, db.tier, "RUNNABLE", 
                chrono::Utc::now().timestamp()
            ],
        )?;

        Ok(db)
    }
}
