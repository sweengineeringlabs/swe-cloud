//! HTTP Gateway - Ingress, Router, Dispatcher

pub mod dispatcher;
#[allow(clippy::module_inception)]
pub mod gateway;
pub mod ingress;

pub use gateway::create_router;
