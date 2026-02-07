//! AWS Control-Plane API
//!
//! Service trait definitions for AWS services.

#![warn(missing_docs)]

pub use aws_control_spi;

/// S3 service traits
pub mod s3;

/// DynamoDB service traits
pub mod dynamodb;

/// SQS service traits
pub mod sqs;

/// SNS service traits
pub mod sns;

/// Lambda service traits
pub mod lambda;

/// Prelude — re-exports all service traits.
pub mod prelude {
    pub use super::s3::*;
    pub use super::dynamodb::*;
    pub use super::sqs::*;
    pub use super::sns::*;
    pub use super::lambda::*;
}
