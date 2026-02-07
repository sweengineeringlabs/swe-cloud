use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vcn {
    pub id: String,
    pub display_name: String,
    pub compartment_id: String,
    pub cidr_block: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subnet {
    pub id: String,
    pub display_name: String,
    pub vcn_id: String,
    pub compartment_id: String,
    pub cidr_block: String,
}

impl StorageEngine {
    pub fn init_networking_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS oci_vcns (
                id TEXT NOT NULL,
                display_name TEXT NOT NULL,
                compartment_id TEXT NOT NULL,
                cidr_block TEXT NOT NULL,
                state TEXT NOT NULL,
                PRIMARY KEY(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS oci_subnets (
                id TEXT NOT NULL,
                display_name TEXT NOT NULL,
                vcn_id TEXT NOT NULL,
                compartment_id TEXT NOT NULL,
                cidr_block TEXT NOT NULL,
                PRIMARY KEY(id)
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_vcn(&self, name: &str, compartment: &str, cidr: &str) -> Result<Vcn> {
        let conn = self.get_connection()?;
        let id = format!("ocid1.vcn.oc1..{}", uuid::Uuid::new_v4());
        
        conn.execute(
            "INSERT INTO oci_vcns (id, display_name, compartment_id, cidr_block, state)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, name, compartment, cidr, "AVAILABLE"],
        )?;

        Ok(Vcn {
            id,
            display_name: name.to_string(),
            compartment_id: compartment.to_string(),
            cidr_block: cidr.to_string(),
            state: "AVAILABLE".to_string(),
        })
    }

    pub fn create_subnet(&self, name: &str, vcn: &str, compartment: &str, cidr: &str) -> Result<Subnet> {
        let conn = self.get_connection()?;
        let id = format!("ocid1.subnet.oc1..{}", uuid::Uuid::new_v4());
        
        conn.execute(
            "INSERT INTO oci_subnets (id, display_name, vcn_id, compartment_id, cidr_block)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, name, vcn, compartment, cidr],
        )?;

        Ok(Subnet {
            id,
            display_name: name.to_string(),
            vcn_id: vcn.to_string(),
            compartment_id: compartment.to_string(),
            cidr_block: cidr.to_string(),
        })
    }
}
