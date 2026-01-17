use zero_control_spi::{ComputeDriver, NetworkDriver, ZeroResult, WorkloadStatus, NetworkStatus};
use async_trait::async_trait;
use parking_lot::Mutex;
use std::collections::HashMap;

/// A mock compute driver that simulates workloads in-memory.
/// Useful for testing, CI, or unsupported environments.
pub struct MockComputeDriver {
    workloads: Mutex<HashMap<String, WorkloadStatus>>,
}

impl MockComputeDriver {
    pub fn new() -> Self {
        Self {
            workloads: Mutex::new(HashMap::new()),
        }
    }
}

/// A mock network driver that simulates networks in-memory.
#[derive(Default)]
pub struct MockNetworkDriver {
    networks: Mutex<HashMap<String, NetworkStatus>>,
}

impl MockNetworkDriver {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl NetworkDriver for MockNetworkDriver {
    async fn create_network(&self, id: &str, cidr: &str) -> ZeroResult<NetworkStatus> {
        let status = NetworkStatus {
            id: id.to_string(),
            cidr: cidr.to_string(),
            state: "Available".to_string(),
        };
        self.networks.lock().insert(id.to_string(), status.clone());
        Ok(status)
    }

    async fn delete_network(&self, id: &str) -> ZeroResult<()> {
        self.networks.lock().remove(id);
        Ok(())
    }

    async fn connect_workload(&self, _workload_id: &str, _network_id: &str) -> ZeroResult<String> {
        Ok("10.0.0.50".to_string())
    }
}

#[async_trait]
impl ComputeDriver for MockComputeDriver {
    async fn create_workload(&self, id: &str, _image: &str, _cpu: f32, _mem_mb: i32) -> ZeroResult<WorkloadStatus> {
        let status = WorkloadStatus {
            id: id.to_string(),
            state: "Running".to_string(),
            ip_address: Some("127.0.0.1".into()),
        };
        self.workloads.lock().insert(id.to_string(), status.clone());
        Ok(status)
    }

    async fn delete_workload(&self, id: &str) -> ZeroResult<()> {
        self.workloads.lock().remove(id);
        Ok(())
    }

    async fn get_workload_status(&self, id: &str) -> ZeroResult<WorkloadStatus> {
        self.workloads.lock().get(id).cloned()
            .ok_or_else(|| zero_control_spi::ZeroError::NotFound(id.to_string()))
    }
}
