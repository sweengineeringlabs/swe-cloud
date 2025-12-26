//! AWS SNS pub/sub implementation.

use cloudkit::core::CloudContext;
use std::sync::Arc;

/// AWS SNS pub/sub implementation.
pub struct SnsPubSub {
    context: Arc<CloudContext>,
}

impl SnsPubSub {
    /// Create a new SNS pub/sub client.
    pub fn new(context: Arc<CloudContext>) -> Self {
        Self { context }
    }
}

// TODO: Implement PubSub trait
