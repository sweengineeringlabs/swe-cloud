//! Azure Control-Plane API
//!
//! Service trait definitions for Azure services.

#![warn(missing_docs)]

pub use azure_control_spi;

/// Blob Storage service traits
pub mod blob;

/// Cosmos DB service traits
pub mod cosmos;

/// Service Bus service traits
pub mod servicebus;

/// Azure Functions service traits
pub mod functions;

/// Key Vault service traits
pub mod keyvault;

/// Event Grid service traits
pub mod eventgrid;

/// Prelude — re-exports all service traits.
pub mod prelude {
    pub use super::blob::*;
    pub use super::cosmos::*;
    pub use super::servicebus::*;
    pub use super::functions::*;
    pub use super::keyvault::*;
    pub use super::eventgrid::*;
}
