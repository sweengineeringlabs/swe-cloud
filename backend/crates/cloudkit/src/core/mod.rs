//! # Core Layer
//!
//! Default implementations and internal utilities.
//!
//! This layer provides base implementations that can be extended
//! by provider-specific crates.

mod client;
mod executor;

pub use client::*;
pub use executor::*;
