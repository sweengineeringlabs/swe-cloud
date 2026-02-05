//! HTTP Gateway - Ingress, Router, Dispatcher

pub mod dispatcher;
#[allow(clippy::module_inception)]
pub mod gateway;
pub mod ingress;
pub mod dashboard;

pub use gateway::create_router;
