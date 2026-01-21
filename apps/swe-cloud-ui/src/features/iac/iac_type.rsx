// IAC Types
// Type definitions for Infrastructure as Code feature

use rsc::prelude::*;

/// Terraform module definition
#[derive(Clone, Debug)]
pub struct TerraformModule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub source: String,
    pub variables: Vec<ModuleVariable>,
    pub outputs: Vec<ModuleOutput>,
}

#[derive(Clone, Debug)]
pub struct ModuleVariable {
    pub name: String,
    pub var_type: String,
    pub description: String,
    pub default: Option<String>,
    pub required: bool,
}

#[derive(Clone, Debug)]
pub struct ModuleOutput {
    pub name: String,
    pub description: String,
}

/// Deployment record
#[derive(Clone, Debug)]
pub struct Deployment {
    pub id: String,
    pub module_id: String,
    pub module_name: String,
    pub environment: String,
    pub status: DeploymentStatus,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub created_by: String,
    pub changes: ChangesSummary,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DeploymentStatus {
    Pending,
    Planning,
    AwaitingApproval,
    Applying,
    Success,
    Failed,
    Cancelled,
}

#[derive(Clone, Debug)]
pub struct ChangesSummary {
    pub add: u32,
    pub change: u32,
    pub destroy: u32,
}

/// Terraform plan
#[derive(Clone, Debug)]
pub struct TerraformPlan {
    pub id: String,
    pub module_id: String,
    pub environment: String,
    pub status: PlanStatus,
    pub created_at: String,
    pub created_by: String,
    pub changes: ChangesSummary,
    pub output: String,
    pub resource_changes: Vec<ResourceChange>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlanStatus {
    Pending,
    Running,
    Ready,
    Approved,
    Rejected,
    Applied,
    Expired,
}

#[derive(Clone, Debug)]
pub struct ResourceChange {
    pub address: String,
    pub action: ChangeAction,
    pub resource_type: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ChangeAction {
    Create,
    Update,
    Delete,
    Replace,
    NoOp,
}

/// Terraform state
#[derive(Clone, Debug)]
pub struct TerraformState {
    pub version: u32,
    pub serial: u64,
    pub resources: Vec<StateResource>,
}

#[derive(Clone, Debug)]
pub struct StateResource {
    pub address: String,
    pub resource_type: String,
    pub name: String,
    pub provider: String,
    pub attributes: HashMap<String, String>,
}
