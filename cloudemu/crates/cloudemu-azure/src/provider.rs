//! Azure provider implementation.

use cloudemu_core::{
    CloudProvider, CloudProviderTrait, CloudResult, Request, Response, ServiceType,
};

/// Azure cloud provider.
pub struct AzureProvider;

impl AzureProvider {
    /// Create a new Azure provider.
    pub fn new() -> Self {
        Self
    }
}

impl Default for AzureProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl CloudProviderTrait for AzureProvider {
    async fn handle_request(&self, _req: Request) -> CloudResult<Response> {
        // TODO: Implement Azure request handling
        Ok(Response::ok("Azure provider response")
            .with_header("X-Provider", "azure"))
    }

    fn supported_services(&self) -> Vec<ServiceType> {
        vec![
            ServiceType::ObjectStorage,    // Blob Storage
            ServiceType::KeyValue,          // Cosmos DB
            ServiceType::MessageQueue,      // Service Bus Queue
            ServiceType::PubSub,            // Event Grid
            ServiceType::Functions,         // Azure Functions
            ServiceType::Secrets,           // Key Vault
            ServiceType::KeyManagement,     // Key Vault
            ServiceType::Events,            // Event Grid
            ServiceType::Monitoring,        // Azure Monitor
            ServiceType::Identity,          // Azure AD
        ]
    }

    fn default_port(&self) -> u16 {
        CloudProvider::Azure.default_port()
    }

    fn provider_name(&self) -> &str {
        "azure"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_azure_provider_creation() {
        let provider = AzureProvider::new();
        assert_eq!(provider.provider_name(), "azure");
        assert_eq!(provider.default_port(), 4567);
        assert!(provider.supported_services().len() > 0);
    }

    #[tokio::test]
    async fn test_azure_provider_handle_request() {
        let provider = AzureProvider::new();
        
        let req = Request {
            method: "GET".to_string(),
            path: "/".to_string(),
            headers: std::collections::HashMap::new(),
            body: vec![],
        };

        let response = provider.handle_request(req).await.unwrap();
        assert_eq!(response.status, 200);
        assert_eq!(response.headers.get("X-Provider"), Some(&"azure".to_string()));
    }
}
