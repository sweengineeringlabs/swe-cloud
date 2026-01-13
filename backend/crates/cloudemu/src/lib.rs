//! # CloudEmu - Production-Grade Local Cloud Emulator
//!
//! CloudEmu emulates AWS cloud services locally with production-level accuracy.
//! It works with Terraform, AWS SDKs, and the AWS CLI out of the box.
//!
//! ## Supported Services
//!
//! - **S3**: Object storage with versioning, policies, lifecycle
//! - **DynamoDB**: NoSQL database
//! - **SQS/SNS**: Messaging and Pub/Sub
//! - **Lambda**: Serverless functions
//! - **KMS/Secrets**: Security and encryption
//! - **And more**: EventBridge, CloudWatch, Cognito, etc.
//!
//! ## Quick Start
//!
//! ```bash
//! # Start the emulator
//! cargo run -p cloudemu
//!
//! # Use with AWS CLI
//! aws --endpoint-url=http://localhost:4566 s3 mb s3://my-bucket
//! ```

pub use control_plane::gateway;
pub use control_plane::services;
pub use control_plane::{Emulator, start_server, ApiError, Result};
pub use data_plane::{Config, StorageEngine, EmulatorError};

// Re-export storage module too?
pub mod storage {
    pub use data_plane::storage::*;
}

pub mod error {
    pub use control_plane::error::*; 
    pub use data_plane::error::EmulatorError;
}
