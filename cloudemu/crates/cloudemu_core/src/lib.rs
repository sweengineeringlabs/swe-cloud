//! CloudEmu Core Layer
//!
//! Core orchestration layer for CloudEmu.
//!
//! This crate implements the orchestration logic that connects the high-level
//! API traits with the provider-specific implementations.
//!
//! ## Architecture
//!
//! CloudEmu Core provides:
//! - **EmulatorContext**: Central configuration and service aggregation
//! - **ProviderType**: Enum for cloud providers (AWS, Azure, GCP)
//! - **RequestRouter**: Routes requests to appropriate provider implementations
//!
//! ## Usage
//!
//! This crate is typically not used directly. Instead, use the `cloudemu_server` facade
//! which orchestrates the emulator runtime.

#![warn(missing_docs)]
#![deny(unsafe_code)]

// Re-export dependencies
pub use cloudemu_spi;
pub use cloudemu_api;

// Re-export provider modules when features are enabled
#[cfg(feature = "aws")]
pub use cloudemu_aws;

#[cfg(feature = "azure")]
pub use cloudemu_azure;

#[cfg(feature = "gcp")]
pub use cloudemu_gcp;
