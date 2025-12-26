//! # API Layer
//!
//! Service contracts (traits) that define the public interface for cloud operations.
//!
//! This layer provides the abstraction over different cloud providers,
//! enabling provider-agnostic code.
//!
//! ## Service Traits
//!
//! - **ObjectStorage** - Blob/object storage operations (S3, Blob, GCS)
//! - **KeyValueStore** - NoSQL key-value operations (DynamoDB, Cosmos, Firestore)
//! - **MessageQueue** - Queue operations (SQS, Service Bus, Pub/Sub)
//! - **PubSub** - Publish/subscribe messaging (SNS, Event Grid, Pub/Sub)
//! - **Functions** - Serverless function invocation (Lambda, Functions, Cloud Functions)

mod functions;
mod kv_store;
mod message_queue;
mod object_storage;
mod pubsub;

pub use functions::*;
pub use kv_store::*;
pub use message_queue::*;
pub use object_storage::*;
pub use pubsub::*;
