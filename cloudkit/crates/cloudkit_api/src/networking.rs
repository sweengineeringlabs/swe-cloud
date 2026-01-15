//! Networking trait for virtual network operations.

use cloudkit_spi::{CloudResult, ResourceId, VpcMetadata, SubnetMetadata, SecurityGroupMetadata};
use async_trait::async_trait;

/// Options for creating a VPC.
#[derive(Debug, Clone, Default)]
pub struct CreateVpcOptions {
    /// CIDR block
    pub cidr_block: String,
    /// Tags
    pub tags: std::collections::HashMap<String, String>,
}

/// Options for creating a subnet.
#[derive(Debug, Clone, Default)]
pub struct CreateSubnetOptions {
    /// VPC ID
    pub vpc_id: String,
    /// CIDR block
    pub cidr_block: String,
    /// Availability zone
    pub availability_zone: Option<String>,
    /// Tags
    pub tags: std::collections::HashMap<String, String>,
}

/// Networking service trait.
///
/// This trait abstracts virtual private network operations across cloud providers:
/// - AWS VPC
/// - Azure Virtual Network
/// - Google Cloud VPC
#[async_trait]
pub trait Networking: Send + Sync {
    // =========================================================================
    // VPC / VNet Operations
    // =========================================================================

    /// Create a new VPC/VNet.
    async fn create_vpc(&self, options: CreateVpcOptions) -> CloudResult<VpcMetadata>;

    /// Describe VPCs.
    async fn describe_vpcs(&self, ids: Option<Vec<ResourceId>>) -> CloudResult<Vec<VpcMetadata>>;

    /// Delete a VPC.
    async fn delete_vpc(&self, id: ResourceId) -> CloudResult<()>;

    // =========================================================================
    // Subnet Operations
    // =========================================================================

    /// Create a new subnet.
    async fn create_subnet(&self, options: CreateSubnetOptions) -> CloudResult<SubnetMetadata>;

    /// Describe subnets.
    async fn describe_subnets(&self, ids: Option<Vec<ResourceId>>) -> CloudResult<Vec<SubnetMetadata>>;

    /// Delete a subnet.
    async fn delete_subnet(&self, id: ResourceId) -> CloudResult<()>;

    // =========================================================================
    // Security Group Operations
    // =========================================================================

    /// Create a new security group.
    async fn create_security_group(
        &self,
        vpc_id: ResourceId,
        name: &str,
        description: Option<&str>,
    ) -> CloudResult<SecurityGroupMetadata>;

    /// Describe security groups.
    async fn describe_security_groups(&self, ids: Option<Vec<ResourceId>>) -> CloudResult<Vec<SecurityGroupMetadata>>;

    /// Delete a security group.
    async fn delete_security_group(&self, id: ResourceId) -> CloudResult<()>;
}
