//! AWS Data-Plane SPI
//!
//! Self-contained foundation types for AWS data emulation.

#![warn(missing_docs)]

mod error;
mod types;
mod traits;

pub use error::*;
pub use types::*;
pub use traits::*;
