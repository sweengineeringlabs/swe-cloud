//! Provider adapters for cloudemu-core integration.

mod aws;

pub use aws::{AwsProvider, AwsStorageAdapter};
