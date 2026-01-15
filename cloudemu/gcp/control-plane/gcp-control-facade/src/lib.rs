//! GCP Control-Plane Facade

#![warn(missing_docs)]

pub use gcp_control_spi;
pub use gcp_control_api;
pub use gcp_control_core;

// Re-export Provider and Gateway
pub use gcp_control_core::GcpProvider;

// Re-export SPI types for convenience if needed by consumers
pub use gcp_control_spi as spi;
