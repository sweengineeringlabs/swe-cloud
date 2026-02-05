//! # CloudEmu Azure Provider
//!
//! Azure cloud provider implementation for CloudEmu.
//!
//! This crate provides Azure-specific implementations of CloudEmu's provider-agnostic
//! traits, enabling local emulation of Azure services.
//!
//! ## Supported Services
//!
//! - **Blob Storage** (equivalent to AWS S3)
//! - **Cosmos DB** (equivalent to AWS DynamoDB)
//! - **Service Bus** (equivalent to AWS SQS/SNS)
//! - **Azure Functions** (equivalent to AWS Lambda)
//! - **Key Vault** (equivalent to AWS Secrets Manager)

mod provider;
pub mod services;

pub use provider::AzureProvider;
pub use azure_data_core::storage::StorageEngine as AzureStorageEngine;
