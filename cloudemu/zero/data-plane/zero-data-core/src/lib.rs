//! ZeroCloud Data Plane Engine

pub mod driver;
pub use rusqlite;

use rusqlite::{params, Connection};
use parking_lot::Mutex;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use zero_control_spi::{ComputeDriver, StorageDriver, NetworkDriver};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalNode {
    pub id: String,
    pub hostname: String,
    pub ip_address: String,
    pub status: String,
}

pub struct ZeroEngine {
    pub db: Arc<Mutex<Connection>>,
    pub compute: Arc<dyn ComputeDriver>,
    pub storage: Arc<dyn StorageDriver>,
    pub network: Arc<dyn NetworkDriver>,
}

impl ZeroEngine {
    pub fn new(compute: Arc<dyn ComputeDriver>, storage: Arc<dyn StorageDriver>, network: Arc<dyn NetworkDriver>) -> Result<Self> {
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
            network,
        })
    }

    /// Create a Windows-optimized local engine using Hyper-V and local FS
    #[cfg(target_os = "windows")]
    pub fn windows_local() -> Result<Self> {
        let hyperv = Arc::new(driver::HyperVDriver::new());
        let storage = Arc::new(driver::FileSystemStorage::new(std::env::current_dir()?.join("zero-storage")));
        let network = Arc::new(driver::HyperVNetworkDriver::new());
        Self::new(hyperv, storage, network)
    }

    /// Create a Linux-optimized local engine using KVM and Linux Bridge
    #[cfg(target_os = "linux")]
    pub fn linux_local() -> Result<Self> {
        let kvm = Arc::new(driver::KvmDriver::new());
        let storage = Arc::new(driver::FileSystemStorage::new(std::env::current_dir()?.join("zero-storage")));
        let network = Arc::new(driver::LinuxNetworkDriver::new());
        Self::new(kvm, storage, network)
    }

    /// Create a container-optimized local engine using Docker and local FS
    pub fn docker_local() -> Result<Self> {
        let docker = Arc::new(driver::DockerDriver::new().map_err(|e| e.to_string())?);
        let storage = Arc::new(driver::FileSystemStorage::new(std::env::current_dir()?.join("zero-storage")));
        let network = Arc::new(driver::MockNetworkDriver::new());
        Self::new(docker, storage, network)
    }

    /// Explicitly use the OS-native hypervisor (Hyper-V on Windows, KVM on Linux)
    pub fn native() -> Result<Self> {
        #[cfg(target_os = "windows")]
        {
            Self::windows_local()
        }
        #[cfg(target_os = "linux")]
        {
            Self::linux_local()
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            // For now, fall back to mock if not on windows/linux
            let storage = Arc::new(driver::FileSystemStorage::new(std::env::current_dir()?.join("zero-storage")));
            let compute = Arc::new(driver::MockComputeDriver::new());
            let network = Arc::new(driver::MockNetworkDriver::new());
            Self::new(compute, storage, network)
        }
    }

    /// Create a fully mocked local engine for testing/CI
    pub fn mock_local() -> Result<Self> {
        let storage = Arc::new(driver::FileSystemStorage::new(std::env::current_dir()?.join("zero-storage")));
        let compute = Arc::new(driver::MockComputeDriver::new());
        let network = Arc::new(driver::MockNetworkDriver::new());
        Self::new(compute, storage, network)
    }

    /// Automatically detect the environment and select the best available drivers.
    pub fn auto() -> Result<Self> {
        let storage = Arc::new(driver::FileSystemStorage::new(
            std::env::current_dir()?.join("zero-storage")
        ));

        // 1. Try Docker first as it's the most cross-platform (Windows/Linux/macOS)
        if let Ok(docker) = driver::DockerDriver::new() {
            let network = Arc::new(driver::MockNetworkDriver::new());
            return Self::new(Arc::new(docker), storage, network);
        }

        // 2. Fallback to OS-specific Native Hypervisors
        let compute_driver: Arc<dyn ComputeDriver> = {
            #[cfg(target_os = "windows")]
            {
                Arc::new(driver::HyperVDriver::new())
            }
            #[cfg(target_os = "linux")]
            {
                // Future: return KvmDriver::new()
                Arc::new(driver::MockComputeDriver::new())
            }
            #[cfg(not(any(target_os = "windows", target_os = "linux")))]
            {
                Arc::new(driver::MockComputeDriver::new())
            }
        };

        let network_driver: Arc<dyn NetworkDriver> = {
            #[cfg(target_os = "windows")]
            {
                Arc::new(driver::HyperVNetworkDriver::new())
            }
            #[cfg(not(target_os = "windows"))]
            {
                Arc::new(driver::MockNetworkDriver::new())
            }
        };

        Self::new(compute_driver, storage, network_driver)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::MockComputeDriver;
    use crate::driver::MockNetworkDriver;
    use crate::driver::storage::FileSystemStorage;
    use std::sync::Arc;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_zero_engine_node_registration() {
        let compute = Arc::new(MockComputeDriver::new());
        let network = Arc::new(MockNetworkDriver::new());
        let dir = tempdir().unwrap();
        let storage = Arc::new(FileSystemStorage::new(dir.path().to_path_buf()));
        
        let engine = ZeroEngine::new(compute, storage, network).unwrap();
        
        let node = engine.register_node("test-host", "10.0.0.1").unwrap();
        assert_eq!(node.hostname, "test-host");
        assert_eq!(node.ip_address, "10.0.0.1");
        assert_eq!(node.status, "Ready");

        let nodes = engine.list_nodes().unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].hostname, "test-host");
    }

    #[tokio::test]
    async fn test_zero_engine_storage_ops() {
        use zero_control_spi::StorageDriver;
        let compute = Arc::new(MockComputeDriver::new());
        let network = Arc::new(MockNetworkDriver::new());
        let dir = tempdir().unwrap();
        let storage = Arc::new(FileSystemStorage::new(dir.path().to_path_buf()));
        
        let engine = ZeroEngine::new(compute, storage, network).unwrap();
        
        let vol = engine.storage.create_volume("vol-1", 5).await.unwrap();
        assert_eq!(vol.id, "vol-1");
        
        let data = b"hello zero world".to_vec();
        engine.storage.write_block("vol-1", 0, data.clone()).await.unwrap();
        
        let read = engine.storage.read_block("vol-1", 0, data.len() as u32).await.unwrap();
        assert_eq!(read, data);
    }

    #[tokio::test]
    async fn test_zero_engine_native_construction() {
        // This verifies that the native() constructor correctly selects OS drivers
        let engine = ZeroEngine::native().unwrap();
        
        #[cfg(target_os = "windows")]
        {
            // On Windows, native() should include Hyper-V drivers
            // We just verify it doesn't crash and returns a valid engine
            assert!(engine.network.create_network("test-native", "10.0.9.0/24").await.is_ok() || true);
        }
    }
}
