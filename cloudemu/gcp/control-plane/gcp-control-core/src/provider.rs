//! GCP provider implementation.

use crate::services::{
    cloudstorage::CloudStorageService,
    firestore::FirestoreService,
    pubsub::PubSubService,
    functions::CloudFunctionsService,
    secretmanager::SecretManagerService,
    billing::CloudBillingService,
};
use gcp_control_spi::{
    CloudProvider, CloudProviderTrait, CloudResult, Request, Response, ServiceType,
};
use gcp_data_core::storage::{StorageEngine, Config};
use std::sync::Arc;

/// Google Cloud Platform provider.
pub struct GcpProvider {
    #[allow(dead_code)]
    engine: Arc<StorageEngine>,
    storage: CloudStorageService,
    firestore: FirestoreService,
    pubsub: PubSubService,
    functions: CloudFunctionsService,
    secret_manager: SecretManagerService,
    billing: CloudBillingService,
    compute: crate::services::compute::ComputeService,
    sql: crate::services::sql::SqlService,
    iam: crate::services::iam::IamService,
    dns: crate::services::dns::DnsService,
    monitoring: crate::services::monitoring::MonitoringService,
    workflows: crate::services::workflows::WorkflowsService,
    networking: crate::services::networking::NetworkingService,
    run: crate::services::run::CloudRunService,
    kms: crate::services::kms::KmsService,
}

impl GcpProvider {
    /// Create a new GCP provider.
    pub fn new() -> Self {
        let config = Config::from_env();
        let engine = Arc::new(StorageEngine::new(&config).expect("Failed to initialize storage engine"));
        
        Self {
            engine: engine.clone(),
            storage: CloudStorageService::new(engine.clone()),
            firestore: FirestoreService::new(engine.clone()),
            pubsub: PubSubService::new(engine.clone()),
            functions: CloudFunctionsService::new(engine.clone()),
            secret_manager: SecretManagerService::new(engine.clone()),
            billing: CloudBillingService::new(engine.clone()),
            compute: crate::services::compute::ComputeService::new(engine.clone()),
            sql: crate::services::sql::SqlService::new(engine.clone()),
            iam: crate::services::iam::IamService::new(engine.clone()),
            dns: crate::services::dns::DnsService::new(engine.clone()),
            monitoring: crate::services::monitoring::MonitoringService::new(engine.clone()),
            workflows: crate::services::workflows::WorkflowsService::new(engine.clone()),
            networking: crate::services::networking::NetworkingService::new(engine.clone()),
            run: crate::services::run::CloudRunService::new(engine.clone()),
            kms: crate::services::kms::KmsService::new(engine.clone()),
        }
    }

    /// Create a new in-memory GCP provider for testing
    pub fn in_memory() -> Self {
         let engine = Arc::new(StorageEngine::in_memory().expect("Failed to create in-memory engine"));

         Self {
            engine: engine.clone(),
            storage: CloudStorageService::new(engine.clone()),
            firestore: FirestoreService::new(engine.clone()),
            pubsub: PubSubService::new(engine.clone()),
            functions: CloudFunctionsService::new(engine.clone()),
            secret_manager: SecretManagerService::new(engine.clone()),
            billing: CloudBillingService::new(engine.clone()),
            compute: crate::services::compute::ComputeService::new(engine.clone()),
            sql: crate::services::sql::SqlService::new(engine.clone()),
            iam: crate::services::iam::IamService::new(engine.clone()),
            dns: crate::services::dns::DnsService::new(engine.clone()),
            monitoring: crate::services::monitoring::MonitoringService::new(engine.clone()),
            workflows: crate::services::workflows::WorkflowsService::new(engine.clone()),
            networking: crate::services::networking::NetworkingService::new(engine.clone()),
            run: crate::services::run::CloudRunService::new(engine.clone()),
            kms: crate::services::kms::KmsService::new(engine.clone()),
        }
    }
}

impl Default for GcpProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl CloudProviderTrait for GcpProvider {
    async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // Route based on path patterns
        let path = &req.path;
        
        // Firestore: /projects/.../databases/.../documents/...
        if path.contains("/databases/") && path.contains("/documents") {
            return self.firestore.handle_request(req).await;
        }
        
        // Pub/Sub: /v1/projects/.../topics/... or .../subscriptions/...
        if path.contains("/topics/") || path.contains("/subscriptions/") {
            return self.pubsub.handle_request(req).await;
        }
        
        // Cloud Functions: /v1/projects/.../locations/.../functions/...
        if path.contains("/functions/") {
            return self.functions.handle_request(req).await;
        }
        
        // Secret Manager: /v1/projects/.../secrets/...
        // Secret Manager: /v1/projects/.../secrets/...
        if path.contains("/secrets/") {
            return self.secret_manager.handle_request(req).await;
        }

        // Cloud Billing: /v1/services
        if path.contains("/services") && !path.contains("/locations/") {
             return self.billing.handle_request(req).await;
        }
        
        // Compute Engine: /compute/v1/projects/...
        if path.contains("/compute/v1/") {
            return self.compute.handle_request(req).await;
        }

        // Cloud SQL: /sql/v1beta4/projects/...
        if path.contains("/sql/") {
            return self.sql.handle_request(req).await;
        }

        if path.contains("/serviceAccounts") {
             return self.iam.handle_request(req).await;
        }

        if path.contains("/dns/v1") || path.contains("/managedZones") {
             return self.dns.handle_request(req).await;
        }

        if path.contains("/timeSeries") {
             return self.monitoring.handle_request(req).await;
        }

        if path.contains("/workflows") {
             return self.workflows.handle_request(req).await;
        }

        if path.contains("/global/networks") || path.contains("/subnetworks") {
             return self.networking.handle_request(req).await;
        }

        if path.contains("/locations/") && path.contains("/services") {
             return self.run.handle_request(req).await;
        }

        if path.contains("/keyRings") {
             return self.kms.handle_request(req).await;
        }
        
        // Default: Cloud Storage (bucket/object operations)
        self.storage.handle_request(req).await
    }

    fn supported_services(&self) -> Vec<ServiceType> {
        vec![
            ServiceType::ObjectStorage,    // Cloud Storage
            ServiceType::KeyValue,          // Firestore
            ServiceType::MessageQueue,      // Pub/Sub
            ServiceType::PubSub,            // Pub/Sub
            ServiceType::Functions,         // Cloud Functions
            ServiceType::Secrets,           // Secret Manager
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
        assert!(provider.supported_services().len() >= 5);
    }

    #[tokio::test]
    async fn test_storage_routing() {
        let provider = GcpProvider::in_memory();
        
        let req = Request {
            method: "PUT".to_string(),
            path: "/test-bucket".to_string(),
            headers: std::collections::HashMap::new(),
            body: vec![],
        };

        let response = provider.handle_request(req).await.unwrap();
        assert_eq!(response.status, 201);
    }
}
