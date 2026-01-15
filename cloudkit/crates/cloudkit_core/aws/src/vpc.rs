use async_trait::async_trait;
use cloudkit_api::{Networking, CreateVpcOptions, CreateSubnetOptions};
use cloudkit_spi::{CloudError, CloudResult, ResourceId, VpcMetadata, SubnetMetadata, SecurityGroupMetadata};
use cloudkit_spi::CloudContext;
use std::sync::Arc;

/// AWS VPC networking implementation.
pub struct VpcNetworking {
    _context: Arc<CloudContext>,
    client: aws_sdk_ec2::Client,
}

impl VpcNetworking {
    /// Create a new VPC networking client.
    pub fn new(context: Arc<CloudContext>, sdk_config: aws_config::SdkConfig) -> Self {
        let client = aws_sdk_ec2::Client::new(&sdk_config);
        Self { _context: context, client }
    }
}

#[async_trait]
impl Networking for VpcNetworking {
    async fn create_vpc(&self, options: CreateVpcOptions) -> CloudResult<VpcMetadata> {
        let resp = self.client.create_vpc()
            .cidr_block(options.cidr_block)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;

        let vpc = resp.vpc().ok_or_else(|| CloudError::ServiceError("No VPC in response".to_string()))?;

        Ok(VpcMetadata {
            id: ResourceId::new(vpc.vpc_id().unwrap_or_default()),
            cidr_block: vpc.cidr_block().unwrap_or_default().to_string(),
            state: vpc.state().map(|s| s.as_str().to_string()).unwrap_or_default(),
            is_default: vpc.is_default().unwrap_or(false),
            tags: vpc.tags().iter().map(|t| (t.key().unwrap_or_default().to_string(), t.value().unwrap_or_default().to_string())).collect(),
        })
    }

    async fn describe_vpcs(&self, ids: Option<Vec<ResourceId>>) -> CloudResult<Vec<VpcMetadata>> {
        let mut req = self.client.describe_vpcs();
        if let Some(ids) = ids {
            for id in ids {
                req = req.vpc_ids(id.to_string());
            }
        }

        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;

        Ok(resp.vpcs().iter().map(|v| {
            VpcMetadata {
                id: ResourceId::new(v.vpc_id().unwrap_or_default()),
                cidr_block: v.cidr_block().unwrap_or_default().to_string(),
                state: v.state().map(|s| s.as_str().to_string()).unwrap_or_default(),
                is_default: v.is_default().unwrap_or(false),
                tags: v.tags().iter().map(|t| (t.key().unwrap_or_default().to_string(), t.value().unwrap_or_default().to_string())).collect(),
            }
        }).collect())
    }

    async fn delete_vpc(&self, id: ResourceId) -> CloudResult<()> {
        self.client.delete_vpc()
            .vpc_id(id.to_string())
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn create_subnet(&self, options: CreateSubnetOptions) -> CloudResult<SubnetMetadata> {
        let mut req = self.client.create_subnet()
            .vpc_id(options.vpc_id)
            .cidr_block(options.cidr_block);

        if let Some(az) = options.availability_zone {
            req = req.availability_zone(az);
        }

        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        let s = resp.subnet().ok_or_else(|| CloudError::ServiceError("No subnet in response".to_string()))?;

        Ok(SubnetMetadata {
            id: ResourceId::new(s.subnet_id().unwrap_or_default()),
            vpc_id: ResourceId::new(s.vpc_id().unwrap_or_default()),
            cidr_block: s.cidr_block().unwrap_or_default().to_string(),
            availability_zone: s.availability_zone().unwrap_or_default().to_string(),
            tags: s.tags().iter().map(|t| (t.key().unwrap_or_default().to_string(), t.value().unwrap_or_default().to_string())).collect(),
        })
    }

    async fn describe_subnets(&self, ids: Option<Vec<ResourceId>>) -> CloudResult<Vec<SubnetMetadata>> {
        let mut req = self.client.describe_subnets();
        if let Some(ids) = ids {
            for id in ids {
                req = req.subnet_ids(id.to_string());
            }
        }

        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;

        Ok(resp.subnets().iter().map(|s| {
            SubnetMetadata {
                id: ResourceId::new(s.subnet_id().unwrap_or_default()),
                vpc_id: ResourceId::new(s.vpc_id().unwrap_or_default()),
                cidr_block: s.cidr_block().unwrap_or_default().to_string(),
                availability_zone: s.availability_zone().unwrap_or_default().to_string(),
                tags: s.tags().iter().map(|t| (t.key().unwrap_or_default().to_string(), t.value().unwrap_or_default().to_string())).collect(),
            }
        }).collect())
    }

    async fn delete_subnet(&self, id: ResourceId) -> CloudResult<()> {
        self.client.delete_subnet()
            .subnet_id(id.to_string())
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn create_security_group(
        &self,
        vpc_id: ResourceId,
        name: &str,
        description: Option<&str>,
    ) -> CloudResult<SecurityGroupMetadata> {
        let mut req = self.client.create_security_group()
            .group_name(name)
            .vpc_id(vpc_id.to_string());

        if let Some(desc) = description {
            req = req.description(desc);
        }

        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;

        Ok(SecurityGroupMetadata {
            id: ResourceId::new(resp.group_id().unwrap_or_default()),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            vpc_id: vpc_id,
            tags: std::collections::HashMap::new(),
        })
    }

    async fn describe_security_groups(&self, ids: Option<Vec<ResourceId>>) -> CloudResult<Vec<SecurityGroupMetadata>> {
        let mut req = self.client.describe_security_groups();
        if let Some(ids) = ids {
            for id in ids {
                req = req.group_ids(id.to_string());
            }
        }

        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;

        Ok(resp.security_groups().iter().map(|sg| {
            SecurityGroupMetadata {
                id: ResourceId::new(sg.group_id().unwrap_or_default()),
                name: sg.group_name().unwrap_or_default().to_string(),
                description: sg.description().map(|s| s.to_string()),
                vpc_id: ResourceId::new(sg.vpc_id().unwrap_or_default()),
                tags: sg.tags().iter().map(|t| (t.key().unwrap_or_default().to_string(), t.value().unwrap_or_default().to_string())).collect(),
            }
        }).collect())
    }

    async fn delete_security_group(&self, id: ResourceId) -> CloudResult<()> {
        self.client.delete_security_group()
            .group_id(id.to_string())
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }
}
