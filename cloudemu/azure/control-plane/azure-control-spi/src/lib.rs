//! Azure Control-Plane SPI
//!
//! Self-contained foundation types for Azure control emulation.

#![warn(missing_docs)]

mod error;
mod types;
mod traits;

pub use error::*;
pub use types::*;
pub use traits::*;
