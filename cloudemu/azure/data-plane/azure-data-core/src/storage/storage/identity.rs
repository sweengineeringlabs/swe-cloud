use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePrincipal {
    pub id: String,
    pub app_id: String,
    pub display_name: String,
    pub object_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAssignment {
    pub id: String,
    pub name: String,
    pub scope: String,
    pub principal_id: String,
    pub role_definition_id: String,
}

impl StorageEngine {
    const TABLE_AAD_SPS: &'static str = "azure_aad_service_principals";
    const TABLE_AAD_ROLES: &'static str = "azure_role_assignments";

    pub fn init_identity_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                app_id TEXT NOT NULL,
                display_name TEXT NOT NULL,
                object_type TEXT NOT NULL,
                created_at INTEGER
            )", 
            Self::TABLE_AAD_SPS
        ), [])?;

        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                scope TEXT NOT NULL,
                principal_id TEXT NOT NULL,
                role_definition_id TEXT NOT NULL,
                created_at INTEGER
            )", 
            Self::TABLE_AAD_ROLES
        ), [])?;

        Ok(())
    }

    pub fn create_service_principal(&self, display_name: &str) -> Result<ServicePrincipal> {
        let conn = self.get_connection()?;
        let id = Uuid::new_v4().to_string();
        let app_id = Uuid::new_v4().to_string();

        conn.execute(
            &format!("INSERT INTO {} (
                id, app_id, display_name, object_type, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5)", Self::TABLE_AAD_SPS),
            params![id, app_id, display_name, "ServicePrincipal", Utc::now().timestamp()],
        )?;

        Ok(ServicePrincipal {
            id,
            app_id,
            display_name: display_name.to_string(),
            object_type: "ServicePrincipal".to_string(),
        })
    }

    pub fn create_role_assignment(&self, scope: &str, principal_id: &str, role_id: &str) -> Result<RoleAssignment> {
        let conn = self.get_connection()?;
        let name = Uuid::new_v4().to_string();
        let id = format!("{}/providers/Microsoft.Authorization/roleAssignments/{}", scope, name);

        conn.execute(
            &format!("INSERT INTO {} (
                id, name, scope, principal_id, role_definition_id, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", Self::TABLE_AAD_ROLES),
            params![id, name, scope, principal_id, role_id, Utc::now().timestamp()],
        )?;

        Ok(RoleAssignment {
            id,
            name,
            scope: scope.to_string(),
            principal_id: principal_id.to_string(),
            role_definition_id: role_id.to_string(),
        })
    }
}
