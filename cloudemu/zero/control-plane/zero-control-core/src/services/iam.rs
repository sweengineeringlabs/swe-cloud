use zero_control_spi::{ZeroResult, ZeroError};
use zero_data_core::ZeroEngine;
use std::sync::Arc;
use serde_json::json;

pub struct IamService {
    engine: Arc<ZeroEngine>,
}

impl IamService {
    pub fn new(engine: Arc<ZeroEngine>) -> Self {
        Self { engine }
    }

    pub async fn create_user(&self, username: &str) -> ZeroResult<()> {
        self.create_entity("users", "username", username).await
    }

    pub async fn attach_user_policy(&self, username: &str, policy_json: &str) -> ZeroResult<()> {
        let conn = self.engine.db.lock();
        // Validate JSON
        let _parsed: serde_json::Value = serde_json::from_str(policy_json)
             .map_err(|e| ZeroError::Validation(format!("Invalid JSON policy: {}", e)))?;

        let update = "UPDATE users SET policy = ?1 WHERE username = ?2";
        let affected = conn.execute(update, zero_data_core::rusqlite::params![policy_json, username])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
        
        if affected == 0 {
            return Err(ZeroError::NotFound(format!("User {} not found", username)));
        }
        Ok(())
    }

    pub async fn list_users(&self) -> ZeroResult<Vec<serde_json::Value>> {
        self.list_entities("users", "username", "UserName")
    }

    pub async fn create_role(&self, rolename: &str) -> ZeroResult<()> {
        self.create_entity("roles", "rolename", rolename).await
    }

    pub async fn list_roles(&self) -> ZeroResult<Vec<serde_json::Value>> {
        self.list_entities("roles", "rolename", "RoleName")
    }

    pub async fn create_group(&self, groupname: &str) -> ZeroResult<()> {
         self.create_entity("groups", "groupname", groupname).await
    }

    pub async fn list_groups(&self) -> ZeroResult<Vec<serde_json::Value>> {
        self.list_entities("groups", "groupname", "GroupName")
    }

    // Generic helper to reduce code duplication
    async fn create_entity(&self, table: &str, pk_col: &str, name: &str) -> ZeroResult<()> {
        let conn = self.engine.db.lock();
        let sql = format!("CREATE TABLE IF NOT EXISTS {} (
            {} TEXT PRIMARY KEY,
            arn TEXT NOT NULL,
            policy TEXT
        )", table, pk_col);
        conn.execute(&sql, []).map_err(|e| ZeroError::Internal(e.to_string()))?;

        let arn_type = if table == "users" { "user" } else if table == "roles" { "role" } else { "group" };
        let arn = format!("arn:zero:iam::000000:{}/{}", arn_type, name);
        
        let insert = format!("INSERT OR REPLACE INTO {} ({}, arn, policy) VALUES (?1, ?2, ?3)", table, pk_col);
        conn.execute(&insert, zero_data_core::rusqlite::params![name, arn, "{}"])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
        Ok(())
    }

    fn list_entities(&self, table: &str, pk_col: &str, key_name: &str) -> ZeroResult<Vec<serde_json::Value>> {
        let conn = self.engine.db.lock();
        let table_exists: bool = conn.query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name=?1",
            zero_data_core::rusqlite::params![table],
            |row| row.get(0),
        ).unwrap_or(false);

        if !table_exists { return Ok(vec![]); }

        let sql = format!("SELECT {}, arn, policy FROM {}", pk_col, table);
        let mut stmt = conn.prepare(&sql).map_err(|e| ZeroError::Internal(e.to_string()))?;
        let items = stmt.query_map([], |row| {
             let name: String = row.get(0)?;
             let arn: String = row.get(1)?;
             let policy: String = row.get(2).unwrap_or("{}".to_string());
             let mut json_obj = json!({ "Arn": arn, "Policy": policy });
             json_obj.as_object_mut().unwrap().insert(key_name.to_string(), json!(name));
             Ok(json_obj)
        }).map_err(|e| ZeroError::Internal(e.to_string()))?
            .collect::<Result<Vec<serde_json::Value>, _>>().map_err(|e| ZeroError::Internal(e.to_string()))?;
        Ok(items)
    }

    // A simple mock of AWS Policy Eval Logic
    // JSON: { "Statement": [ { "Effect": "Allow", "Action": "*", "Resource": "*" } ] }
    pub fn verify_permission(&self, username: &str, action: &str, resource: &str) -> bool {
        let conn = self.engine.db.lock();
        
        let mut stmt = match conn.prepare("SELECT policy FROM users WHERE username = ?1") {
            Ok(s) => s,
            Err(_) => return false,
        };
        
        // Retrieve policy
        let policy_str: String = match stmt.query_row(zero_data_core::rusqlite::params![username], |row| row.get(0)) {
            Ok(p) => p,
            Err(_) => return false, // User not found or error
        };

        // Parse JSON
        let policy: serde_json::Value = match serde_json::from_str(&policy_str) {
            Ok(p) => p,
            Err(_) => return false,
        };

        // Iterate Statements
        if let Some(statements) = policy["Statement"].as_array() {
            for stmt in statements {
                let effect = stmt["Effect"].as_str().unwrap_or("Deny");
                let stmt_action = stmt["Action"].as_str().unwrap_or("");
                let stmt_resource = stmt["Resource"].as_str().unwrap_or("");

                // Super Simple Matcher (supports "*" wildcard only)
                let action_match = stmt_action == "*" || stmt_action == action;
                let resource_match = stmt_resource == "*" || stmt_resource == resource;

                if effect == "Allow" && action_match && resource_match {
                    return true;
                }
            }
        }

        false
    }
}
