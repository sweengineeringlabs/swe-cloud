use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsCluster {
    pub arn: String,
    pub name: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcsTaskDefinition {
    pub arn: String,
    pub family: String,
    pub revision: i32,
    pub container_definitions: Vec<ContainerDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerDefinition {
    pub name: String,
    pub image: String,
    pub cpu: i32,
    pub memory: i32,
    pub port_mappings: Vec<PortMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    pub container_port: i32,
    pub host_port: i32,
    pub protocol: String,
}

impl StorageEngine {
    // ECS Table constants
    const TABLE_ECS_CLUSTERS: &'static str = "aws_ecs_clusters";
    const TABLE_ECS_TASK_DEFS: &'static str = "aws_ecs_task_definitions";

    pub fn init_ecs_tables(&self) -> Result<()> {
        let conn = self.db.lock();
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                arn TEXT PRIMARY KEY,
                name TEXT UNIQUE NOT NULL,
                status TEXT NOT NULL,
                created_at INTEGER
            )", 
            Self::TABLE_ECS_CLUSTERS
        ), [])?;

        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                arn TEXT PRIMARY KEY,
                family TEXT NOT NULL,
                revision INTEGER NOT NULL,
                definition_json TEXT NOT NULL,
                created_at INTEGER,
                UNIQUE(family, revision)
            )", 
            Self::TABLE_ECS_TASK_DEFS
        ), [])?;

        Ok(())
    }

    pub fn create_cluster(&self, name: &str) -> Result<EcsCluster> {
        let conn = self.db.lock();
        let arn = format!("arn:aws:ecs:us-east-1:000000000000:cluster/{}", name);
        let status = "ACTIVE";
        
        conn.execute(
            &format!("INSERT INTO {} (arn, name, status, created_at) VALUES (?1, ?2, ?3, ?4)", Self::TABLE_ECS_CLUSTERS),
            params![arn, name, status, chrono::Utc::now().timestamp()],
        )?;

        Ok(EcsCluster { arn, name: name.to_string(), status: status.to_string() })
    }

    pub fn list_clusters(&self) -> Result<Vec<String>> {
        let conn = self.db.lock();
        let mut stmt = conn.prepare(&format!("SELECT arn FROM {}", Self::TABLE_ECS_CLUSTERS))?;
        
        let arns = stmt.query_map([], |row| row.get(0))?
            .collect::<std::result::Result<Vec<String>, _>>()?;
            
        Ok(arns)
    }

    pub fn register_task_definition(
        &self, 
        family: &str, 
        containers: Vec<ContainerDefinition>
    ) -> Result<EcsTaskDefinition> {
        let conn = self.db.lock();

        // Get next revision
        let last_rev: Option<i32> = conn.query_row(
            &format!("SELECT MAX(revision) FROM {} WHERE family = ?1", Self::TABLE_ECS_TASK_DEFS),
            params![family],
            |row| row.get(0)
        ).ok();

        let revision = last_rev.unwrap_or(0) + 1;
        let arn = format!("arn:aws:ecs:us-east-1:000000000000:task-definition/{}:{}", family, revision);
        
        let def = EcsTaskDefinition {
            arn: arn.clone(),
            family: family.to_string(),
            revision,
            container_definitions: containers,
        };

        let json = serde_json::to_string(&def).unwrap();

        conn.execute(
            &format!("INSERT INTO {} (arn, family, revision, definition_json, created_at) VALUES (?1, ?2, ?3, ?4, ?5)", Self::TABLE_ECS_TASK_DEFS),
            params![arn, family, revision, json, chrono::Utc::now().timestamp()],
        )?;

        Ok(def)
    }
}
