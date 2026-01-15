use async_trait::async_trait;
use cloudkit_api::{Compute, RunInstanceOptions};
use cloudkit_spi::{CloudError, CloudResult, InstanceMetadata, ResourceId};
use cloudkit_spi::CloudContext;
use std::sync::Arc;
use std::collections::HashMap;

/// AWS EC2 compute implementation.
pub struct Ec2Compute {
    _context: Arc<CloudContext>,
    client: aws_sdk_ec2::Client,
}

impl Ec2Compute {
    /// Create a new EC2 compute client.
    pub fn new(context: Arc<CloudContext>, sdk_config: aws_config::SdkConfig) -> Self {
        let client = aws_sdk_ec2::Client::new(&sdk_config);
        Self { _context: context, client }
    }
}

#[async_trait]
impl Compute for Ec2Compute {
    async fn run_instances(&self, options: RunInstanceOptions) -> CloudResult<Vec<InstanceMetadata>> {
        let mut req = self.client.run_instances()
            .instance_type(aws_sdk_ec2::types::InstanceType::from(options.instance_type.as_str()))
            .image_id(options.image_id)
            .min_count(1)
            .max_count(1);

        if let Some(key) = options.key_name {
            req = req.key_name(key);
        }

        if let Some(subnet) = options.subnet_id {
            req = req.subnet_id(subnet);
        }

        for sg in options.security_group_ids {
            req = req.security_group_ids(sg);
        }

        if let Some(data) = options.user_data {
            req = req.user_data(base64::Engine::encode(&base64::prelude::BASE64_STANDARD, data));
        }

        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;

        let instances = resp.instances().iter().map(|i| {
            InstanceMetadata {
                id: ResourceId::new(i.instance_id().unwrap_or_default()),
                instance_type: i.instance_type().map(|t| t.as_str().to_string()).unwrap_or_default(),
                state: i.state().map(|s| s.name().map(|n| n.as_str().to_string()).unwrap_or_default()).unwrap_or_default(),
                private_ip: i.private_ip_address().map(|s| s.to_string()),
                public_ip: i.public_ip_address().map(|s| s.to_string()),
                vpc_id: i.vpc_id().map(|s| ResourceId::new(s)),
                subnet_id: i.subnet_id().map(|s| ResourceId::new(s)),
                launch_time: i.launch_time().map(|d| chrono::DateTime::<chrono::Utc>::from_timestamp(d.secs(), 0).unwrap_or_default()).unwrap_or_default(),
                tags: i.tags().iter().map(|t| (t.key().unwrap_or_default().to_string(), t.value().unwrap_or_default().to_string())).collect(),
            }
        }).collect();

        Ok(instances)
    }

    async fn describe_instances(&self, ids: Option<Vec<ResourceId>>) -> CloudResult<Vec<InstanceMetadata>> {
        let mut req = self.client.describe_instances();
        if let Some(ids) = ids {
            for id in ids {
                req = req.instance_ids(id.to_string());
            }
        }

        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        let mut instances = Vec::new();
        for reservation in resp.reservations() {
            for i in reservation.instances() {
                instances.push(InstanceMetadata {
                    id: ResourceId::new(i.instance_id().unwrap_or_default()),
                    instance_type: i.instance_type().map(|t| t.as_str().to_string()).unwrap_or_default(),
                    state: i.state().map(|s| s.name().map(|n| n.as_str().to_string()).unwrap_or_default()).unwrap_or_default(),
                    private_ip: i.private_ip_address().map(|s| s.to_string()),
                    public_ip: i.public_ip_address().map(|s| s.to_string()),
                    vpc_id: i.vpc_id().map(|s| ResourceId::new(s)),
                    subnet_id: i.subnet_id().map(|s| ResourceId::new(s)),
                    launch_time: i.launch_time().map(|d| chrono::DateTime::<chrono::Utc>::from_timestamp(d.secs(), 0).unwrap_or_default()).unwrap_or_default(),
                    tags: i.tags().iter().map(|t| (t.key().unwrap_or_default().to_string(), t.value().unwrap_or_default().to_string())).collect(),
                });
            }
        }

        Ok(instances)
    }

    async fn terminate_instances(&self, ids: Vec<ResourceId>) -> CloudResult<()> {
        let mut req = self.client.terminate_instances();
        for id in ids {
            req = req.instance_ids(id.to_string());
        }

        req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn start_instances(&self, ids: Vec<ResourceId>) -> CloudResult<()> {
        let mut req = self.client.start_instances();
        for id in ids {
            req = req.instance_ids(id.to_string());
        }

        req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn stop_instances(&self, ids: Vec<ResourceId>) -> CloudResult<()> {
        let mut req = self.client.stop_instances();
        for id in ids {
            req = req.instance_ids(id.to_string());
        }

        req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn create_key_pair(&self, name: &str) -> CloudResult<String> {
        let resp = self.client.create_key_pair()
            .key_name(name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;

        Ok(resp.key_material().unwrap_or_default().to_string())
    }

    async fn delete_key_pair(&self, name: &str) -> CloudResult<()> {
        self.client.delete_key_pair()
            .key_name(name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }
}
