//! ZeroCloud Control SPI

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;

/// ZeroCloud Result type
pub type ZeroResult<T> = Result<T, ZeroError>;

/// ZeroCloud Error types
#[derive(Debug, thiserror::Error)]
pub enum ZeroError {
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Resource already exists: {0}")]
    AlreadyExists(String),

    #[error("Driver error: {0}")]
    Driver(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}

/// Generic HTTP-like request for ZeroCloud services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

/// Generic HTTP-like response for ZeroCloud services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl ZeroResponse {
    pub fn ok(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    pub fn error(msg: &str) -> Self {
        Self {
            status: 500,
            headers: HashMap::new(),
            body: msg.as_bytes().to_vec(),
        }
    }
    
    pub fn json(val: serde_json::Value) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        Self {
            status: 200,
            headers,
            body: val.to_string().into_bytes(),
        }
    }

    pub fn json_bytes(body: Vec<u8>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        Self {
            status: 200,
            headers,
            body,
        }
    }
}

/// Trait for ZeroCloud compute drivers (Docker, KVM, etc.)
#[async_trait]
pub trait ComputeDriver: Send + Sync {
    async fn create_workload(&self, id: &str, image: &str, cpu: f32, mem_mb: i32) -> ZeroResult<WorkloadStatus>;
    async fn delete_workload(&self, id: &str) -> ZeroResult<()>;
    async fn get_workload_status(&self, id: &str) -> ZeroResult<WorkloadStatus>;
    async fn list_workloads(&self) -> ZeroResult<Vec<WorkloadStatus>>;
    async fn get_stats(&self) -> ZeroResult<NodeStats>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStats {
    pub cpu_usage_percent: f32,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub storage_used_gb: u64,
    pub storage_total_gb: u64,
}

/// Trait for ZeroCloud storage drivers (Local FS, NVMe, etc.)
#[async_trait]
pub trait StorageDriver: Send + Sync {
    async fn create_volume(&self, id: &str, size_gb: i32) -> ZeroResult<VolumeStatus>;
    async fn delete_volume(&self, id: &str) -> ZeroResult<()>;
    async fn write_block(&self, volume_id: &str, offset: u64, data: Vec<u8>) -> ZeroResult<()>;
    async fn read_block(&self, volume_id: &str, offset: u64, length: u32) -> ZeroResult<Vec<u8>>;
    async fn list_volumes(&self) -> ZeroResult<Vec<VolumeStatus>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadStatus {
    pub id: String,
    pub state: String, // Running, Stopped, Failed
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeStatus {
    pub id: String,
    pub path: String,
    pub state: String, // Available, InUse
}

/// Trait for ZeroCloud networking drivers (Linux Bridge, OVS, Hyper-V Switch)
#[async_trait]
pub trait NetworkDriver: Send + Sync {
    async fn create_network(&self, id: &str, cidr: &str) -> ZeroResult<NetworkStatus>;
    async fn delete_network(&self, id: &str) -> ZeroResult<()>;
    async fn connect_workload(&self, workload_id: &str, network_id: &str) -> ZeroResult<String>; // Returns assigned IP
    async fn list_networks(&self) -> ZeroResult<Vec<NetworkStatus>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    pub id: String,
    pub cidr: String,
    pub state: String, // Available, Failed
}

/// Trait that all ZeroCloud services must implement
#[async_trait]
pub trait ZeroService: Send + Sync {
    async fn handle_request(&self, req: ZeroRequest) -> ZeroResult<ZeroResponse>;
}
