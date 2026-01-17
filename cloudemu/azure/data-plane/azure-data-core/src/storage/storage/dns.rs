use chrono::Utc;
use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsZone {
    pub id: String,
    pub name: String,
    pub resource_group: String,
    pub zone_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordSet {
    pub id: String,
    pub name: String,
    pub type_name: String,
    pub ttl: i64,
    pub records: Vec<String>, // simplified
}

impl StorageEngine {
    const TABLE_DNS_ZONES: &'static str = "azure_dns_zones";
    const TABLE_DNS_RECORDS: &'static str = "azure_dns_records";

    pub fn init_dns_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                resource_group TEXT NOT NULL,
                zone_type TEXT NOT NULL,
                created_at INTEGER
            )", 
            Self::TABLE_DNS_ZONES
        ), [])?;

        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                zone_id TEXT NOT NULL,
                name TEXT NOT NULL,
                type_name TEXT NOT NULL,
                ttl INTEGER NOT NULL,
                records_json TEXT NOT NULL,
                created_at INTEGER
            )", 
            Self::TABLE_DNS_RECORDS
        ), [])?;

        Ok(())
    }

    pub fn create_dns_zone(&self, name: &str, rg: &str) -> Result<DnsZone> {
        let conn = self.get_connection()?;
        let id = format!("/subscriptions/sub-1/resourceGroups/{}/providers/Microsoft.Network/dnsZones/{}", rg, name);

        conn.execute(
            &format!("INSERT INTO {} (
                id, name, resource_group, zone_type, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5)", Self::TABLE_DNS_ZONES),
            params![id, name, rg, "Public", chrono::Utc::now().timestamp()],
        )?;

        Ok(DnsZone {
            id,
            name: name.to_string(),
            resource_group: rg.to_string(),
            zone_type: "Public".to_string(),
        })
    }
}
