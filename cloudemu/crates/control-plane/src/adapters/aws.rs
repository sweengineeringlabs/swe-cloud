//! AWS provider adapter implementing cloudemu-core traits.

use cloudemu_core::{
    CloudProvider, CloudProviderTrait, CloudResource, CloudResult, Request, Response,
    ResourceFilter, ServiceType, StorageEngine,
};
use std::sync::Arc;

/// AWS cloud provider implementation.
pub struct AwsProvider {
    _emulator: Arc<crate::Emulator>,
}

impl AwsProvider {
    /// Create a new AWS provider with the given emulator.
    pub fn new(emulator: Arc<crate::Emulator>) -> Self {
        Self { _emulator: emulator }
    }
}

#[async_trait::async_trait]
impl CloudProviderTrait for AwsProvider {
    async fn handle_request(&self, _req: Request) -> CloudResult<Response> {
        // For now, return a simple response
        // This will be integrated with existing AWS handlers in the next step
        Ok(Response::ok("AWS provider response"))
    }

    fn supported_services(&self) -> Vec<ServiceType> {
        vec![
            ServiceType::ObjectStorage,    // S3
            ServiceType::KeyValue,          // DynamoDB
            ServiceType::MessageQueue,      // SQS
            ServiceType::PubSub,            // SNS
            ServiceType::Functions,         // Lambda
            ServiceType::Secrets,           // Secrets Manager
            ServiceType::KeyManagement,     // KMS
            ServiceType::Events,            // EventBridge
            ServiceType::Monitoring,        // CloudWatch
            ServiceType::Identity,          // Cognito
            ServiceType::Workflows,         // Step Functions
        ]
    }

    fn default_port(&self) -> u16 {
        CloudProvider::Aws.default_port()
    }

    fn provider_name(&self) -> &str {
        "aws"
    }
}

/// Storage adapter that bridges data-plane Emulator to cloudemu-core StorageEngine.
pub struct AwsStorageAdapter {
    _emulator: Arc<crate::Emulator>,
}

impl AwsStorageAdapter {
    /// Create a new storage adapter.
    pub fn new(emulator: Arc<crate::Emulator>) -> Self {
        Self { _emulator: emulator }
    }
}

#[async_trait::async_trait]
impl StorageEngine for AwsStorageAdapter {
    async fn store(&self, _resource: CloudResource) -> CloudResult<()> {
        // This will be implemented to bridge to the existing storage engine
        // For now, return success
        Ok(())
    }

    async fn retrieve(&self, id: &str) -> CloudResult<CloudResource> {
        // This will be implemented to bridge to the existing storage engine
        Err(cloudemu_core::CloudError::NotFound {
            resource_type: "Resource".to_string(),
            resource_id: id.to_string(),
        })
    }

    async fn list(&self, _filter: ResourceFilter) -> CloudResult<Vec<CloudResource>> {
        // This will be implemented to bridge to the existing storage engine
        Ok(vec![])
    }

    async fn update(&self, _resource: CloudResource) -> CloudResult<()> {
        // This will be implemented to bridge to the existing storage engine
        Ok(())
    }

    async fn delete(&self, _id: &str) -> CloudResult<()> {
        // This will be implemented to bridge to the existing storage engine
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use data_plane::Config;

    #[tokio::test]
    async fn test_aws_provider_creation() {
        let config = Config::default();
        let emulator = Arc::new(crate::Emulator::with_config(config).unwrap());
        let provider = AwsProvider::new(emulator);

        assert_eq!(provider.provider_name(), "aws");
        assert_eq!(provider.default_port(), 4566);
        assert!(provider.supported_services().len() > 0);
    }

    #[tokio::test]
    async fn test_aws_provider_handle_request() {
        let config = Config::default();
        let emulator = Arc::new(crate::Emulator::with_config(config).unwrap());
        let provider = AwsProvider::new(emulator);

        let req = Request {
            method: "GET".to_string(),
            path: "/".to_string(),
            headers: std::collections::HashMap::new(),
            body: vec![],
        };

        let response = provider.handle_request(req).await.unwrap();
        assert_eq!(response.status, 200);
    }
}
