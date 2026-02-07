//! AWS Data-Plane API
//!
//! Storage-layer trait definitions for AWS data operations.

#![warn(missing_docs)]

pub use aws_data_spi;

/// Storage operations trait.
pub mod storage;

/// Prelude — re-exports all data-plane traits.
pub mod prelude {
    pub use super::storage::*;
}
