//! Compute trait for virtual machine operations.

use cloudkit_spi::{CloudResult, InstanceMetadata, ResourceId};
use async_trait::async_trait;

/// Options for running instances.
#[derive(Debug, Clone, Default)]
pub struct RunInstanceOptions {
    /// Instance type/size
    pub instance_type: String,
    /// Image ID (AMI)
    pub image_id: String,
    /// Key pair name
    pub key_name: Option<String>,
    /// Subnet ID
    pub subnet_id: Option<String>,
    /// Security group IDs
    pub security_group_ids: Vec<String>,
    /// User data (startup scripts)
    pub user_data: Option<String>,
    /// Tags
    pub tags: std::collections::HashMap<String, String>,
}

impl RunInstanceOptions {
    /// Create new run instance options.
    pub fn new(instance_type: impl Into<String>, image_id: impl Into<String>) -> Self {
        Self {
            instance_type: instance_type.into(),
            image_id: image_id.into(),
            ..Default::default()
        }
    }

    /// Set key name.
    pub fn key_name(mut self, name: impl Into<String>) -> Self {
        self.key_name = Some(name.into());
        self
    }

    /// Set subnet ID.
    pub fn subnet_id(mut self, id: impl Into<String>) -> Self {
        self.subnet_id = Some(id.into());
        self
    }

    /// Add security group ID.
    pub fn security_group(mut self, id: impl Into<String>) -> Self {
        self.security_group_ids.push(id.into());
        self
    }

    /// Set user data.
    pub fn user_data(mut self, data: impl Into<String>) -> Self {
        self.user_data = Some(data.into());
        self
    }

    /// Add tag.
    pub fn tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.tags.insert(key.into(), value.into());
        self
    }
}

/// Compute service trait.
///
/// This trait abstracts virtual machine operations across cloud providers:
/// - AWS EC2
/// - Azure Virtual Machines
/// - Google Compute Engine
#[async_trait]
pub trait Compute: Send + Sync {
    /// Run one or more instances.
    async fn run_instances(&self, options: RunInstanceOptions) -> CloudResult<Vec<InstanceMetadata>>;

    /// Describe instances.
    async fn describe_instances(&self, ids: Option<Vec<ResourceId>>) -> CloudResult<Vec<InstanceMetadata>>;

    /// Terminate instances.
    async fn terminate_instances(&self, ids: Vec<ResourceId>) -> CloudResult<()>;

    /// Start instances.
    async fn start_instances(&self, ids: Vec<ResourceId>) -> CloudResult<()>;

    /// Stop instances.
    async fn stop_instances(&self, ids: Vec<ResourceId>) -> CloudResult<()>;

    /// Create a new key pair.
    async fn create_key_pair(&self, name: &str) -> CloudResult<String>;

    /// Delete a key pair.
    async fn delete_key_pair(&self, name: &str) -> CloudResult<()>;
}
