//! # CloudKit GCP Provider
//!
//! Google Cloud Platform implementation of CloudKit service traits.
//!
//! ## Supported Services
//!
//! - **Cloud Storage** - Object storage (feature: `gcs`)
//! - **Pub/Sub** - Messaging (feature: `pubsub`)
//! - **Firestore** - Key-value store (feature: `firestore`)
//! - **Secret Manager** - Secrets management (feature: `secrets`)
//! - **Monitoring** - Cloud Monitoring & Logging (feature: `monitor`)
//! - **Eventarc** - Event bus (feature: `eventarc`)
//! - **Identity Platform** - Identity & Access (feature: `identity`)
//! - **Cloud KMS** - Key Management Service (feature: `kms`)
//! - **Workflows** - Workflow orchestration (feature: `workflows`)
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

#[cfg(feature = "gcs")]
mod gcs;
#[cfg(feature = "pubsub")]
mod pubsub;
#[cfg(feature = "firestore")]
mod firestore;
#[cfg(feature = "secrets")]
mod secrets;
#[cfg(feature = "monitor")]
mod monitor;
#[cfg(feature = "eventarc")]
mod eventarc;
#[cfg(feature = "identity")]
mod identity;
#[cfg(feature = "kms")]
mod kms;
#[cfg(feature = "workflows")]
mod workflows;

pub use builder::*;

#[cfg(feature = "gcs")]
pub use gcs::*;
#[cfg(feature = "pubsub")]
pub use pubsub::*;
#[cfg(feature = "firestore")]
pub use firestore::*;
#[cfg(feature = "secrets")]
pub use secrets::*;
#[cfg(feature = "monitor")]
pub use monitor::*;
#[cfg(feature = "eventarc")]
pub use eventarc::*;
#[cfg(feature = "identity")]
pub use identity::*;
#[cfg(feature = "kms")]
pub use kms::*;
#[cfg(feature = "workflows")]
pub use workflows::*;
