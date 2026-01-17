//! ZeroCloud Data Plane Engine

pub mod drivers;

use rusqlite::{params, Connection};
use parking_lot::Mutex;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use zero_control_spi::{ComputeDriver, StorageDriver};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalNode {
    pub id: String,
    pub hostname: String,
    pub ip_address: String,
    pub status: String,
}

pub struct ZeroEngine {
    db: Arc<Mutex<Connection>>,
    pub compute: Arc<dyn ComputeDriver>,
    pub storage: Arc<dyn StorageDriver>,
}

impl ZeroEngine {
    pub fn new(compute: Arc<dyn ComputeDriver>, storage: Arc<dyn StorageDriver>) -> Result<Self> {
        let conn = Connection::open_in_memory()?; 
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS nodes (
                id TEXT PRIMARY KEY,
                hostname TEXT NOT NULL,
                ip_address TEXT NOT NULL,
                status TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self {
            db: Arc::new(Mutex::new(conn)),
            compute,
            storage,
        })
    }

    /// Create a Windows-optimized local engine using Hyper-V and local FS
    pub fn windows_local() -> Result<Self> {
        let hyperv = Arc::new(drivers::HyperVDriver::new());
        let storage = Arc::new(drivers::FileSystemStorage::new(std::env::current_dir()?.join("zero-storage")));
        Self::new(hyperv, storage)
    }

    /// Create a container-optimized local engine using Docker and local FS
    pub fn docker_local() -> Result<Self> {
        let docker = Arc::new(drivers::DockerDriver::new().map_err(|e| e.to_string())?);
        let storage = Arc::new(drivers::FileSystemStorage::new(std::env::current_dir()?.join("zero-storage")));
        Self::new(docker, storage)
    }

    /// Automatically detect the environment and select the best available drivers.
    pub fn auto() -> Result<Self> {
        let storage = Arc::new(drivers::FileSystemStorage::new(
            std::env::current_dir()?.join("zero-storage")
        ));

        // 1. Try Docker first as it's the most cross-platform (Windows/Linux/macOS)
        if let Ok(docker) = drivers::DockerDriver::new() {
            return Self::new(Arc::new(docker), storage);
        }

        // 2. Fallback to OS-specific Native Hypervisors
        let compute_driver: Arc<dyn ComputeDriver> = {
            #[cfg(target_os = "windows")]
            {
                Arc::new(drivers::HyperVDriver::new())
            }
            #[cfg(target_os = "linux")]
            {
                // Future: return KvmDriver::new()
                Arc::new(drivers::MockComputeDriver::new())
            }
            #[cfg(not(any(target_os = "windows", target_os = "linux")))]
            {
                Arc::new(drivers::MockComputeDriver::new())
            }
        };

        Self::new(compute_driver, storage)
    }

    pub fn register_node(&self, hostname: &str, ip: &str) -> Result<LocalNode> {
        let conn = self.db.lock();
        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO nodes (id, hostname, ip_address, status) VALUES (?1, ?2, ?3, ?4)",
            params![id, hostname, ip, "Ready"],
        )?;
        Ok(LocalNode {
            id,
            hostname: hostname.to_string(),
            ip_address: ip.to_string(),
            status: "Ready".to_string(),
        })
    }

    pub fn list_nodes(&self) -> Result<Vec<LocalNode>> {
        let conn = self.db.lock();
        let mut stmt = conn.prepare("SELECT id, hostname, ip_address, status FROM nodes")?;
        let nodes = stmt.query_map([], |row| {
            Ok(LocalNode {
                id: row.get(0)?,
                hostname: row.get(1)?,
                ip_address: row.get(2)?,
                status: row.get(3)?,
            })
        })?.collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(nodes)
    }
}
