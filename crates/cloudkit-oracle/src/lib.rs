//! # CloudKit Oracle Cloud Provider
//!
//! Oracle Cloud Infrastructure (OCI) implementation of CloudKit service traits.
//!
//! ## Supported Services
//!
//! - **Object Storage** - Object storage (feature: `object-storage`)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use cloudkit::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> CloudResult<()> {
//!     let oracle = CloudKit::oracle()
//!         .region(Region::oracle_af_johannesburg_1())
//!         .build()
//!         .await?;
//!
//!     oracle.storage().put_object("bucket", "key", b"data").await?;
//!     
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]

mod builder;

pub use builder::*;

/// Oracle Object Storage implementation.
pub struct OciObjectStorage;
