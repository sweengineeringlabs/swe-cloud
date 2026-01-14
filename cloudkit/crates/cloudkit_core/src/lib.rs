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
//! This crate is typically not used directly by end users. Instead, use the `cloudkit` facade
//! crate for a simplified experience.
//!
//! ### Advanced Usage (Direct Builder)
//!
//! For advanced use cases (e.g., custom emulation, specific provider features), you can use
//! the builders directly:
//!
//! ```rust,ignore
//! use cloudkit_spi::{CloudConfig, Region};
//! use cloudkit_core::aws::AwsBuilder; // Requires cloudkit-aws crate
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to a local CloudEmu instance
//!     let config = CloudConfig::builder()
//!         .region(Region::aws_us_east_1())
//!         .endpoint("http://localhost:4566")
//!         .build()?;
//!
//!     // Use the provider-specific builder
//!     let client = AwsBuilder::new()
//!         .config(config)
//!         .build()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]

// Re-export dependencies
pub use cloudkit_spi;
pub use cloudkit_api;

// Core modules
mod executor;

// Re-export core types
pub use executor::*;
