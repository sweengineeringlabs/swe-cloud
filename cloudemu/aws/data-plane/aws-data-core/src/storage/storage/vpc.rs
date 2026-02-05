use super::engine::{StorageEngine, VpcMetadata, SubnetMetadata, SecurityGroupMetadata};
use crate::error::Result;
use rusqlite::params;
use uuid::Uuid;

impl StorageEngine {
    // ==================== VPC Operations ====================

    pub fn create_vpc(&self, cidr_block: &str) -> Result<VpcMetadata> {
        let db = self.db.lock();
        let id = format!("vpc-{}", &Uuid::new_v4().to_string()[..8]);
        
        db.execute(
            "INSERT INTO vpc_vpcs (id, cidr_block) VALUES (?, ?)",
            params![id, cidr_block],
        )?;
        
        Ok(VpcMetadata {
            id,
            cidr_block: cidr_block.to_string(),
            state: "available".into(),
            is_default: false,
            tags: None,
        })
    }

    pub fn list_vpcs(&self) -> Result<Vec<VpcMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT id, cidr_block, state, is_default, tags FROM vpc_vpcs")?;
        let vpcs = stmt.query_map([], |row| {
            Ok(VpcMetadata {
                id: row.get(0)?,
                cidr_block: row.get(1)?,
                state: row.get(2)?,
                is_default: row.get::<_, i32>(3)? != 0,
                tags: row.get(4)?,
            })
        })?.filter_map(|r| r.ok()).collect();
        Ok(vpcs)
    }

    pub fn create_subnet(&self, vpc_id: &str, cidr_block: &str, az: &str) -> Result<SubnetMetadata> {
        let db = self.db.lock();
        let id = format!("subnet-{}", &Uuid::new_v4().to_string()[..8]);
        
        db.execute(
            "INSERT INTO vpc_subnets (id, vpc_id, cidr_block, availability_zone) VALUES (?, ?, ?, ?)",
            params![id, vpc_id, cidr_block, az],
        )?;
        
        Ok(SubnetMetadata {
            id,
            vpc_id: vpc_id.to_string(),
            cidr_block: cidr_block.to_string(),
            availability_zone: az.to_string(),
            state: "available".into(),
            map_public_ip_on_launch: false,
            tags: None,
        })
    }

    pub fn list_subnets(&self) -> Result<Vec<SubnetMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT id, vpc_id, cidr_block, availability_zone, state, map_public_ip_on_launch, tags FROM vpc_subnets")?;
        let subnets = stmt.query_map([], |row| {
            Ok(SubnetMetadata {
                id: row.get(0)?,
                vpc_id: row.get(1)?,
                cidr_block: row.get(2)?,
                availability_zone: row.get(3)?,
                state: row.get(4)?,
                map_public_ip_on_launch: row.get::<_, i32>(5)? != 0,
                tags: row.get(6)?,
            })
        })?.filter_map(|r| r.ok()).collect();
        Ok(subnets)
    }

    pub fn create_security_group(&self, vpc_id: &str, name: &str, desc: &str) -> Result<SecurityGroupMetadata> {
        let db = self.db.lock();
        let id = format!("sg-{}", &Uuid::new_v4().to_string()[..8]);
        
        db.execute(
            "INSERT INTO vpc_security_groups (id, group_name, description, vpc_id) VALUES (?, ?, ?, ?)",
            params![id, name, desc, vpc_id],
        )?;
        
        Ok(SecurityGroupMetadata {
            id,
            name: name.to_string(),
            description: Some(desc.to_string()),
            vpc_id: vpc_id.to_string(),
            tags: None,
        })
    }

    pub fn list_security_groups(&self) -> Result<Vec<SecurityGroupMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT id, group_name, description, vpc_id, tags FROM vpc_security_groups")?;
        let sgs = stmt.query_map([], |row| {
            Ok(SecurityGroupMetadata {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                vpc_id: row.get(3)?,
                tags: row.get(4)?,
            })
        })?.filter_map(|r| r.ok()).collect();
        Ok(sgs)
    }
}
