//! # CloudKit AWS Provider
//!
//! AWS implementation of CloudKit service traits.
//!
//! ## Supported Services
//!
//! - **S3** - Object storage (feature: `s3`)
//! - **DynamoDB** - Key-value store (feature: `dynamodb`)
//! - **SQS** - Message queue (feature: `sqs`)
//! - **SNS** - Pub/Sub messaging (feature: `sns`)
//! - **Lambda** - Serverless functions (feature: `lambda`)
//! - **Secrets Manager** - Secret management (feature: `secrets`)
//! - **CloudWatch** - Metrics and logging (feature: `cloudwatch`)
//! - **EventBridge** - Event bus (feature: `eventbridge`)
//! - **Step Functions** - Workflow orchestration (feature: `stepfunctions`)
//! - **Cognito** - Identity provider (feature: `cognito`)
//! - **KMS** - Key management (feature: `kms`)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use cloudkit_spi::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> CloudResult<()> {
//!     let aws = CloudKit::aws()
//!         .region(Region::aws_us_east_1())
//!         .build()
//!         .await?;
//!
//!     aws.storage().put_object("bucket", "key", b"data").await?;
//!     
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]

mod builder;

#[cfg(feature = "s3")]
mod s3;

#[cfg(feature = "dynamodb")]
mod dynamodb;

#[cfg(feature = "sqs")]
mod sqs;

#[cfg(feature = "sns")]
mod sns;

#[cfg(feature = "lambda")]
mod lambda;

#[cfg(feature = "secrets")]
mod secrets;

#[cfg(feature = "cloudwatch")]
mod cloudwatch;

#[cfg(feature = "eventbridge")]
mod eventbridge;

#[cfg(feature = "stepfunctions")]
mod stepfunctions;

#[cfg(feature = "cognito")]
mod cognito;

#[cfg(feature = "kms")]
mod kms;

#[cfg(feature = "ec2")]
mod ec2;

#[cfg(feature = "vpc")]
mod vpc;

pub use builder::*;

#[cfg(feature = "s3")]
pub use s3::*;

#[cfg(feature = "dynamodb")]
pub use dynamodb::*;

#[cfg(feature = "sqs")]
pub use sqs::*;

#[cfg(feature = "sns")]
pub use sns::*;

#[cfg(feature = "lambda")]
pub use lambda::*;

#[cfg(feature = "secrets")]
pub use secrets::*;

#[cfg(feature = "cloudwatch")]
pub use cloudwatch::*;

#[cfg(feature = "eventbridge")]
pub use eventbridge::*;

#[cfg(feature = "stepfunctions")]
pub use stepfunctions::*;

#[cfg(feature = "cognito")]
pub use cognito::*;

#[cfg(feature = "kms")]
pub use kms::*;

#[cfg(feature = "ec2")]
pub use ec2::*;

#[cfg(feature = "vpc")]
pub use vpc::*;

