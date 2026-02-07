//! Oracle control-plane trait definitions.

use crate::{CloudResult, Request, Response};
use async_trait::async_trait;

/// Oracle cloud provider interface.
#[async_trait]
pub trait CloudProviderTrait: Send + Sync {
    /// Handle an incoming HTTP request.
    async fn handle_request(&self, req: Request) -> CloudResult<Response>;
}
