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
//!
//! ## Usage
//!
//! ```rust,ignore
//! use cloudkit::prelude::*;
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
