use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiGateway {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub endpoint_type: String, // REGIONAL, PRIVATE, EDGE
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResource {
    pub id: String,
    pub api_id: String,
    pub parent_id: Option<String>,
    pub path_part: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMethod {
    pub api_id: String,
    pub resource_id: String,
    pub http_method: String,
    pub authorization_type: String,
    pub api_key_required: bool,
}

impl StorageEngine {
    pub fn init_apigateway_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS aws_api_gateways (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                endpoint_type TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS aws_api_resources (
                id TEXT PRIMARY KEY,
                api_id TEXT NOT NULL,
                parent_id TEXT,
                path_part TEXT NOT NULL,
                path TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS aws_api_methods (
                api_id TEXT NOT NULL,
                resource_id TEXT NOT NULL,
                http_method TEXT NOT NULL,
                authorization_type TEXT NOT NULL,
                api_key_required BOOLEAN NOT NULL,
                PRIMARY KEY(api_id, resource_id, http_method)
            )",
            [],
        )?;

        Ok(())
    }

    pub fn create_rest_api(&self, name: &str, description: Option<&str>) -> Result<ApiGateway> {
        let conn = self.get_connection()?;
        let id = uuid::Uuid::new_v4().to_string().replace("-", "").to_lowercase()[..10].to_string();
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO aws_api_gateways (id, name, description, endpoint_type, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, name, description, "REGIONAL", now],
        )?;

        // Create root resource
        let root_id = uuid::Uuid::new_v4().to_string().replace("-", "").to_lowercase()[..10].to_string();
        conn.execute(
            "INSERT INTO aws_api_resources (id, api_id, parent_id, path_part, path)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![root_id, id, Option::<String>::None, "/", "/"],
        )?;

        Ok(ApiGateway {
            id,
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            endpoint_type: "REGIONAL".to_string(),
            created_at: now,
        })
    }
    
    pub fn list_rest_apis(&self) -> Result<Vec<ApiGateway>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT id, name, description, endpoint_type, created_at FROM aws_api_gateways")?;
        
        let rows = stmt.query_map([], |row| {
            Ok(ApiGateway {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                endpoint_type: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        
        let mut apis = Vec::new();
        for api in rows {
            apis.push(api?);
        }
        
        Ok(apis)
    }

    pub fn get_rest_api(&self, api_id: &str) -> Result<ApiGateway> {
         let conn = self.get_connection()?;
         let mut stmt = conn.prepare("SELECT id, name, description, endpoint_type, created_at FROM aws_api_gateways WHERE id = ?1")?;
         
         let api = stmt.query_row(params![api_id], |row| {
            Ok(ApiGateway {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                endpoint_type: row.get(3)?,
                created_at: row.get(4)?,
            })
         })?;
         
         Ok(api)
    }

    pub fn create_resource(&self, api_id: &str, parent_id: &str, path_part: &str) -> Result<ApiResource> {
        let conn = self.get_connection()?;
        let id = uuid::Uuid::new_v4().to_string().replace("-", "").to_lowercase()[..10].to_string();
        
        // Simple path calculation (mock)
        let path = format!("/{}/{}", parent_id, path_part);

        conn.execute(
            "INSERT INTO aws_api_resources (id, api_id, parent_id, path_part, path)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, api_id, parent_id, path_part, path],
        )?;

        Ok(ApiResource {
            id,
            api_id: api_id.to_string(),
            parent_id: Some(parent_id.to_string()),
            path_part: path_part.to_string(),
            path,
        })
    }
    
    pub fn list_resources(&self, api_id: &str) -> Result<Vec<ApiResource>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT id, api_id, parent_id, path_part, path FROM aws_api_resources WHERE api_id = ?1")?;
        
        let rows = stmt.query_map(params![api_id], |row| {
            Ok(ApiResource {
                id: row.get(0)?,
                api_id: row.get(1)?,
                parent_id: row.get(2)?,
                path_part: row.get(3)?,
                path: row.get(4)?,
            })
        })?;
        
        let mut resources = Vec::new();
        for r in rows {
            resources.push(r?);
        }
        
        Ok(resources)
    }

    pub fn put_method(&self, api_id: &str, resource_id: &str, http_method: &str, auth_type: &str) -> Result<ApiMethod> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "INSERT OR REPLACE INTO aws_api_methods (api_id, resource_id, http_method, authorization_type, api_key_required)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![api_id, resource_id, http_method, auth_type, false],
        )?;

        Ok(ApiMethod {
            api_id: api_id.to_string(),
            resource_id: resource_id.to_string(),
            http_method: http_method.to_string(),
            authorization_type: auth_type.to_string(),
            api_key_required: false,
        })
    }
}
