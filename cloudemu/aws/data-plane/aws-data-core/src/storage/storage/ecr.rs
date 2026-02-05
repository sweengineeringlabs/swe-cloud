use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcrRepository {
    pub repository_name: String,
    pub repository_arn: String,
    pub registry_id: String,
    pub repository_uri: String,
    pub created_at: String,
}

impl StorageEngine {
    pub fn init_ecr_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS aws_ecr_repositories (
                repository_name TEXT PRIMARY KEY,
                repository_arn TEXT NOT NULL,
                registry_id TEXT NOT NULL,
                repository_uri TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    pub fn create_repository(&self, name: &str) -> Result<EcrRepository> {
        let conn = self.get_connection()?;
        let account_id = "000000000000";
        let region = "us-east-1";
        let arn = format!("arn:aws:ecr:{}:{}:repository/{}", region, account_id, name);
        let uri = format!("{}.dkr.ecr.{}.amazonaws.com/{}", account_id, region, name);
        let now = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO aws_ecr_repositories (repository_name, repository_arn, registry_id, repository_uri, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![name, arn, account_id, uri, now],
        )?;

        Ok(EcrRepository {
            repository_name: name.to_string(),
            repository_arn: arn,
            registry_id: account_id.to_string(),
            repository_uri: uri,
            created_at: now,
        })
    }
    
    pub fn list_repositories(&self) -> Result<Vec<EcrRepository>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT repository_name, repository_arn, registry_id, repository_uri, created_at FROM aws_ecr_repositories")?;
        
        let rows = stmt.query_map([], |row| {
             Ok(EcrRepository {
                repository_name: row.get(0)?,
                repository_arn: row.get(1)?,
                registry_id: row.get(2)?,
                repository_uri: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        
        let mut repos = Vec::new();
        for r in rows {
            repos.push(r?);
        }
        
        Ok(repos)
    }
}
