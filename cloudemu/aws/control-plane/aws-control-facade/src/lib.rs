//! AWS Control-Plane Facade
//!
//! HTTP routing and public API for AWS emulation.

#![warn(missing_docs)]

pub use aws_control_spi;
pub use aws_control_api;
pub use aws_control_core;

// Re-export gateway/routing from core
pub use aws_control_core::gateway;
