//! AWS Provider for CloudEmu
//!
//! This crate integrates the control-plane (API routing) and data-plane (storage)
//! for AWS service emulation.

#![warn(missing_docs)]
#![deny(unsafe_code)]

// Re-export control-plane
pub use aws_control_plane as control;

// Re-export data-plane  
pub use aws_data_plane as data;

// Convenience re-exports
pub use aws_control_plane::*;
