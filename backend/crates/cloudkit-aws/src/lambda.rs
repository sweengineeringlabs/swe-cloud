//! AWS Lambda functions implementation.

use cloudkit::core::CloudContext;
use std::sync::Arc;

/// AWS Lambda functions implementation.
pub struct LambdaFunctions {
    context: Arc<CloudContext>,
}

impl LambdaFunctions {
    /// Create a new Lambda functions client.
    pub fn new(context: Arc<CloudContext>) -> Self {
        Self { context }
    }
}

// TODO: Implement Functions trait
