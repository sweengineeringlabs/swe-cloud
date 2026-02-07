//! # CloudKit API
//!
//! High-level service API traits for CloudKit.
//!
//! This crate provides the user-facing abstractions for cloud services.
//!
//! ## Architecture
//!
//! This layer extends the low-level SPI contracts with higher-level operations
//! that are more convenient for application developers.
//!
//! ## Service Traits
//!
//! - **ObjectStorage** - Blob/object storage operations (S3, Blob, GCS)
//! - **KeyValueStore** - NoSQL key-value operations (DynamoDB, Cosmos, Firestore)
//! - **MessageQueue** - Queue operations (SQS, Service Bus, Pub/Sub)
//! - **PubSub** - Publish/subscribe messaging (SNS, Event Grid, Pub/Sub)
//! - **Functions** - Serverless function invocation (Lambda, Functions, Cloud Functions)
//! - **SecretsManager** - Secret management (Secrets Manager, Key Vault, Secret Manager)
//! - **MetricsService** - Metrics collection (CloudWatch, Azure Monitor, Cloud Monitoring)
//! - **LoggingService** - Log management (CloudWatch Logs, Log Analytics, Cloud Logging)
//! - **EventBus** - Event routing (EventBridge, Event Grid, Eventarc)
//! - **WorkflowService** - Workflow orchestration (Step Functions, Logic Apps, Workflows)
//! - **IdentityProvider** - Authentication (Cognito, Azure AD B2C, Identity Platform)
//! - **KeyManagement** - Encryption keys (KMS, Key Vault, Cloud KMS)

#![warn(missing_docs)]
#![deny(unsafe_code)]

// Re-export cloudkit_spi for convenience
pub use cloudkit_spi;

// API modules
mod encryption;
mod events;
mod functions;
mod identity;
mod kv_store;
mod message_queue;
mod monitoring;
mod object_storage;
mod pubsub;
mod secrets;
mod workflow;
mod compute;
mod networking;

// Re-export all API types
pub use encryption::*;
pub use events::*;
pub use functions::*;
pub use identity::*;
pub use kv_store::*;
pub use message_queue::*;
pub use monitoring::*;
pub use object_storage::*;
pub use pubsub::*;
pub use secrets::*;
pub use workflow::*;
pub use compute::*;
pub use networking::*;
