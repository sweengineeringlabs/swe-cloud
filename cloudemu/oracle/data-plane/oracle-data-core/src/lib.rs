//! Oracle Cloud Data Plane Core
//!
//! Handles persistence for OCI emulation using SQLite.

pub mod storage;
pub mod error;

pub use storage::StorageEngine;
pub use error::Error;
