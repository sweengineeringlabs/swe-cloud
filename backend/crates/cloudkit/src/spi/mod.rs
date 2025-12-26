//! # SPI Layer (Service Provider Interface)
//!
//! Extension points for customizing CloudKit behavior.
//!
//! This layer defines traits that can be implemented to extend or customize
//! the SDK's behavior without modifying the core implementation.
//!
//! ## Extension Points
//!
//! - **AuthProvider** - Custom authentication mechanisms
//! - **RetryPolicy** - Custom retry strategies
//! - **MetricsCollector** - Observability integration
//! - **Logger** - Custom logging

mod auth;
mod logger;
mod metrics;
mod retry;

pub use auth::*;
pub use logger::*;
pub use metrics::*;
pub use retry::*;
