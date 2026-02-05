//! Functions trait for serverless function invocation.

use cloudkit_spi::CloudResult;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

/// Invocation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InvocationType {
    /// Synchronous invocation (wait for response)
    #[default]
    RequestResponse,
    /// Asynchronous invocation (fire and forget)
    Event,
    /// Dry run (validate only)
    DryRun,
}

/// Function invocation options.
#[derive(Debug, Clone, Default)]
pub struct InvokeOptions {
    /// Invocation type
    pub invocation_type: InvocationType,
    /// Client context (base64 encoded JSON)
    pub client_context: Option<String>,
    /// Qualifier (version or alias)
    pub qualifier: Option<String>,
}

impl InvokeOptions {
    /// Create new invoke options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set invocation type.
    pub fn invocation_type(mut self, invocation_type: InvocationType) -> Self {
        self.invocation_type = invocation_type;
        self
    }

    /// Set qualifier.
    pub fn qualifier(mut self, qualifier: impl Into<String>) -> Self {
        self.qualifier = Some(qualifier.into());
        self
    }

    /// Set async invocation.
    pub fn async_invoke(mut self) -> Self {
        self.invocation_type = InvocationType::Event;
        self
    }
}

/// Function invocation result.
#[derive(Debug, Clone)]
pub struct InvokeResult {
    /// Status code
    pub status_code: u16,
    /// Response payload
    pub payload: Option<Vec<u8>>,
    /// Function error (if any)
    pub function_error: Option<String>,
    /// Executed version
    pub executed_version: Option<String>,
    /// Log result (last 4KB of logs)
    pub log_result: Option<String>,
}

impl InvokeResult {
    /// Check if invocation was successful.
    pub fn is_success(&self) -> bool {
        self.function_error.is_none() && self.status_code == 200
    }

    /// Parse response payload as JSON.
    pub fn parse_payload<T: DeserializeOwned>(&self) -> CloudResult<Option<T>> {
        match &self.payload {
            Some(data) => {
                let value = serde_json::from_slice(data)?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
}

/// Serverless functions service trait.
///
/// This trait abstracts serverless function invocation across cloud providers:
/// - AWS Lambda
/// - Azure Functions
/// - Google Cloud Functions
/// - Oracle Functions
///
/// # Example
///
/// ```rust,ignore
/// use cloudkit::api::Functions;
///
/// #[derive(Serialize)]
/// struct Request { name: String }
///
/// #[derive(Deserialize)]
/// struct Response { greeting: String }
///
/// async fn invoke<F: Functions>(functions: &F) -> CloudResult<()> {
///     let request = Request { name: "World".to_string() };
///     let response: Response = functions.invoke_json("hello", &request).await?;
///     println!("{}", response.greeting);
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait Functions: Send + Sync {
    /// Invoke a function with raw payload.
    async fn invoke(
        &self,
        function_name: &str,
        payload: &[u8],
    ) -> CloudResult<InvokeResult>;

    /// Invoke a function with options.
    async fn invoke_with_options(
        &self,
        function_name: &str,
        payload: &[u8],
        options: InvokeOptions,
    ) -> CloudResult<InvokeResult>;

    /// Invoke a function with JSON payload and response.
    async fn invoke_json<T, R>(&self, function_name: &str, payload: &T) -> CloudResult<R>
    where
        T: Serialize + Send + Sync,
        R: DeserializeOwned + Send;

    /// Invoke a function asynchronously (fire and forget).
    async fn invoke_async(
        &self,
        function_name: &str,
        payload: &[u8],
    ) -> CloudResult<()>;

    /// List available functions.
    async fn list_functions(&self) -> CloudResult<Vec<String>>;

    /// Check if a function exists.
    async fn function_exists(&self, function_name: &str) -> CloudResult<bool>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invoke_options_builder() {
        let options = InvokeOptions::new()
            .invocation_type(InvocationType::Event)
            .qualifier("v1");

        assert_eq!(options.invocation_type, InvocationType::Event);
        assert_eq!(options.qualifier, Some("v1".to_string()));
    }

    #[test]
    fn test_invoke_result_success() {
        let result = InvokeResult {
            status_code: 200,
            payload: Some(b"{}".to_vec()),
            function_error: None,
            executed_version: Some("$LATEST".to_string()),
            log_result: None,
        };

        assert!(result.is_success());
    }

    #[test]
    fn test_invoke_result_error() {
        let result = InvokeResult {
            status_code: 200,
            payload: None,
            function_error: Some("Unhandled".to_string()),
            executed_version: None,
            log_result: None,
        };

        assert!(!result.is_success());
    }
}

