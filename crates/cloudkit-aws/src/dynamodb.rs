//! AWS DynamoDB key-value store implementation.

use cloudkit::core::CloudContext;
use std::sync::Arc;

/// AWS DynamoDB key-value store implementation.
pub struct DynamoDbStore {
    context: Arc<CloudContext>,
}

impl DynamoDbStore {
    /// Create a new DynamoDB store.
    pub fn new(context: Arc<CloudContext>) -> Self {
        Self { context }
    }
}

// TODO: Implement KeyValueStore trait
