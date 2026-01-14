//! Azure provider implementation.

use crate::services::blob::BlobService;
use cloudemu_core::{
    CloudProvider, CloudProviderTrait, CloudResult, Request, Response, ServiceType,
};
use std::sync::Arc;

/// Azure cloud provider.
pub struct AzureProvider {
    blob_service: BlobService,
}

impl AzureProvider {
    /// Create a new Azure provider.
    pub fn new() -> Self {
        Self {
            blob_service: BlobService::new(),
        }
    }
}

#[async_trait::async_trait]
impl CloudProviderTrait for AzureProvider {
    async fn handle_request(&self, mut req: Request) -> CloudResult<Response> {
        // Logic to handle "local emulator" style URLs (http://host:ip/account/container/blob)
        // We strip the account name (first path segment) so BlobService sees /container/blob
        
        // Handle URL query parameters that might define service type? No, usually port based.
        // Assuming Blob Service for now.

        // Strip account prefix
        // 1. Check path
        if req.path.starts_with('/') {
            // Find second slash
            // /account/container -> /container
            if let Some(idx) = req.path[1..].find('/') {
                 // idx is index relative to [1..], so real index is idx + 1 + 1? No.
                 // path[1..] is "account/container". find returns index of '/'.
                 // split_at(idx + 2)
                 
                 let (_account, rest) = req.path.split_at(idx + 1); // +1 because we skipped first slash
                 // req.path = rest.to_string(); 
                 
                 // Wait. /account/container. 
                 // chars: / a c c o u n t / c...
                 // idx of / is 0.
                 // idx of 2nd / is 8.
                 // split_at(8). 0..8 is /account. 8.. is /container...
                 
                 req.path = rest.to_string();
            } else {
                // /account -> / (Root listing for account?)
                req.path = "/".to_string();
            }
        }
        
        self.blob_service.handle_request(req).await
    }

    fn supported_services(&self) -> Vec<ServiceType> {
        vec![
            ServiceType::ObjectStorage, // Blob
            ServiceType::KeyValue,      // Cosmos
            ServiceType::MessageQueue,  // Service Bus
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
        
        // Emulate typical generic emulator request: /accountname/?comp=list
        let req = Request {
            method: "GET".to_string(),
            path: "/devstoreaccount1/?comp=list".to_string(),
            headers: std::collections::HashMap::new(),
            body: vec![],
        };

        let response = provider.handle_request(req).await.unwrap();
        assert_eq!(response.status, 200);
        let body = String::from_utf8(response.body).unwrap();
        assert!(body.contains("EnumerationResults"));
    }

    #[tokio::test]
    async fn test_create_container() {
        let provider = AzureProvider::new();
        let req = Request {
            method: "PUT".to_string(),
            path: "/devstoreaccount1/mycontainer?restype=container".to_string(),
            headers: std::collections::HashMap::new(),
            body: vec![],
        };
        
        let response = provider.handle_request(req).await.unwrap();
        assert_eq!(response.status, 201);
    }

    #[tokio::test]
    async fn test_put_blob() {
        let provider = AzureProvider::new();
        let req = Request {
            method: "PUT".to_string(),
            path: "/devstoreaccount1/mycontainer/blob.txt".to_string(),
            headers: std::collections::HashMap::new(),
            body: b"hello azure".to_vec(),
        };
        
        let response = provider.handle_request(req).await.unwrap();
        assert_eq!(response.status, 201);
    }
}
