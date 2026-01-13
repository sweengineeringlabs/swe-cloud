//! # CloudKit Core
//!
//! Core orchestration layer for CloudKit.
//!
//! This crate implements the orchestration logic that connects the high-level
//! API traits with the provider-specific implementations.
//!
//! ## Architecture
//!
//! CloudKit Core provides:
//! - **CloudContext**: Central configuration and service aggregation
//! - **ProviderType**: Enum for cloud providers (AWS, Azure, GCP, Oracle)
//! - **OperationExecutor**: Retry and metrics handling
//!
//! ## Usage
//!
//! This crate is typically not used directly. Instead, use the `cloudkit` facade
//! crate which re-exports everything from here.

#![warn(missing_docs)]
#![deny(unsafe_code)]

// Re-export dependencies
pub use cloudkit_spi;
pub use cloudkit_api;

// Core modules
mod executor;

// Re-export core types
pub use executor::*;
