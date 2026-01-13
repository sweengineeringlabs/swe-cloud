//! AWS EventBridge implementation.

use async_trait::async_trait;
use cloudkit::api::{
    Event, EventBus, EventRule, EventTarget, FailedEntry, PutEventsResult, RuleState,
};
use cloudkit::common::{CloudResult, CloudError};
use cloudkit::core::CloudContext;
use std::sync::Arc;

/// AWS EventBridge implementation.
pub struct AwsEvents {
    _context: Arc<CloudContext>,
    client: aws_sdk_eventbridge::Client,
}

impl AwsEvents {
    /// Create a new EventBridge client.
    pub fn new(context: Arc<CloudContext>, sdk_config: aws_config::SdkConfig) -> Self {
        let client = aws_sdk_eventbridge::Client::new(&sdk_config);
        Self { _context: context, client }
    }
}

#[async_trait]
impl EventBus for AwsEvents {
    async fn put_events(
        &self,
        bus_name: &str,
        events: Vec<Event>,
    ) -> CloudResult<PutEventsResult> {
        let mut entries = Vec::new();
        for event in events {
            entries.push(aws_sdk_eventbridge::types::PutEventsRequestEntry::builder()
                .event_bus_name(bus_name)
                .source(event.source)
                .detail_type(event.detail_type)
                .detail(event.detail.to_string())
                .set_resources(Some(event.resources))
                .build());
        }
        
        let resp = self.client.put_events()
            .set_entries(Some(entries))
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        let mut failed_entries = Vec::new();
        for entry in resp.entries() {
            if entry.error_code().is_some() {
                failed_entries.push(FailedEntry {
                    event_id: entry.event_id().unwrap_or_default().to_string(),
                    error_code: entry.error_code().unwrap_or_default().to_string(),
                    error_message: entry.error_message().unwrap_or_default().to_string(),
                });
            }
        }
        
        Ok(PutEventsResult {
            successful_count: resp.entries().len() - failed_entries.len(),
            failed_count: failed_entries.len(),
            failed_entries,
        })
    }

    async fn create_event_bus(&self, name: &str) -> CloudResult<String> {
        let resp = self.client.create_event_bus()
            .name(name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.event_bus_arn().unwrap_or_default().to_string())
    }

    async fn delete_event_bus(&self, name: &str) -> CloudResult<()> {
        self.client.delete_event_bus()
            .name(name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn list_event_buses(&self) -> CloudResult<Vec<String>> {
        let resp = self.client.list_event_buses()
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.event_buses().iter().map(|b| b.name().unwrap_or_default().to_string()).collect())
    }

    async fn put_rule(&self, bus_name: &str, rule: EventRule) -> CloudResult<String> {
        let mut req = self.client.put_rule()
            .event_bus_name(bus_name)
            .name(rule.name);
            
        if let Some(pattern) = rule.event_pattern {
            req = req.event_pattern(pattern.to_string());
        }
        
        if let Some(schedule) = rule.schedule_expression {
            req = req.schedule_expression(schedule);
        }
        
        req = req.set_state(Some(match rule.state {
            RuleState::Enabled => aws_sdk_eventbridge::types::RuleState::Enabled,
            RuleState::Disabled => aws_sdk_eventbridge::types::RuleState::Disabled,
        }));
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(resp.rule_arn().unwrap_or_default().to_string())
    }

    async fn delete_rule(&self, bus_name: &str, rule_name: &str) -> CloudResult<()> {
        self.client.delete_rule()
            .event_bus_name(bus_name)
            .name(rule_name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn enable_rule(&self, bus_name: &str, rule_name: &str) -> CloudResult<()> {
        self.client.enable_rule()
            .event_bus_name(bus_name)
            .name(rule_name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn disable_rule(&self, bus_name: &str, rule_name: &str) -> CloudResult<()> {
        self.client.disable_rule()
            .event_bus_name(bus_name)
            .name(rule_name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn list_rules(&self, bus_name: &str) -> CloudResult<Vec<EventRule>> {
        let resp = self.client.list_rules()
            .event_bus_name(bus_name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.rules().iter().map(|r| {
            EventRule {
                name: r.name().unwrap_or_default().to_string(),
                description: r.description().map(|s| s.to_string()),
                event_pattern: r.event_pattern().and_then(|p| serde_json::from_str(p).ok()),
                schedule_expression: r.schedule_expression().map(|s| s.to_string()),
                state: match r.state().unwrap_or(&aws_sdk_eventbridge::types::RuleState::Enabled) {
                    aws_sdk_eventbridge::types::RuleState::Enabled => RuleState::Enabled,
                    aws_sdk_eventbridge::types::RuleState::Disabled => RuleState::Disabled,
                    _ => RuleState::Enabled,
                },
                arn: r.arn().map(|s| s.to_string()),
            }
        }).collect())
    }

    async fn put_targets(
        &self,
        bus_name: &str,
        rule_name: &str,
        targets: Vec<EventTarget>,
    ) -> CloudResult<()> {
        let mut aws_targets = Vec::new();
        for t in targets {
            let mut builder = aws_sdk_eventbridge::types::Target::builder()
                .id(t.id)
                .arn(t.arn);
                
            if let Some(template) = t.input_template {
                builder = builder.input_transformer(
                    aws_sdk_eventbridge::types::InputTransformer::builder()
                        .input_template(template)
                        .set_input_paths_map(Some(t.input_paths))
                        .build()
                        .unwrap()
                );
            }
                
            aws_targets.push(builder.build().unwrap());
        }
        
        self.client.put_targets()
            .event_bus_name(bus_name)
            .rule(rule_name)
            .set_targets(Some(aws_targets))
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn remove_targets(
        &self,
        bus_name: &str,
        rule_name: &str,
        target_ids: &[&str],
    ) -> CloudResult<()> {
        self.client.remove_targets()
            .event_bus_name(bus_name)
            .rule(rule_name)
            .set_ids(Some(target_ids.iter().map(|s| s.to_string()).collect()))
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn list_targets(
        &self,
        bus_name: &str,
        rule_name: &str,
    ) -> CloudResult<Vec<EventTarget>> {
        let resp = self.client.list_targets_by_rule()
            .event_bus_name(bus_name)
            .rule(rule_name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.targets().iter().map(|t| {
            let (template, paths) = if let Some(transformer) = t.input_transformer() {
                (
                    Some(transformer.input_template().to_string()),
                    transformer.input_paths_map().unwrap_or(&std::collections::HashMap::new()).clone()
                )
            } else {
                (None, std::collections::HashMap::new())
            };

            EventTarget {
                id: t.id().to_string(),
                arn: t.arn().to_string(),
                input_template: template,
                input_paths: paths,
            }
        }).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cloudkit::core::ProviderType;

    async fn create_test_context() -> Arc<CloudContext> {
        Arc::new(
            CloudContext::builder(ProviderType::Aws)
                .build()
                .await
                .unwrap(),
        )
    }

    #[tokio::test]
    async fn test_eventbridge_new() {
        let sdk_config = aws_config::load_from_env().await;
        let context = create_test_context().await;
        let _events = AwsEvents::new(context, sdk_config);
    }
}
