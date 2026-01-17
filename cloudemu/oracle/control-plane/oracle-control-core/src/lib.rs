use std::sync::Arc;
use oracle_data_core::StorageEngine;
use oracle_control_spi::{CloudProviderTrait, Request, Response, CloudResult};
use async_trait::async_trait;

pub mod services;
use services::pricing::PricingService;

pub struct OracleProvider {
    storage: Arc<StorageEngine>,
    pricing: PricingService,
    compute: services::compute::ComputeService,
    database: services::database::DatabaseService,
    identity: services::identity::IdentityService,
    dns: services::dns::DnsService,
    object_storage: services::object_storage::ObjectStorageService,
    monitoring: services::monitoring::MonitoringService,
    functions: services::functions::FunctionsService,
    queue: services::queue::QueueService,
    networking: services::networking::NetworkingService,
    containers: services::containers::ContainerService,
    vault: services::vault::VaultService,
    nosql: services::nosql::NoSqlService,
}

impl OracleProvider {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { 
            storage: storage.clone(),
            pricing: PricingService::new(storage.clone()),
            compute: services::compute::ComputeService::new(storage.clone()),
            database: services::database::DatabaseService::new(storage.clone()),
            identity: services::identity::IdentityService::new(storage.clone()),
            dns: services::dns::DnsService::new(storage.clone()),
            object_storage: services::object_storage::ObjectStorageService::new(storage.clone()),
            monitoring: services::monitoring::MonitoringService::new(storage.clone()),
            functions: services::functions::FunctionsService::new(storage.clone()),
            queue: services::queue::QueueService::new(storage.clone()),
            networking: services::networking::NetworkingService::new(storage.clone()),
            containers: services::containers::ContainerService::new(storage.clone()),
            vault: services::vault::VaultService::new(storage.clone()),
            nosql: services::nosql::NoSqlService::new(storage),
        }
    }
}

#[async_trait]
impl CloudProviderTrait for OracleProvider {
    async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // Pricing/Billing API
        if req.path.contains("/metering/api/v1/prices") {
            return self.pricing.handle_request(req).await;
        }

        // Compute
        if req.path.contains("/instances") {
            return self.compute.handle_request(req).await;
        }

        // Database
        if req.path.contains("/autonomousDatabases") {
            return self.database.handle_request(req).await;
        }

        if req.path.contains("/users") {
             return self.identity.handle_request(req).await;
        }

        if req.path.contains("/zones") {
             return self.dns.handle_request(req).await;
        }

        if req.path.contains("/n/") && req.path.contains("/b/") {
             return self.object_storage.handle_request(req).await;
        }

        if req.path.contains("/postMetricData") {
             return self.monitoring.handle_request(req).await;
        }

        if req.path.contains("/functions") {
             return self.functions.handle_request(req).await;
        }

        if req.path.contains("/queues") {
             return self.queue.handle_request(req).await;
        }

        if req.path.contains("/vcns") || req.path.contains("/subnets") {
             return self.networking.handle_request(req).await;
        }

        if req.path.contains("/containerInstances") {
             return self.containers.handle_request(req).await;
        }

        if req.path.contains("/secrets") || req.path.contains("/vaults") {
             return self.vault.handle_request(req).await;
        }

        if req.path.contains("/tables") || req.path.contains("/rows") {
             return self.nosql.handle_request(req).await;
        }

        // Basic routing logic
        if req.path.starts_with("/v1") {
             return Ok(Response::ok(b"Oracle API Root".to_vec()));
        }
        Ok(Response::not_found("Not Found"))
    }
}
