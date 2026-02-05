//! Provider adapters for cloudemu-core integration.

mod aws;

pub use aws::{AwsProvider, AwsStorageAdapter};
pub mod aws_query;
