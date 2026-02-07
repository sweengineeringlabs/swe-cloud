//! # CloudKit Azure Provider
//!
//! Azure implementation of CloudKit service traits.
//!
//! ## Supported Services
//!
//! - **Blob Storage** - Object storage (feature: `blob`)
//! - **Cosmos DB** - Key-value store (feature: `cosmos`)
//! - **Key Vault** - Secrets management (feature: `keyvault`)
//! - **Azure Monitor** - Metrics and logging (feature: `monitor`)
//! - **Event Grid** - Event routing (feature: `eventgrid`)
//! - **Azure AD** - Identity provider (feature: `identity`)
//! - **Service Bus** - Message queue (feature: `servicebus`)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use cloudkit_spi::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> CloudResult<()> {
//!     let azure = CloudKit::azure()
//!         .region(Region::azure_east_us())
//!         .build()
//!         .await?;
//!
//!     azure.storage().put_object("container", "key", b"data").await?;
//!     
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]

mod builder;

#[cfg(feature = "blob")]
mod blob;

#[cfg(feature = "cosmos")]
mod cosmos;

#[cfg(feature = "keyvault")]
mod keyvault;

#[cfg(feature = "monitor")]
mod monitor;

#[cfg(feature = "eventgrid")]
mod eventgrid;

#[cfg(feature = "identity")]
mod identity;

#[cfg(feature = "servicebus")]
mod servicebus;

pub use builder::*;

#[cfg(feature = "blob")]
pub use blob::*;

#[cfg(feature = "cosmos")]
pub use cosmos::*;

#[cfg(feature = "keyvault")]
pub use keyvault::*;

#[cfg(feature = "monitor")]
pub use monitor::*;

#[cfg(feature = "eventgrid")]
pub use eventgrid::*;

#[cfg(feature = "identity")]
pub use identity::*;

#[cfg(feature = "servicebus")]
pub use servicebus::*;

