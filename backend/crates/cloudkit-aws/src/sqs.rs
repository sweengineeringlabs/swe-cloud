//! AWS SQS message queue implementation.

use cloudkit::core::CloudContext;
use std::sync::Arc;

/// AWS SQS message queue implementation.
pub struct SqsQueue {
    context: Arc<CloudContext>,
}

impl SqsQueue {
    /// Create a new SQS queue client.
    pub fn new(context: Arc<CloudContext>) -> Self {
        Self { context }
    }
}

// TODO: Implement MessageQueue trait
