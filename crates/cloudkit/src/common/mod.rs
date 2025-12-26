//! # Common Layer
//!
//! Shared types, errors, and utilities used across all layers.
//!
//! This is the foundation layer that all other layers depend on.

mod config;
mod error;
mod region;
mod types;

pub use config::*;
pub use error::*;
pub use region::*;
pub use types::*;
