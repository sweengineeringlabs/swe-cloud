//! AWS Data-Plane Facade
//!
//! Public API for AWS data operations.

#![warn(missing_docs)]

/// SAF — Service Access Facade re-exports.
pub mod saf;

pub use aws_data_spi;
pub use aws_data_api;
pub use aws_data_core;

