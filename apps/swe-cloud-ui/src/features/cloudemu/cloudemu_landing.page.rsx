// CloudEmu Landing Page
// Overview page for cloud emulation feature

use rsc::prelude::*;
use crate::modules::context::{ProviderProvider, EnvironmentProvider, use_provider};
use crate::modules::navigation::ContextBar;
use crate::modules::layout::{StatCard, SectionHeader};
use crate::components::{ProviderCard, RequestTable, HealthGrid};

/// CloudEmu landing page
#[page(route = "/cloudemu", name = "cloudemu-landing", title = "CloudEmu")]
pub fn CloudemuLanding() -> Element {
    rsx! {
        ProviderProvider {
            EnvironmentProvider {
                CloudemuLandingContent()
            }
        }
    }
}

#[component]
fn CloudemuLandingContent() -> Element {
    let provider = use_provider();

    rsx! {
        div(class: "cloudemu-page") {
            ContextBar()
            div(class: "landing-page cloudemu-landing") {
            // Header
            section(class: "page-header") {
                h1 { "CloudEmu" }
                p { "Cloud service emulation for local development" }
            }

            // Stats
            section(class: "stats-section") {
                div(class: "stats-grid") {
                    StatCard(
                        title: "Active Services".to_string(),
                        value: "12".to_string(),
                        icon: "cloud".to_string(),
                        source: "/api/cloudemu/stats/services".to_string(),
                    )
                    StatCard(
                        title: "Requests".to_string(),
                        value: "1.2k".to_string(),
                        icon: "activity".to_string(),
                        source: "/api/cloudemu/stats/requests".to_string(),
                    )
                    StatCard(
                        title: "Avg Latency".to_string(),
                        value: "12ms".to_string(),
                        icon: "clock".to_string(),
                        source: "/api/cloudemu/stats/latency".to_string(),
                    )
                }
            }

            // Providers overview
            section(class: "providers-section") {
                SectionHeader(title: "Active Providers".to_string())
                div(class: "provider-grid") {
                    for p in &provider.options {
                        if p.id != "multi" {
                            ProviderCard(
                                id: p.id.clone(),
                                label: p.label.clone(),
                                icon: p.icon.clone(),
                                color: p.color.clone(),
                                endpoint: p.endpoints.api.clone(),
                                service_count: p.services.len(),
                            )
                        }
                    }
                }
            }

            // Recent requests
            section(class: "requests-section") {
                SectionHeader(
                    title: "Recent Requests".to_string(),
                    action_label: Some("View All".to_string()),
                    action_href: Some("/cloudemu/logs".to_string()),
                )
                RequestTable(limit: 10)
            }

            // Service health
            section(class: "health-section") {
                SectionHeader(title: "Service Health".to_string())
                HealthGrid()
            }
            }
        }
    }
}
