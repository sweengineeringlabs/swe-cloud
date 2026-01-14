//! GCP provider implementation.

use cloudemu_core::{
    CloudProvider, CloudProviderTrait, CloudResult, Request, Response, ServiceType,
};

/// Google Cloud Platform provider.
pub struct GcpProvider;

impl GcpProvider {
    /// Create a new GCP provider.
    pub fn new() -> Self {
        Self
    }
}

impl Default for GcpProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl CloudProviderTrait for GcpProvider {
    async fn handle_request(&self, _req: Request) -> CloudResult<Response> {
        // TODO: Implement GCP request handling
        Ok(Response::ok("GCP provider response")
            .with_header("X-Provider", "gcp"))
    }

    fn supported_services(&self) -> Vec<ServiceType> {
        vec![
            ServiceType::ObjectStorage,    // Cloud Storage
            ServiceType::KeyValue,          // Firestore
            ServiceType::MessageQueue,      // Pub/Sub
            ServiceType::PubSub,            // Pub/Sub
            ServiceType::Functions,         // Cloud Functions
            ServiceType::Secrets,           // Secret Manager
            ServiceType::KeyManagement,     // Cloud KMS
            ServiceType::Events,            // Eventarc
            ServiceType::Monitoring,        // Cloud Monitoring
            ServiceType::Identity,          // Identity Platform
        ]
    }

    fn default_port(&self) -> u16 {
        CloudProvider::Gcp.default_port()
    }

    fn provider_name(&self) -> &str {
        "gcp"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gcp_provider_creation() {
        let provider = GcpProvider::new();
        assert_eq!(provider.provider_name(), "gcp");
        assert_eq!(provider.default_port(), 4568);
        assert!(!provider.supported_services().is_empty());
    }

    #[tokio::test]
    async fn test_gcp_provider_handle_request() {
        let provider = GcpProvider::new();
        
        let req = Request {
            method: "GET".to_string(),
            path: "/".to_string(),
            headers: std::collections::HashMap::new(),
            body: vec![],
        };

        let response = provider.handle_request(req).await.unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(response.headers.get("X-Provider"), Some(&"gcp".to_string()));
    }
}
