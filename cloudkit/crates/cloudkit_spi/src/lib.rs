//! # CloudKit SPI (Service Provider Interface)
//!
//! Low-level provider contracts and foundational types for CloudKit.
//!
//! This crate provides:
//! - **Error types**: Unified error handling across all providers
//! - **Common types**: Shared data structures (Region, Metadata, etc.)
//! - **Configuration**: Cloud provider configuration
//! - **Extension points**: Traits for retry policies, metrics, auth, and logging
//!
//! ## Architecture
//!
//! This is the foundation layer that all other CloudKit crates depend on.
//! It defines the contracts that provider implementations must fulfill.

#![warn(missing_docs)]
#![deny(unsafe_code)]

// Core types and errors
mod error;
mod types;
mod region;
mod config;
mod context;

// Extension points (SPI traits)
mod auth;
mod retry;
mod metrics;
mod logger;

// Re-export everything
pub use error::*;
pub use types::*;
pub use region::*;
pub use config::*;
pub use context::*;

pub use auth::*;
pub use retry::*;
pub use metrics::*;
pub use logger::*;
