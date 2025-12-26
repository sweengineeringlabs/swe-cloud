//! # CloudKit Azure Provider
//!
//! Azure implementation of CloudKit service traits.
//!
//! ## Supported Services
//!
//! - **Blob Storage** - Object storage (feature: `blob`)
//! - **Cosmos DB** - Key-value store (feature: `cosmos`)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use cloudkit::prelude::*;
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

pub use builder::*;

/// Azure Blob Storage implementation.
pub struct AzureBlobStorage;

/// Azure Cosmos DB implementation.
pub struct AzureCosmosDb;
