//! IacLanding page component.
//!
//! Route: /iac
//! Main landing page for IAC (Infrastructure as Code) feature.

use rsc::prelude::*;
use crate::modules::context::{ProviderProvider, EnvironmentProvider, use_environment};
use crate::modules::navigation::ContextBar;
use crate::modules::layout::{StatCard, SectionHeader, ActionCard};

#[page]
pub fn IacLanding() -> Element {
    rsx! {
        ProviderProvider {
            EnvironmentProvider {
                div(class: "iac-page") {
                    ContextBar()

                    IacLandingContent()
                }
            }
        }
    }
}

#[component]
fn IacLandingContent() -> Element {
    let environment = use_environment();

    rsx! {
        div(class: "landing-page iac-landing") {
            // Header
            section(class: "page-header") {
                h1 { "Infrastructure as Code" }
                p { "Manage and deploy infrastructure with Terraform" }

                if environment.is_production() {
                    div(class: "env-warning") {
                        span { "Warning: " }
                        "Production environment - changes require approval"
                    }
                }
            }

            // Stats
            section(class: "stats-section") {
                div(class: "stats-grid") {
                    StatCard(
                        title: "Modules".to_string(),
                        value: "8".to_string(),
                        icon: "box".to_string(),
                        source: "/api/iac/stats/modules".to_string(),
                    )
                    StatCard(
                        title: "Deployments".to_string(),
                        value: "24".to_string(),
                        icon: "rocket".to_string(),
                        source: "/api/iac/stats/deployments".to_string(),
                    )
                    StatCard(
                        title: "Resources".to_string(),
                        value: "156".to_string(),
                        icon: "server".to_string(),
                        source: "/api/iac/stats/resources".to_string(),
                    )
                    StatCard(
                        title: "Pending".to_string(),
                        value: "2".to_string(),
                        icon: "clock".to_string(),
                        source: "/api/iac/stats/pending".to_string(),
                    )
                }
            }

            // Quick actions
            section(class: "actions-section") {
                SectionHeader(title: "Quick Actions".to_string())
                div(class: "actions-grid") {
                    ActionCard(
                        id: "deploy".to_string(),
                        title: "Deploy".to_string(),
                        description: "Deploy infrastructure changes".to_string(),
                        icon: "upload-cloud".to_string(),
                        href: "/workflow/deploy-infrastructure".to_string(),
                        primary: true,
                    )
                    ActionCard(
                        id: "modules".to_string(),
                        title: "Modules".to_string(),
                        description: "Browse Terraform modules".to_string(),
                        icon: "box".to_string(),
                        href: "/iac/modules".to_string(),
                        primary: false,
                    )
                    ActionCard(
                        id: "state".to_string(),
                        title: "State".to_string(),
                        description: "View infrastructure state".to_string(),
                        icon: "file-text".to_string(),
                        href: "/iac/state".to_string(),
                        primary: false,
                    )
                }
            }
        }
    }
}
