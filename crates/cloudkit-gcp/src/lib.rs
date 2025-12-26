//! # CloudKit GCP Provider
//!
//! Google Cloud Platform implementation of CloudKit service traits.
//!
//! ## Supported Services
//!
//! - **Cloud Storage** - Object storage (feature: `gcs`)
//! - **Pub/Sub** - Messaging (feature: `pubsub`)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use cloudkit::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> CloudResult<()> {
//!     let gcp = CloudKit::gcp()
//!         .project("my-project")
//!         .build()
//!         .await?;
//!
//!     gcp.storage().put_object("bucket", "key", b"data").await?;
//!     
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]

mod builder;

pub use builder::*;

/// Google Cloud Storage implementation.
pub struct GcsStorage;

/// Google Cloud Pub/Sub implementation.
pub struct GcpPubSub;
