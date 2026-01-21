// CloudEmu Types
// Type definitions for cloud emulation feature

use rsc::prelude::*;

/// Cloud provider representation
#[derive(Clone, Debug)]
pub struct Provider {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub color: String,
    pub endpoint: String,
    pub services: Vec<Service>,
}

/// Cloud service representation
#[derive(Clone, Debug)]
pub struct Service {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub description: Option<String>,
}

/// Cloud resource representation
#[derive(Clone, Debug)]
pub struct Resource {
    pub id: String,
    pub name: String,
    pub service: String,
    pub provider: String,
    pub created_at: String,
    pub updated_at: String,
    pub status: ResourceStatus,
    pub metadata: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ResourceStatus {
    Active,
    Creating,
    Deleting,
    Error,
}

/// Request log entry
#[derive(Clone, Debug)]
pub struct RequestLog {
    pub id: String,
    pub method: String,
    pub path: String,
    pub provider: String,
    pub service: String,
    pub status: u16,
    pub duration_ms: u32,
    pub timestamp: String,
    pub request_headers: HashMap<String, String>,
    pub request_body: Option<String>,
    pub response_headers: HashMap<String, String>,
    pub response_body: Option<String>,
}

/// Request log filter
#[derive(Clone, Default)]
pub struct LogFilter {
    pub method: Option<String>,
    pub status: Option<String>,
    pub provider: Option<String>,
    pub search: Option<String>,
}

/// Service health status
#[derive(Clone, Debug)]
pub struct ServiceHealth {
    pub provider: String,
    pub status: HealthStatus,
    pub latency_ms: u32,
    pub last_check: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}
