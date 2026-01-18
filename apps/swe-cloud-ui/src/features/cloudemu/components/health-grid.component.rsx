// Health Grid Component
// Displays service health status across providers

use rsc::prelude::*;
use crate::cloudemu_type::{ServiceHealth, HealthStatus};

/// Grid displaying service health status
#[component]
pub fn HealthGrid() -> Element {
    let (health, _) = use_state(get_mock_health());

    rsx! {
        div(class: "health-grid") {
            for item in &health {
                HealthCard(health: item.clone())
            }
        }
    }
}

/// Individual health status card
#[component]
fn HealthCard(health: ServiceHealth) -> Element {
    let status_class = match health.status {
        HealthStatus::Healthy => "healthy",
        HealthStatus::Degraded => "degraded",
        HealthStatus::Unhealthy => "unhealthy",
        HealthStatus::Unknown => "unknown",
    };

    let status_icon = match health.status {
        HealthStatus::Healthy => "checkmark",
        HealthStatus::Degraded => "warning",
        HealthStatus::Unhealthy => "error",
        HealthStatus::Unknown => "question",
    };

    rsx! {
        div(class: format!("health-card {}", status_class)) {
            div(class: "health-header") {
                span(class: "health-provider") { {&health.provider} }
                span(class: format!("health-status {}", status_class)) {
                    span(class: "status-icon") { {status_icon} }
                }
            }
            div(class: "health-metrics") {
                div(class: "metric") {
                    span(class: "metric-label") { "Latency" }
                    span(class: "metric-value") { {format!("{}ms", health.latency_ms)} }
                }
                div(class: "metric") {
                    span(class: "metric-label") { "Last Check" }
                    span(class: "metric-value") { {&health.last_check} }
                }
            }
        }
    }
}

fn get_mock_health() -> Vec<ServiceHealth> {
    vec![
        ServiceHealth {
            provider: "AWS".to_string(),
            status: HealthStatus::Healthy,
            latency_ms: 12,
            last_check: "Just now".to_string(),
        },
        ServiceHealth {
            provider: "Azure".to_string(),
            status: HealthStatus::Healthy,
            latency_ms: 18,
            last_check: "1 min ago".to_string(),
        },
        ServiceHealth {
            provider: "GCP".to_string(),
            status: HealthStatus::Degraded,
            latency_ms: 45,
            last_check: "2 min ago".to_string(),
        },
        ServiceHealth {
            provider: "ZeroCloud".to_string(),
            status: HealthStatus::Healthy,
            latency_ms: 8,
            last_check: "Just now".to_string(),
        },
    ]
}
