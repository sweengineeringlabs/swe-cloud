//! CloudemuLanding page component.
//!
//! Route: /cloudemu
//! Main landing page for CloudEmu feature.

use rsc::prelude::*;
use crate::modules::context::{ProviderProvider, EnvironmentProvider, use_provider};
use crate::modules::navigation::ContextBar;
use crate::modules::layout::{StatCard, SectionHeader};

#[page]
pub fn CloudemuLanding() -> Element {
    rsx! {
        ProviderProvider {
            EnvironmentProvider {
                div(class: "cloudemu-page") {
                    ContextBar()

                    CloudemuLandingContent()
                }
            }
        }
    }
}

#[component]
fn CloudemuLandingContent() -> Element {
    let provider = use_provider();

    rsx! {
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
                            )
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ProviderCard(id: String, label: String, icon: String, color: String) -> Element {
    rsx! {
        a(
            href: format!("/cloudemu/{}", id),
            class: "provider-card",
            style: format!("--provider-color: {}", color)
        ) {
            div(class: "provider-icon") { {&icon} }
            div(class: "provider-label") { {&label} }
        }
    }
}
