//! ZeroCloud trait definitions.

use crate::{
    NetworkStatus, NodeStats, VolumeStatus, WorkloadStatus, ZeroRequest, ZeroResponse, ZeroResult,
};
use async_trait::async_trait;

/// Trait for ZeroCloud compute drivers (Docker, KVM, etc.).
#[async_trait]
pub trait ComputeDriver: Send + Sync {
    /// Create a workload.
    async fn create_workload(
        &self,
        id: &str,
        image: &str,
        cpu: f32,
        mem_mb: i32,
    ) -> ZeroResult<WorkloadStatus>;
    /// Delete a workload.
    async fn delete_workload(&self, id: &str) -> ZeroResult<()>;
    /// Get workload status.
    async fn get_workload_status(&self, id: &str) -> ZeroResult<WorkloadStatus>;
    /// List all workloads.
    async fn list_workloads(&self) -> ZeroResult<Vec<WorkloadStatus>>;
    /// Get node statistics.
    async fn get_stats(&self) -> ZeroResult<NodeStats>;
}

/// Trait for ZeroCloud storage drivers (Local FS, NVMe, etc.).
#[async_trait]
pub trait StorageDriver: Send + Sync {
    /// Create a volume.
    async fn create_volume(&self, id: &str, size_gb: i32) -> ZeroResult<VolumeStatus>;
    /// Delete a volume.
    async fn delete_volume(&self, id: &str) -> ZeroResult<()>;
    /// Write a block of data.
    async fn write_block(&self, volume_id: &str, offset: u64, data: Vec<u8>) -> ZeroResult<()>;
    /// Read a block of data.
    async fn read_block(
        &self,
        volume_id: &str,
        offset: u64,
        length: u32,
    ) -> ZeroResult<Vec<u8>>;
    /// List all volumes.
    async fn list_volumes(&self) -> ZeroResult<Vec<VolumeStatus>>;
}

/// Trait for ZeroCloud networking drivers (Linux Bridge, OVS, Hyper-V Switch).
#[async_trait]
pub trait NetworkDriver: Send + Sync {
    /// Create a network.
    async fn create_network(&self, id: &str, cidr: &str) -> ZeroResult<NetworkStatus>;
    /// Delete a network.
    async fn delete_network(&self, id: &str) -> ZeroResult<()>;
    /// Connect a workload to a network. Returns the assigned IP.
    async fn connect_workload(
        &self,
        workload_id: &str,
        network_id: &str,
    ) -> ZeroResult<String>;
    /// List all networks.
    async fn list_networks(&self) -> ZeroResult<Vec<NetworkStatus>>;
}

/// Trait that all ZeroCloud services must implement.
#[async_trait]
pub trait ZeroService: Send + Sync {
    /// Handle an incoming request.
    async fn handle_request(&self, req: ZeroRequest) -> ZeroResult<ZeroResponse>;
}
