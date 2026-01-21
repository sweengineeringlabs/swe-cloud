// CloudKit Types
// Type definitions for cloud toolkit feature

use rsc::prelude::*;

/// Cloud resource for CloudKit
#[derive(Clone, Debug)]
pub struct CloudResource {
    pub id: String,
    pub name: String,
    pub resource_type: String,
    pub provider: String,
    pub region: Option<String>,
    pub status: String,
    pub created_at: String,
    pub tags: HashMap<String, String>,
}

/// Cloud operation definition
#[derive(Clone, Debug)]
pub struct CloudOperation {
    pub id: String,
    pub name: String,
    pub service: String,
    pub description: String,
    pub parameters: Vec<OperationParameter>,
}

#[derive(Clone, Debug)]
pub struct OperationParameter {
    pub name: String,
    pub param_type: String,
    pub required: bool,
    pub description: String,
    pub default: Option<String>,
}

/// Operation execution result
#[derive(Clone, Debug)]
pub struct OperationResult {
    pub id: String,
    pub operation: String,
    pub status: OperationStatus,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub output: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OperationStatus {
    Pending,
    Running,
    Success,
    Failed,
}

/// API endpoint for explorer
#[derive(Clone, Debug)]
pub struct ApiEndpoint {
    pub service: String,
    pub operation: String,
    pub method: String,
    pub path: String,
    pub description: String,
    pub request_schema: Option<String>,
    pub response_schema: Option<String>,
}
