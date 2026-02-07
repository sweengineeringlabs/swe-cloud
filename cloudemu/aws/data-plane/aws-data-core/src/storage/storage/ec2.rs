use super::engine::{StorageEngine, InstanceMetadata, KeyPairMetadata};
use crate::error::Result;
use rusqlite::params;
use uuid::Uuid;

impl StorageEngine {
    // ==================== EC2 Operations ====================

    pub fn run_instances(
        &self, 
        image_id: &str, 
        instance_type: &str, 
        vpc_id: Option<&str>, 
        subnet_id: Option<&str>,
        key_name: Option<&str>
    ) -> Result<InstanceMetadata> {
        let db = self.db.lock();
        let id = format!("i-{}", &Uuid::new_v4().to_string()[..8]);
        let launch_time = chrono::Utc::now().to_rfc3339();
        
        // Mock IP assignment
        let private_ip = format!("10.0.0.{}", rand::random::<u8>());
        let public_ip = format!("54.12.34.{}", rand::random::<u8>());
        
        db.execute(
            "INSERT INTO ec2_instances (id, image_id, instance_type, key_name, private_ip, public_ip, vpc_id, subnet_id, launch_time) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![id, image_id, instance_type, key_name, private_ip, public_ip, vpc_id, subnet_id, launch_time],
        )?;
        
        Ok(InstanceMetadata {
            id,
            image_id: image_id.to_string(),
            instance_type: instance_type.to_string(),
            key_name: key_name.map(|s| s.to_string()),
            state: "running".into(),
            private_ip: Some(private_ip),
            public_ip: Some(public_ip),
            vpc_id: vpc_id.map(|s| s.to_string()),
            subnet_id: subnet_id.map(|s| s.to_string()),
            security_groups: vec![],
            launch_time,
            tags: None,
        })
    }

    pub fn list_instances(&self) -> Result<Vec<InstanceMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT id, image_id, instance_type, key_name, state, private_ip, public_ip, vpc_id, subnet_id, security_groups, launch_time, tags FROM ec2_instances")?;
        let instances = stmt.query_map([], |row| {
            let sg_json: Option<String> = row.get(9)?;
            let security_groups = if let Some(j) = sg_json {
                serde_json::from_str(&j).unwrap_or_default()
            } else {
                vec![]
            };

            Ok(InstanceMetadata {
                id: row.get(0)?,
                image_id: row.get(1)?,
                instance_type: row.get(2)?,
                key_name: row.get(3)?,
                state: row.get(4)?,
                private_ip: row.get(5)?,
                public_ip: row.get(6)?,
                vpc_id: row.get(7)?,
                subnet_id: row.get(8)?,
                security_groups,
                launch_time: row.get(10)?,
                tags: row.get(11)?,
            })
        })?.filter_map(|r| r.ok()).collect();
        Ok(instances)
    }

    pub fn create_key_pair(&self, name: &str) -> Result<KeyPairMetadata> {
        let db = self.db.lock();
        let fingerprint = format!("ae:{:02x}:{:02x}:{:02x}", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>());
        
        db.execute(
            "INSERT INTO ec2_key_pairs (key_name, key_fingerprint) VALUES (?, ?)",
            params![name, fingerprint],
        )?;
        
        Ok(KeyPairMetadata {
            key_name: name.to_string(),
            key_fingerprint: fingerprint,
            key_material: Some("-----BEGIN RSA PRIVATE KEY-----\nMOCK MATERIAL\n-----END RSA PRIVATE KEY-----".into()),
            tags: None,
        })
    }

    pub fn list_key_pairs(&self) -> Result<Vec<KeyPairMetadata>> {
        let db = self.db.lock();
        let mut stmt = db.prepare("SELECT key_name, key_fingerprint, tags FROM ec2_key_pairs")?;
        let keys = stmt.query_map([], |row| {
            Ok(KeyPairMetadata {
                key_name: row.get(0)?,
                key_fingerprint: row.get(1)?,
                key_material: None,
                tags: row.get(2)?,
            })
        })?.filter_map(|r| r.ok()).collect();
        Ok(keys)
    }
}
