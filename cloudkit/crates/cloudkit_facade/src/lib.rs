//! # CloudKit - Multi-Cloud SDK
//!
//! A unified, type-safe Rust SDK for interacting with multiple cloud providers
//! through a single, consistent API.
//!
//! ## Architecture (SEA - Stratified Encapsulation Architecture)
//!
//! CloudKit is organized into multiple foundational crates:
//!
//! 1. **cloudkit_spi** - Service Provider Interface (foundation)
//! 2. **cloudkit_api** - High-level service API traits
//! 3. **cloudkit_core** - Core orchestration layer
//! 4. **cloudkit** - Facade (this crate)
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use cloudkit::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), CloudError> {
//!     let cloud = CloudKit::aws()
//!         .region(Region::UsEast1)
//!         .build()
//!         .await?;
//!
//!     cloud.storage()
//!         .put_object("bucket", "key", b"data")
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Feature Flags
//!
//! - `aws` - Enable AWS provider
//! - `azure` - Enable Azure provider
//! - `gcp` - Enable GCP provider
//! - `oracle` - Enable Oracle Cloud provider
//! - `full` - Enable all providers

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![deny(unsafe_code)]

// =============================================================================
// RE-EXPORT FOUNDATION CRATES
// =============================================================================

/// SPI (Service Provider Interface) - Foundation types and extension points
pub use cloudkit_spi;

/// API - High-level service traits
pub use cloudkit_api;

/// Core - Orchestration layer
pub use cloudkit_core;

// =============================================================================
// FACADE - Public API surface
// =============================================================================
pub mod facade;

// =============================================================================
// PRELUDE - Convenient re-exports
// =============================================================================
pub mod prelude;

// Re-export facade as the primary API
pub use facade::*;
