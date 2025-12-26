//! # Prelude
//!
//! Convenient re-exports for common usage.
//!
//! # Example
//!
//! ```rust,ignore
//! use cloudkit::prelude::*;
//! ```

// Common types
pub use crate::common::{
    BucketMetadata,
    CloudConfig,
    CloudError,
    CloudResult,
    Credentials,
    ListResult,
    ObjectMetadata,
    PaginationToken,
    Region,
    ResourceId,
    ResourceMetadata,
};

// API traits
pub use crate::api::{
    Functions,
    GetOptions,
    InvocationType,
    InvokeOptions,
    InvokeResult,
    KeyValueStore,
    KvGetOptions,
    KvPutOptions,
    KvQueryOptions,
    ListOptions,
    Message,
    MessageQueue,
    ObjectStorage,
    PubSub,
    PubSubMessage,
    PutOptions,
    ReceiveOptions,
    SendOptions,
    SubscriptionConfig,
};

// SPI traits
pub use crate::spi::{
    AuthProvider,
    ExponentialBackoff,
    FixedDelay,
    Logger,
    LogLevel,
    MetricsCollector,
    NoRetry,
    OperationMetrics,
    OperationOutcome,
    RetryDecision,
    RetryPolicy,
};

// Core types
pub use crate::core::{
    CloudContext,
    ProviderType,
};

// Facade
pub use crate::facade::CloudKit;
