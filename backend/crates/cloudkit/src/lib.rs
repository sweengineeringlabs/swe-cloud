//! # CloudKit - Multi-Cloud SDK
//!
//! A unified, type-safe Rust SDK for interacting with multiple cloud providers
//! through a single, consistent API.
//!
//! ## Architecture (SEA - Stratified Encapsulation Architecture)
//!
//! This crate is organized into five layers:
//!
//! 1. **Common** - Shared types, errors, and utilities
//! 2. **SPI** - Service Provider Interface for extensions
//! 3. **API** - Service contracts and traits
//! 4. **Core** - Default implementations
//! 5. **Facade** - Public API surface
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
// LAYER 1: COMMON - Shared types, errors, and utilities
// =============================================================================
pub mod common;

// =============================================================================
// LAYER 2: SPI - Service Provider Interface for extensions
// =============================================================================
pub mod spi;

// =============================================================================
// LAYER 3: API - Service contracts and traits
// =============================================================================
pub mod api;

// =============================================================================
// LAYER 4: CORE - Default implementations
// =============================================================================
pub mod core;

// =============================================================================
// LAYER 5: FACADE - Public API surface
// =============================================================================
pub mod facade;

// =============================================================================
// PRELUDE - Convenient re-exports
// =============================================================================
pub mod prelude;

// Re-export facade as the primary API
pub use facade::*;
