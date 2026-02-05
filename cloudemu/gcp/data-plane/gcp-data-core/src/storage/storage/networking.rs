use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub name: String,
    pub project_id: String,
    pub auto_create_subnetworks: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subnetwork {
    pub name: String,
    pub network: String,
    pub project_id: String,
    pub region: String,
    pub ip_cidr_range: String,
}

impl StorageEngine {
    pub fn init_networking_tables(&self) -> Result<()> {
        let conn = self.db.lock();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS gcp_networks (
                name TEXT NOT NULL,
                project_id TEXT NOT NULL,
                auto_create_subnetworks BOOLEAN NOT NULL,
                created_at TEXT NOT NULL,
                PRIMARY KEY(project_id, name)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS gcp_subnetworks (
                name TEXT NOT NULL,
                network TEXT NOT NULL,
                project_id TEXT NOT NULL,
                region TEXT NOT NULL,
                ip_cidr_range TEXT NOT NULL,
                PRIMARY KEY(project_id, region, name)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_network(&self, name: &str, project: &str, auto_subnets: bool) -> Result<Network> {
        let conn = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT INTO gcp_networks (name, project_id, auto_create_subnetworks, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![name, project, auto_subnets, now],
        )?;

        Ok(Network {
            name: name.to_string(),
            project_id: project.to_string(),
            auto_create_subnetworks: auto_subnets,
            created_at: now,
        })
    }

    pub fn create_subnetwork(&self, name: &str, net: &str, project: &str, region: &str, cidr: &str) -> Result<Subnetwork> {
        let conn = self.db.lock();
        
        conn.execute(
            "INSERT INTO gcp_subnetworks (name, network, project_id, region, ip_cidr_range)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![name, net, project, region, cidr],
        )?;

        Ok(Subnetwork {
            name: name.to_string(),
            network: net.to_string(),
            project_id: project.to_string(),
            region: region.to_string(),
            ip_cidr_range: cidr.to_string(),
        })
    }
}
