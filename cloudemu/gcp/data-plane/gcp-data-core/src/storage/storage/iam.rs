use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAccount {
    pub name: String,
    pub project_id: String,
    pub unique_id: String,
    pub email: String,
    pub display_name: String,
}

impl StorageEngine {
    const TABLE_IAM_SA: &'static str = "gcp_iam_service_accounts";

    pub fn init_iam_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                name TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                unique_id TEXT NOT NULL,
                email TEXT NOT NULL,
                display_name TEXT NOT NULL,
                created_at INTEGER
            )", 
            Self::TABLE_IAM_SA
        ), [])?;

        Ok(())
    }

    pub fn create_service_account(&self, project: &str, account_id: &str, display_name: &str) -> Result<ServiceAccount> {
        let conn = self.get_connection()?;
        let email = format!("{}@{}.iam.gserviceaccount.com", account_id, project);
        let name = format!("projects/{}/serviceAccounts/{}", project, email);
        let unique_id = uuid::Uuid::new_v4().to_string();

        conn.execute(
            &format!("INSERT INTO {} (
                name, project_id, unique_id, email, display_name, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", Self::TABLE_IAM_SA),
            params![name, project, unique_id, email, display_name, chrono::Utc::now().timestamp()],
        )?;

        Ok(ServiceAccount {
            name,
            project_id: project.to_string(),
            unique_id,
            email,
            display_name: display_name.to_string(),
        })
    }
}
