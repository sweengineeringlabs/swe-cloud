use super::engine::{StorageEngine, GcpInstanceMetadata};
use crate::error::{EmulatorError, Result};
use rusqlite::params;

impl StorageEngine {
    const TABLE_GCP_INSTANCES: &'static str = "gcp_instances";

    pub fn init_compute_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;

        conn.execute(&format!(
            "CREATE TABLE IF NOT EXISTS {} (
                self_link TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                project_id TEXT NOT NULL,
                zone TEXT NOT NULL,
                machine_type TEXT NOT NULL,
                image TEXT NOT NULL,
                network TEXT NOT NULL,
                private_ip TEXT,
                public_ip TEXT,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL,
                UNIQUE(name, project_id, zone)
            )",
            Self::TABLE_GCP_INSTANCES
        ), [])?;

        Ok(())
    }

    pub fn create_gcp_instance(
        &self,
        name: &str,
        project_id: &str,
        zone: &str,
        machine_type: &str,
        image: &str,
        network: &str,
    ) -> Result<GcpInstanceMetadata> {
        let conn = self.get_connection()?;
        let self_link = format!(
            "https://www.googleapis.com/compute/v1/projects/{}/zones/{}/instances/{}",
            project_id, zone, name
        );
        let now = chrono::Utc::now().to_rfc3339();

        // Generate mock IPs based on name hash
        let hash = name.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32));
        let private_ip = format!("10.128.0.{}", (hash % 256) as u8);
        let public_ip = format!("34.{}.{}.{}", (hash % 256) as u8, ((hash >> 8) % 256) as u8, ((hash >> 16) % 256) as u8);

        conn.execute(
            &format!(
                "INSERT INTO {} (
                    self_link, name, project_id, zone, machine_type, image, network,
                    private_ip, public_ip, status, created_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                Self::TABLE_GCP_INSTANCES
            ),
            params![
                self_link,
                name,
                project_id,
                zone,
                machine_type,
                image,
                network,
                private_ip,
                public_ip,
                "RUNNING",
                now,
            ],
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
        let conn = self.get_connection()?;

        conn.query_row(
            &format!(
                "SELECT name, project_id, zone, machine_type, image, network, private_ip, public_ip, status, created_at
                 FROM {} WHERE name = ? AND project_id = ? AND zone = ?",
                Self::TABLE_GCP_INSTANCES
            ),
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
        ).map_err(|_| EmulatorError::NotFound("GcpInstance".into(), name.into()))
    }
}
