//! AWS provider adapter implementing cloudemu-core traits.

use aws_control_spi::{
    CloudProvider, CloudProviderTrait, CloudResource, CloudResult, Request, Response,
    ResourceFilter, ServiceType, StorageEngine,
};
use axum::{
    body::Body,
    http::{Method, Request as HttpRequest},
};
use std::str::FromStr;
use std::sync::Arc;
use tower::ServiceExt; // for oneshot
use http_body_util::BodyExt; // for collecting body

/// AWS cloud provider implementation.
#[derive(Clone)]
pub struct AwsProvider {
    router: axum::Router,
}

impl AwsProvider {
    /// Create a new AWS provider with the given emulator.
    pub fn new(emulator: Arc<crate::Emulator>) -> Self {
        let router = crate::gateway::create_router(emulator);
        Self { router }
    }
}

#[async_trait::async_trait]
impl CloudProviderTrait for AwsProvider {
    async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // Convert cloudemu_spi::Request to http::Request
        let method = Method::from_str(&req.method)
            .map_err(|e| aws_control_spi::CloudError::Validation(format!("Invalid method: {}", e)))?;

        let mut builder = HttpRequest::builder()
            .method(method)
            .uri(&req.path);

        for (k, v) in req.headers {
            builder = builder.header(k, v);
        }

        let http_req = builder
            .body(Body::from(req.body))
            .map_err(|e| aws_control_spi::CloudError::Validation(format!("Invalid request: {}", e)))?;

        // Dispatch to Axum router
        // We clone the router because oneshot consumes the service, but Router is cheap to clone
        let response = self.router.clone()
            .oneshot(http_req)
            .await
            .map_err(|e| aws_control_spi::CloudError::Internal(format!("Router error: {}", e)))?;

        // Convert http::Response to cloudemu_spi::Response
        let status = response.status().as_u16();
        
        let mut headers = std::collections::HashMap::new();
        for (k, v) in response.headers() {
            if let Ok(val) = v.to_str() {
                headers.insert(k.to_string(), val.to_string());
            }
        }

        let body_bytes = response.into_body().collect().await
            .map_err(|e| aws_control_spi::CloudError::Internal(format!("Body error: {}", e)))?
            .to_bytes();

        Ok(Response {
            status,
            headers,
            body: body_bytes.to_vec(),
        })
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
        Err(aws_control_spi::CloudError::NotFound {
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
    use aws_data_core::Config;

    #[tokio::test]
    async fn test_aws_provider_creation() {
        let emulator = Arc::new(crate::Emulator::in_memory().unwrap());
        let provider = AwsProvider::new(emulator);

        assert_eq!(provider.provider_name(), "aws");
        assert_eq!(provider.default_port(), 4566);
        assert!(!provider.supported_services().is_empty());
    }

    #[tokio::test]
    async fn test_aws_provider_handle_request() {
        let emulator = Arc::new(crate::Emulator::in_memory().unwrap());
        let provider = AwsProvider::new(emulator);

        let req = Request {
            method: "GET".to_string(),
            path: "/health".to_string(), // Use health check which is registered in router
            headers: std::collections::HashMap::new(),
            body: vec![],
        };

        let response = provider.handle_request(req).await.unwrap();
        assert_eq!(response.status, 200);
        
        let body = String::from_utf8(response.body).unwrap();
        assert!(body.contains("running"));
    }
}
