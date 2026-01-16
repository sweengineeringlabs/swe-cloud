use super::engine::{StorageEngine, GcpInstanceMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    // ==================== Instance Operations ====================

    pub fn create_gcp_instance(&self, name: &str, project_id: &str, zone: &str, machine_type: &str, image: &str, network: &str) -> Result<GcpInstanceMetadata> {
        let db = self.db.lock();
        let now = chrono::Utc::now().to_rfc3339();
        
        // Mock IP assignment
        let private_ip = "10.0.0.5".to_string();
        let public_ip = "35.20.30.40".to_string();

        db.execute(
            r#"INSERT INTO gcp_instances 
               (name, project_id, zone, machine_type, image, network, private_ip, public_ip, status, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'RUNNING', ?)"#,
            params![name, project_id, zone, machine_type, image, network, private_ip, public_ip, &now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint") {
                EmulatorError::AlreadyExists(format!("Instance {} already exists", name))
            } else {
                EmulatorError::Database(e.to_string())
            }
        })?;

        Ok(GcpInstanceMetadata {
            name: name.to_string(),
            project_id: project_id.to_string(),
            zone: zone.to_string(),
            machine_type: machine_type.to_string(),
            image: image.to_string(),
            network: network.to_string(),
            private_ip: Some(private_ip),
            public_ip: Some(public_ip),
            status: "RUNNING".to_string(),
            created_at: now,
        })
    }

    pub fn get_gcp_instance(&self, name: &str, project_id: &str, zone: &str) -> Result<GcpInstanceMetadata> {
        let db = self.db.lock();
        db.query_row(
            r#"SELECT name, project_id, zone, machine_type, image, network, private_ip, public_ip, status, created_at 
               FROM gcp_instances WHERE name = ? AND project_id = ? AND zone = ?"#,
            params![name, project_id, zone],
            |row| {
                Ok(GcpInstanceMetadata {
                    name: row.get(0)?,
                    project_id: row.get(1)?,
                    zone: row.get(2)?,
                    machine_type: row.get(3)?,
                    image: row.get(4)?,
                    network: row.get(5)?,
                    private_ip: row.get(6)?,
                    public_ip: row.get(7)?,
                    status: row.get(8)?,
                    created_at: row.get(9)?,
                })
            },
        ).map_err(|_| EmulatorError::NotFound("GcpInstance".into(), format!("Instance {} in project {} zone {}", name, project_id, zone)))
    }

    pub fn delete_gcp_instance(&self, name: &str, project_id: &str, zone: &str) -> Result<()> {
        let db = self.db.lock();
        let count = db.execute(
            "DELETE FROM gcp_instances WHERE name = ? AND project_id = ? AND zone = ?",
            params![name, project_id, zone]
        )?;
        
        if count == 0 {
            return Err(EmulatorError::NotFound("GcpInstance".into(), name.to_string()));
        }
        Ok(())
    }

    pub fn list_gcp_instances(&self, project_id: &str, zone: Option<&str>) -> Result<Vec<GcpInstanceMetadata>> {
        let db = self.db.lock();
        
        let map_row = |row: &rusqlite::Row| {
             Ok(GcpInstanceMetadata {
                name: row.get(0)?,
                project_id: row.get(1)?,
                zone: row.get(2)?,
                machine_type: row.get(3)?,
                image: row.get(4)?,
                network: row.get(5)?,
                private_ip: row.get(6)?,
                public_ip: row.get(7)?,
                status: row.get(8)?,
                created_at: row.get(9)?,
            })
        };

        let mut stmt;
        let instances = if let Some(z) = zone {
            stmt = db.prepare("SELECT name, project_id, zone, machine_type, image, network, private_ip, public_ip, status, created_at FROM gcp_instances WHERE project_id = ? AND zone = ?")?;
            stmt.query_map(params![project_id, z], map_row)?
        } else {
            stmt = db.prepare("SELECT name, project_id, zone, machine_type, image, network, private_ip, public_ip, status, created_at FROM gcp_instances WHERE project_id = ?")?;
            stmt.query_map(params![project_id], map_row)?
        }
        .filter_map(|r| r.ok())
        .collect();

        Ok(instances)
    }
}
