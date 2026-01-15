//! Azure provider implementation.

use crate::services::{
    blob::BlobService,
    cosmos::CosmosService,
    servicebus::ServiceBusService,
    functions::FunctionsService,
    keyvault::KeyVaultService,
};
use azure_control_spi::{
    CloudProvider, CloudProviderTrait, CloudResult, Request, Response, ServiceType,
};
use azure_data_core::storage::{StorageEngine, Config};
use std::sync::Arc;

/// Azure cloud provider.
pub struct AzureProvider {
    engine: Arc<StorageEngine>,
    blob: BlobService,
    cosmos: CosmosService,
    servicebus: ServiceBusService,
    functions: FunctionsService,
    keyvault: KeyVaultService,
}

impl Default for AzureProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AzureProvider {
    /// Create a new Azure provider.
    pub fn new() -> Self {
        let config = Config::from_env();
        let engine = Arc::new(StorageEngine::new(&config).expect("Failed to initialize storage engine"));
        
        Self {
            engine: engine.clone(),
            blob: BlobService::new(engine.clone()),
            cosmos: CosmosService::new(engine.clone()),
            servicebus: ServiceBusService::new(engine.clone()),
            functions: FunctionsService::new(engine.clone()),
            keyvault: KeyVaultService::new(engine.clone()),
        }
    }
}

#[async_trait::async_trait]
impl CloudProviderTrait for AzureProvider {
    async fn handle_request(&self, mut req: Request) -> CloudResult<Response> {
        // Dispatch based on path patterns
        
        // Cosmos DB (SQL API)
        if req.path.starts_with("/dbs") {
            return self.cosmos.handle_request(req).await;
        }

        // Azure Functions
        if req.path.starts_with("/api") || req.path.starts_with("/admin/functions") {
            return self.functions.handle_request(req).await;
        }

        // Key Vault
        if req.path.starts_with("/secrets") || req.path.starts_with("/keys") {
            return self.keyvault.handle_request(req).await;
        }

        // Service Bus (Simplified: /queue/messages or /topic/...)
        if req.path.contains("/messages") || req.path.starts_with("/queue") || req.path.starts_with("/topic") {
             return self.servicebus.handle_request(req).await;
        }

        // Default: Blob Storage
        // Assumes format: /<account>/<container>/<blob>
        // We need to strip the account name for the BlobService which expects /<container>/<blob>
        
        let path = req.path.clone();
        if path.starts_with('/') {
            if let Some(idx) = path[1..].find('/') {
                 // Split after /account
                 let (_account, rest) = path.split_at(idx + 1);
                 req.path = rest.to_string();
            } else {
                // /account -> /
                if !path.trim_matches('/').is_empty() {
                    req.path = "/".to_string();
                }
            }
        }
        
        self.blob.handle_request(req).await
    }

    fn supported_services(&self) -> Vec<ServiceType> {
        vec![
            ServiceType::ObjectStorage, // Blob
            ServiceType::KeyValue,      // Cosmos
            ServiceType::MessageQueue,  // Service Bus
            ServiceType::Functions,     // Functions
            ServiceType::Secrets,       // Key Vault
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
        assert!(provider.supported_services().len() >= 5);
    }

    #[tokio::test]
    async fn test_blob_routing() {
        let provider = AzureProvider::new();
        let req = Request {
            method: "GET".to_string(),
            path: "/devstoreaccount1/?comp=list".to_string(),
            headers: std::collections::HashMap::new(),
            body: vec![],
        };
        let response = provider.handle_request(req).await.unwrap();
        // Should reach BlobService
        let body = String::from_utf8(response.body).unwrap();
        assert!(body.contains("EnumerationResults"));
    }

    #[tokio::test]
    async fn test_cosmos_routing() {
        let provider = AzureProvider::new();
        let req = Request {
            method: "GET".to_string(),
            path: "/dbs".to_string(),
            headers: std::collections::HashMap::new(),
            body: vec![],
        };
        let response = provider.handle_request(req).await.unwrap();
        let body = String::from_utf8(response.body).unwrap();
        assert!(body.contains("_rid"));
    }
}
