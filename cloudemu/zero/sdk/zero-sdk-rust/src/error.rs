use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZeroSdkError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error (Status: {status}): {body}")]
    Api {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Internal SDK error: {0}")]
    Internal(String),
}
