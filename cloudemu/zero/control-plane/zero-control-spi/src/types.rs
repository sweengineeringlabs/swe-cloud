//! ZeroCloud types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Generic HTTP-like request for ZeroCloud services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroRequest {
    /// HTTP method
    pub method: String,
    /// Request path
    pub path: String,
    /// Headers
    pub headers: HashMap<String, String>,
    /// Request body
    pub body: Vec<u8>,
}

/// Generic HTTP-like response for ZeroCloud services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroResponse {
    /// HTTP status code
    pub status: u16,
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Response body
    pub body: Vec<u8>,
}

impl ZeroResponse {
    /// Create a successful response.
    pub fn ok(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    /// Create an error response.
    pub fn error(msg: &str) -> Self {
        Self {
            status: 500,
            headers: HashMap::new(),
            body: msg.as_bytes().to_vec(),
        }
    }

    /// Create a JSON response.
    pub fn json(val: serde_json::Value) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        Self {
            status: 200,
            headers,
            body: val.to_string().into_bytes(),
        }
    }

    /// Create a JSON response from raw bytes.
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

/// Workload status information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadStatus {
    /// Workload identifier
    pub id: String,
    /// Current state (Running, Stopped, Failed)
    pub state: String,
    /// Assigned IP address
    pub ip_address: Option<String>,
}

/// Volume status information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeStatus {
    /// Volume identifier
    pub id: String,
    /// Volume path
    pub path: String,
    /// Current state (Available, InUse)
    pub state: String,
}

/// Network status information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    /// Network identifier
    pub id: String,
    /// Network CIDR
    pub cidr: String,
    /// Current state (Available, Failed)
    pub state: String,
}

/// Node statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStats {
    /// CPU usage percentage
    pub cpu_usage_percent: f32,
    /// Memory used in MB
    pub memory_used_mb: u64,
    /// Total memory in MB
    pub memory_total_mb: u64,
    /// Storage used in GB
    pub storage_used_gb: u64,
    /// Total storage in GB
    pub storage_total_gb: u64,
}
