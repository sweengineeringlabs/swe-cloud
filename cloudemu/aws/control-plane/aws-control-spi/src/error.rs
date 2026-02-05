//! AWS control-plane specific errors

use thiserror::Error;

/// AWS control-plane errors
#[derive(Error, Debug)]
pub enum AwsControlError {
    /// Generic error
    #[error("AWS control error: {0}")]
    Generic(String),
}
