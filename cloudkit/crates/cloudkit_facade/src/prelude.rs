//! # Prelude
//!
//! Convenient re-exports for common usage.
//!
//! # Example
//!
//! ```rust,ignore
//! use cloudkit::prelude::*;
//! ```

// Re-export everything from cloudkit_spi
pub use cloudkit_spi::*;

// Re-export everything from cloudkit_api
pub use cloudkit_api::*;

// Resolve ambiguity for LogLevel (prefer API)
pub use cloudkit_api::LogLevel;

// Re-export everything from cloudkit_core
pub use cloudkit_core::*;

// Re-export facade
pub use crate::facade::*;
