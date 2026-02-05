use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostedZone {
    pub id: String,
    pub name: String,
    pub caller_reference: String,
    pub config_comment: Option<String>,
    pub private_zone: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRecordSet {
    pub name: String,
    pub r#type: String,
    pub ttl: i64,
    pub resource_records: Vec<ResourceRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRecord {
    pub value: String,
}

impl StorageEngine {
    const TABLE_ROUTE53_ZONES: &'static str = "aws_route53_zones";
    const TABLE_ROUTE53_RECORDS: &'static str = "aws_route53_records";

    pub fn init_route53_tables(&self) -> Result<()> {
        let conn = self.db.lock();
        
        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                caller_reference TEXT NOT NULL,
                config_comment TEXT,
                private_zone BOOLEAN NOT NULL,
                created_at INTEGER
            )", 
            Self::TABLE_ROUTE53_ZONES
        ), [])?;

        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                zone_id TEXT NOT NULL,
                name TEXT NOT NULL,
                type TEXT NOT NULL,
                ttl INTEGER NOT NULL,
                records_json TEXT NOT NULL,
                created_at INTEGER,
                PRIMARY KEY(zone_id, name, type)
            )", 
            Self::TABLE_ROUTE53_RECORDS
        ), [])?;

        Ok(())
    }

    pub fn create_hosted_zone(&self, name: &str, caller_ref: &str) -> Result<HostedZone> {
        let conn = self.db.lock();
        let id = format!("/hostedzone/Z{}", uuid::Uuid::new_v4().to_string().replace("-", "").to_uppercase()[..14].to_string());
        
        conn.execute(
            &format!("INSERT INTO {} (
                id, name, caller_reference, private_zone, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5)", Self::TABLE_ROUTE53_ZONES),
            params![id, name, caller_ref, false, chrono::Utc::now().timestamp()],
        )?;

        // Default SOA and NS records would typically be created here

        Ok(HostedZone {
            id,
            name: name.to_string(),
            caller_reference: caller_ref.to_string(),
            config_comment: None,
            private_zone: false,
        })
    }

    pub fn list_hosted_zones(&self) -> Result<Vec<HostedZone>> {
        let conn = self.db.lock();
        let mut stmt = conn.prepare(&format!("SELECT id, name, caller_reference, config_comment, private_zone FROM {}", Self::TABLE_ROUTE53_ZONES))?;
        
        let zones = stmt.query_map([], |row| {
            Ok(HostedZone {
                id: row.get(0)?,
                name: row.get(1)?,
                caller_reference: row.get(2)?,
                config_comment: row.get(3)?,
                private_zone: row.get(4)?,
            })
        })?
        .collect::<std::result::Result<Vec<HostedZone>, _>>()?;
            
        Ok(zones)
    }

    pub fn change_resource_record_sets(&self, zone_id: &str, change_batch: Vec<ResourceRecordSet>) -> Result<()> {
        let conn = self.db.lock();
        
        for record in change_batch {
            let records_json = serde_json::to_string(&record.resource_records).unwrap();
            conn.execute(
                &format!("INSERT OR REPLACE INTO {} (
                    zone_id, name, type, ttl, records_json, created_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", Self::TABLE_ROUTE53_RECORDS),
                params![zone_id, record.name, record.r#type, record.ttl, records_json, chrono::Utc::now().timestamp()],
            )?;
        }

        Ok(())
    }
    
    pub fn list_resource_record_sets(&self, zone_id: &str) -> Result<Vec<ResourceRecordSet>> {
         let conn = self.db.lock();
         let mut stmt = conn.prepare(&format!("SELECT name, type, ttl, records_json FROM {} WHERE zone_id = ?1", Self::TABLE_ROUTE53_RECORDS))?;
         
         let records = stmt.query_map(params![zone_id], |row| {
            let json_str: String = row.get(3)?;
            let resource_records: Vec<ResourceRecord> = serde_json::from_str(&json_str).unwrap_or_default();
            
            Ok(ResourceRecordSet {
                name: row.get(0)?,
                r#type: row.get(1)?,
                ttl: row.get(2)?,
                resource_records,
            })
         })?
         .collect::<std::result::Result<Vec<ResourceRecordSet>, _>>()?;
         
         Ok(records)
    }
}
