use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IamRole {
    pub name: String,
    pub arn: String,
    pub path: String,
    pub assume_role_policy_document: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IamPolicy {
    pub name: String,
    pub arn: String,
    pub path: String,
    pub default_version_id: String,
    pub document: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IamUser {
    pub name: String,
    pub arn: String,
    pub path: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IamAccessKey {
    pub user_name: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePolicyAttachment {
    pub role_name: String,
    pub policy_arn: String,
}

impl StorageEngine {
    const TABLE_IAM_ROLES: &'static str = "aws_iam_roles";
    const TABLE_IAM_POLICIES: &'static str = "aws_iam_policies";
    const TABLE_IAM_USERS: &'static str = "aws_iam_users";
    const TABLE_IAM_ACCESS_KEYS: &'static str = "aws_iam_access_keys";
    const TABLE_IAM_ROLE_ATTACHMENTS: &'static str = "aws_iam_role_policy_attachments";

    pub fn init_iam_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                arn TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL,
                assume_role_policy_document TEXT NOT NULL,
                description TEXT,
                created_at INTEGER,
                UNIQUE(name)
            )", 
            Self::TABLE_IAM_ROLES
        ), [])?;

        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                arn TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL,
                default_version_id TEXT NOT NULL,
                document TEXT NOT NULL,
                created_at INTEGER,
                UNIQUE(name)
            )", 
            Self::TABLE_IAM_POLICIES
        ), [])?;

        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                arn TEXT NOT NULL,
                path TEXT NOT NULL,
                created_at INTEGER,
                UNIQUE(name)
            )", 
            Self::TABLE_IAM_USERS
        ), [])?;

        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                access_key_id TEXT PRIMARY KEY,
                user_name TEXT NOT NULL,
                secret_access_key TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at INTEGER
            )", 
            Self::TABLE_IAM_ACCESS_KEYS
        ), [])?;

        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                role_name TEXT NOT NULL,
                policy_arn TEXT NOT NULL,
                created_at INTEGER,
                PRIMARY KEY(role_name, policy_arn)
            )", 
            Self::TABLE_IAM_ROLE_ATTACHMENTS
        ), [])?;

        Ok(())
    }

    // Role methods
    pub fn create_role(&self, name: &str, document: &str) -> Result<IamRole> {
        let conn = self.get_connection()?;
        let arn = format!("arn:aws:iam::000000000000:role/{}", name);
        let path = "/";

        conn.execute(
            &format!("INSERT INTO {} (
                arn, name, path, assume_role_policy_document, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5)", Self::TABLE_IAM_ROLES),
            params![arn, name, path, document, chrono::Utc::now().timestamp()],
        )?;

        Ok(IamRole {
            name: name.to_string(),
            arn,
            path: path.to_string(),
            assume_role_policy_document: document.to_string(),
            description: None,
        })
    }

    pub fn get_role(&self, name: &str) -> Result<IamRole> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(&format!("SELECT arn, path, assume_role_policy_document, description FROM {} WHERE name = ?1", Self::TABLE_IAM_ROLES))?;
        
        let role = stmt.query_row(params![name], |row| {
            Ok(IamRole {
                name: name.to_string(),
                arn: row.get(0)?,
                path: row.get(1)?,
                assume_role_policy_document: row.get(2)?,
                description: row.get(3)?,
            })
        })?;

        Ok(role)
    }

    pub fn list_roles(&self) -> Result<Vec<IamRole>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(&format!("SELECT name, arn, path, assume_role_policy_document, description FROM {}", Self::TABLE_IAM_ROLES))?;
        
        let roles = stmt.query_map([], |row| {
             Ok(IamRole {
                name: row.get(0)?,
                arn: row.get(1)?,
                path: row.get(2)?,
                assume_role_policy_document: row.get(3)?,
                description: row.get(4)?,
            })
        })?
        .collect::<std::result::Result<Vec<IamRole>, _>>()?;

        Ok(roles)
    }

    // Policy methods
    pub fn create_policy(&self, name: &str, document: &str) -> Result<IamPolicy> {
        let conn = self.db.lock();
        let arn = format!("arn:aws:iam::000000000000:policy/{}", name);
        let path = "/";
        let version = "v1";

        conn.execute(
            &format!("INSERT INTO {} (
                arn, name, path, default_version_id, document, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", Self::TABLE_IAM_POLICIES),
            params![arn, name, path, version, document, chrono::Utc::now().timestamp()],
        )?;

        Ok(IamPolicy {
            name: name.to_string(),
            arn,
            path: path.to_string(),
            default_version_id: version.to_string(),
            document: document.to_string(),
        })
    }
    
    pub fn list_policies(&self) -> Result<Vec<IamPolicy>> {
        let conn = self.db.lock();
        let mut stmt = conn.prepare(&format!("SELECT name, arn, path, default_version_id, document FROM {}", Self::TABLE_IAM_POLICIES))?;
        
        let policies = stmt.query_map([], |row| {
             Ok(IamPolicy {
                name: row.get(0)?,
                arn: row.get(1)?,
                path: row.get(2)?,
                default_version_id: row.get(3)?,
                document: row.get(4)?,
            })
        })?
        .collect::<std::result::Result<Vec<IamPolicy>, _>>()?;

        Ok(policies)
    }

    pub fn attach_role_policy(&self, role_name: &str, policy_arn: &str) -> Result<()> {
        let conn = self.db.lock();
        conn.execute(
            &format!("INSERT OR IGNORE INTO {} (role_name, policy_arn, created_at) VALUES (?1, ?2, ?3)", Self::TABLE_IAM_ROLE_ATTACHMENTS),
            params![role_name, policy_arn, chrono::Utc::now().timestamp()],
        )?;
        Ok(())
    }

    // User methods
    pub fn create_user(&self, name: &str) -> Result<IamUser> {
         let conn = self.db.lock();
         let id = format!("AIDA{}", uuid::Uuid::new_v4().to_string().replace("-","").to_uppercase()[..16].to_string());
         let arn = format!("arn:aws:iam::000000000000:user/{}", name);
         let path = "/";

         conn.execute(
            &format!("INSERT INTO {} (
                id, name, arn, path, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5)", Self::TABLE_IAM_USERS),
            params![id, name, arn, path, chrono::Utc::now().timestamp()],
        )?;

        Ok(IamUser {
            id,
            name: name.to_string(),
            arn,
            path: path.to_string(),
        })
    }


    pub fn list_users(&self) -> Result<Vec<IamUser>> {
        let conn = self.db.lock();
        let mut stmt = conn.prepare(&format!("SELECT id, name, arn, path FROM {}", Self::TABLE_IAM_USERS))?;
        
        let users = stmt.query_map([], |row| {
             Ok(IamUser {
                id: row.get(0)?,
                name: row.get(1)?,
                arn: row.get(2)?,
                path: row.get(3)?,
            })
        })?
        .collect::<std::result::Result<Vec<IamUser>, _>>()?;

        Ok(users)
    }

    // Access Key methods
    pub fn create_access_key(&self, user_name: &str) -> Result<IamAccessKey> {
        let conn = self.db.lock();
        let access_key = format!("AKIA{}", uuid::Uuid::new_v4().to_string().replace("-","").to_uppercase()[..16].to_string());
        let secret = uuid::Uuid::new_v4().to_string().replace("-",""); // simple secret
        let status = "Active";

         conn.execute(
            &format!("INSERT INTO {} (
                access_key_id, user_name, secret_access_key, status, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5)", Self::TABLE_IAM_ACCESS_KEYS),
            params![access_key, user_name, secret, status, chrono::Utc::now().timestamp()],
        )?;

        Ok(IamAccessKey {
            user_name: user_name.to_string(),
            access_key_id: access_key,
            secret_access_key: secret,
            status: status.to_string(),
        })
    }
}
