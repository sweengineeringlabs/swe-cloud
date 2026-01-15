//! Azure Control-Plane Facade

#![warn(missing_docs)]

pub use azure_control_spi;
pub use azure_control_api;
pub use azure_control_core;

// Re-export Provider and Gateway
pub use azure_control_core::AzureProvider;

// Re-export SPI types for convenience if needed by consumers
pub use azure_control_spi as spi;
