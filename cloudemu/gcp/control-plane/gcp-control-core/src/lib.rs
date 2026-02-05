//! # CloudEmu GCP Provider
//!
//! Google Cloud Platform provider implementation for CloudEmu.
//!
//! This crate provides GCP-specific implementations of CloudEmu's provider-agnostic
//! traits, enabling local emulation of Google Cloud services.
//!
//! ## Supported Services
//!
//! - **Cloud Storage** (equivalent to AWS S3)
//! - **Firestore** (equivalent to AWS DynamoDB)
//! - **Pub/Sub** (equivalent to AWS SQS/SNS)
//! - **Cloud Functions** (equivalent to AWS Lambda)
//! - **Secret Manager** (equivalent to AWS Secrets Manager)

mod provider;
pub mod services;

pub use provider::GcpProvider;
pub use gcp_data_core::storage::StorageEngine as GcpStorageEngine;
