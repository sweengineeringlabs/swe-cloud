//! # CloudEmu Core
//!
//! Provider-agnostic abstractions for multi-cloud emulation.
//!
//! This crate defines the foundational traits and types that enable CloudEmu
//! to support multiple cloud providers (AWS, Azure, GCP) through a unified interface.
//!
//! ## Architecture
//!
//! CloudEmu Core provides:
//! - **CloudProvider Trait**: Unified interface for handling cloud-specific requests
//! - **StorageEngine Trait**: Provider-agnostic persistence abstraction
//! - **Resource Types**: Universal cloud resource representation
//! - **Error Types**: Unified error handling across all providers

#![warn(missing_docs)]
#![deny(unsafe_code)]

mod error;
mod provider;
mod storage;
mod types;

pub use error::*;
pub use provider::*;
pub use storage::*;
pub use types::*;
