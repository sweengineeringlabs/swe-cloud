use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancer {
    pub arn: String,
    pub name: String,
    pub dns_name: String,
    pub scheme: String, // internet-facing | internal
    pub vpc_id: String,
    pub state: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetGroup {
    pub arn: String,
    pub name: String,
    pub protocol: String,
    pub port: i32,
    pub vpc_id: String,
    pub target_type: String, // instance | ip | lambda
}

impl StorageEngine {
    pub fn init_elb_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS aws_load_balancers (
                arn TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                dns_name TEXT NOT NULL,
                scheme TEXT NOT NULL,
                vpc_id TEXT NOT NULL,
                state TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS aws_target_groups (
                arn TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                protocol TEXT NOT NULL,
                port INTEGER NOT NULL,
                vpc_id TEXT NOT NULL,
                target_type TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    pub fn create_load_balancer(&self, name: &str, subnets: Vec<String>, scheme: &str) -> Result<LoadBalancer> {
        let conn = self.get_connection()?;
        let arn = format!("arn:aws:elasticloadbalancing:us-east-1:000000000000:loadbalancer/app/{}/{}", name, uuid::Uuid::new_v4().to_string().replace("-", "")[..16].to_string());
        let dns_name = format!("{}.elb.localhost.localstack.cloud", name);
        let now = chrono::Utc::now().to_rfc3339();
        let state = "active";
        // Mock VPC ID from subnet (assuming first subnet dictates VPC)
        let vpc_id = "vpc-mock-id"; 
        
        let _ = subnets; // unused for now in this mock

        conn.execute(
            "INSERT INTO aws_load_balancers (arn, name, dns_name, scheme, vpc_id, state, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![arn, name, dns_name, scheme, vpc_id, state, now],
        )?;

        Ok(LoadBalancer {
            arn,
            name: name.to_string(),
            dns_name,
            scheme: scheme.to_string(),
            vpc_id: vpc_id.to_string(),
            state: state.to_string(),
            created_at: now,
        })
    }
    
    pub fn list_load_balancers(&self) -> Result<Vec<LoadBalancer>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT arn, name, dns_name, scheme, vpc_id, state, created_at FROM aws_load_balancers")?;
        
        let rows = stmt.query_map([], |row| {
            Ok(LoadBalancer {
                arn: row.get(0)?,
                name: row.get(1)?,
                dns_name: row.get(2)?,
                scheme: row.get(3)?,
                vpc_id: row.get(4)?,
                state: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;
        
        let mut elbs = Vec::new();
        for elb in rows {
            elbs.push(elb?);
        }
        
        Ok(elbs)
    }

    pub fn create_target_group(&self, name: &str, protocol: &str, port: i32, vpc_id: &str) -> Result<TargetGroup> {
        let conn = self.get_connection()?;
        let arn = format!("arn:aws:elasticloadbalancing:us-east-1:000000000000:targetgroup/{}/{}", name, uuid::Uuid::new_v4().to_string().replace("-", "")[..16].to_string());
        
        conn.execute(
            "INSERT INTO aws_target_groups (arn, name, protocol, port, vpc_id, target_type)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![arn, name, protocol, port, vpc_id, "instance"],
        )?;

        Ok(TargetGroup {
            arn,
            name: name.to_string(),
            protocol: protocol.to_string(),
            port,
            vpc_id: vpc_id.to_string(),
            target_type: "instance".to_string(),
        })
    }
    
    pub fn list_target_groups(&self) -> Result<Vec<TargetGroup>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT arn, name, protocol, port, vpc_id, target_type FROM aws_target_groups")?;
        
        let rows = stmt.query_map([], |row| {
            Ok(TargetGroup {
                arn: row.get(0)?,
                name: row.get(1)?,
                protocol: row.get(2)?,
                port: row.get(3)?,
                vpc_id: row.get(4)?,
                target_type: row.get(5)?,
            })
        })?;
        
        let mut tgs = Vec::new();
        for tg in rows {
            tgs.push(tg?);
        }
        
        Ok(tgs)
    }
}
