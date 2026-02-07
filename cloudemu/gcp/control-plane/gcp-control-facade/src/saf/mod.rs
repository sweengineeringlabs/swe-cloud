//! SAF — Service Access Facade re-exports.
//!
//! Provides a unified public API surface for the GCP control-plane.

pub use gcp_control_spi::*;
pub use gcp_control_api::prelude::*;
pub use gcp_control_core::GcpProvider;
