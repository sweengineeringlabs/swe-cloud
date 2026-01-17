use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudRunService {
    pub name: String,
    pub project_id: String,
    pub location: String,
    pub image: String,
    pub url: String,
}

impl StorageEngine {
    pub fn init_run_tables(&self) -> Result<()> {
        let conn = self.db.lock();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS gcp_cloud_run (
                name TEXT NOT NULL,
                project_id TEXT NOT NULL,
                location TEXT NOT NULL,
                image TEXT NOT NULL,
                url TEXT NOT NULL,
                PRIMARY KEY(project_id, location, name)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_run_service(&self, name: &str, project: &str, location: &str, image: &str) -> Result<CloudRunService> {
        let conn = self.db.lock();
        let url = format!("https://{}-{}.a.run.app", name, project); // mock url
        
        conn.execute(
            "INSERT INTO gcp_cloud_run (name, project_id, location, image, url)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![name, project, location, image, url],
        )?;

        Ok(CloudRunService {
            name: name.to_string(),
            project_id: project.to_string(),
            location: location.to_string(),
            image: image.to_string(),
            url,
        })
    }
}
