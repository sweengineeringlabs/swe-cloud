//! GCP Control-Plane API
//!
//! Service trait definitions for GCP services.

#![warn(missing_docs)]

pub use gcp_control_spi;

/// Cloud Storage service traits
pub mod cloud_storage;

/// Firestore service traits
pub mod firestore;

/// Pub/Sub service traits
pub mod pubsub;

/// Cloud Functions service traits
pub mod functions;

/// Secret Manager service traits
pub mod secret_manager;

/// Prelude — re-exports all service traits.
pub mod prelude {
    pub use super::cloud_storage::*;
    pub use super::firestore::*;
    pub use super::pubsub::*;
    pub use super::functions::*;
    pub use super::secret_manager::*;
}
