//! Oracle Control-Plane SPI
//!
//! Foundation types for Oracle cloud emulation.

#![warn(missing_docs)]

mod error;
mod types;
mod traits;

pub use error::*;
pub use types::*;
pub use traits::*;
