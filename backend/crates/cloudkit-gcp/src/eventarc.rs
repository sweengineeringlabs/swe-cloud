//! Google Cloud Eventarc implementation.

use async_trait::async_trait;
use cloudkit::api::{
    Event, EventBus, EventRule, EventTarget, PutEventsResult,
};
use cloudkit::common::CloudResult;
use cloudkit::core::CloudContext;
use std::sync::Arc;

/// Google Cloud Eventarc implementation.
pub struct GcpEventarc {
    _context: Arc<CloudContext>,
    // In a real implementation:
    // client: google_cloud_eventarc::Client,
}

impl GcpEventarc {
    /// Create a new Eventarc client.
    pub fn new(context: Arc<CloudContext>) -> Self {
        Self { _context: context }
    }
}

#[async_trait]
impl EventBus for GcpEventarc {
    async fn create_event_bus(&self, name: &str) -> CloudResult<String> {
        tracing::info!(
            provider = "gcp",
            service = "eventarc",
            bus = %name,
            "create_event_bus (channel) called"
        );
        Ok(format!("projects/my-project/locations/global/channels/{}", name))
    }

    async fn delete_event_bus(&self, name: &str) -> CloudResult<()> {
        tracing::info!(
            provider = "gcp",
            service = "eventarc",
            bus = %name,
            "delete_event_bus (channel) called"
        );
        Ok(())
    }

    async fn list_event_buses(&self) -> CloudResult<Vec<String>> {
        tracing::info!(
            provider = "gcp",
            service = "eventarc",
            "list_event_buses called"
        );
        Ok(vec![])
    }

    async fn put_events(&self, bus_name: &str, events: Vec<Event>) -> CloudResult<PutEventsResult> {
        tracing::info!(
            provider = "gcp",
            service = "eventarc",
            bus = %bus_name,
            count = %events.len(),
            "put_events called"
        );
        Ok(PutEventsResult {
            successful_count: events.len(),
            failed_count: 0,
            failed_entries: vec![],
        })
    }

    async fn put_rule(&self, bus_name: &str, rule: EventRule) -> CloudResult<String> {
        tracing::info!(
            provider = "gcp",
            service = "eventarc",
            bus = %bus_name,
            rule = %rule.name,
            "put_rule (trigger) called"
        );
        Ok(format!("projects/my-project/locations/global/triggers/{}", rule.name))
    }

    async fn delete_rule(&self, bus_name: &str, rule_name: &str) -> CloudResult<()> {
        tracing::info!(
            provider = "gcp",
            service = "eventarc",
            bus = %bus_name,
            rule = %rule_name,
            "delete_rule (trigger) called"
        );
        Ok(())
    }

    async fn enable_rule(&self, bus_name: &str, rule_name: &str) -> CloudResult<()> {
        tracing::info!(
            provider = "gcp",
            service = "eventarc",
            bus = %bus_name,
            rule = %rule_name,
            "enable_rule called"
        );
        Ok(())
    }

    async fn disable_rule(&self, bus_name: &str, rule_name: &str) -> CloudResult<()> {
        tracing::info!(
            provider = "gcp",
            service = "eventarc",
            bus = %bus_name,
            rule = %rule_name,
            "disable_rule called"
        );
        Ok(())
    }

    async fn list_rules(&self, bus_name: &str) -> CloudResult<Vec<EventRule>> {
        tracing::info!(
            provider = "gcp",
            service = "eventarc",
            bus = %bus_name,
            "list_rules called"
        );
        Ok(vec![])
    }

    async fn put_targets(
        &self,
        bus_name: &str,
        rule_name: &str,
        targets: Vec<EventTarget>,
    ) -> CloudResult<()> {
        tracing::info!(
            provider = "gcp",
            service = "eventarc",
            bus = %bus_name,
            rule = %rule_name,
            count = %targets.len(),
            "put_targets called"
        );
        Ok(())
    }

    async fn remove_targets(
        &self,
        bus_name: &str,
        rule_name: &str,
        target_ids: &[&str],
    ) -> CloudResult<()> {
        tracing::info!(
            provider = "gcp",
            service = "eventarc",
            bus = %bus_name,
            rule = %rule_name,
            count = %target_ids.len(),
            "remove_targets called"
        );
        Ok(())
    }

    async fn list_targets(
        &self,
        bus_name: &str,
        rule_name: &str,
    ) -> CloudResult<Vec<EventTarget>> {
        tracing::info!(
            provider = "gcp",
            service = "eventarc",
            bus = %bus_name,
            rule = %rule_name,
            "list_targets called"
        );
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cloudkit::core::ProviderType;

    async fn create_test_context() -> Arc<CloudContext> {
        Arc::new(
            CloudContext::builder(ProviderType::Gcp)
                .build()
                .await
                .unwrap(),
        )
    }

    #[tokio::test]
    async fn test_eventarc_new() {
        let context = create_test_context().await;
        let _bus = GcpEventarc::new(context);
    }
}
