//! GCP Data-Plane SPI
//!
//! Self-contained foundation types for GCP data emulation.

#![warn(missing_docs)]

mod error;
mod types;
mod traits;

pub use error::*;
pub use types::*;
pub use traits::*;
