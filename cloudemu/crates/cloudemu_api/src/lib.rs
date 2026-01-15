//! CloudEmu API Layer
//!
//! Defines the service traits that emulator providers must implement.
//! This layer is analogous to `cloudkit_api` in the CloudKit SDK.

#![warn(missing_docs)]
#![deny(unsafe_code)]

// Re-export foundation
pub use cloudemu_spi;

// Service trait modules (to be populated as needed)
pub mod storage;
pub mod database;
pub mod messaging;

/// Prelude for common API types
pub mod prelude {
    pub use crate::storage::*;
    pub use crate::database::*;
    pub use crate::messaging::*;
}
