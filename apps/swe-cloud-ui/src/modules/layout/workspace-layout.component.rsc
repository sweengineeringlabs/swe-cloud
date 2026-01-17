// Workspace Layout Component
// Main application workspace with context bar, header, sidebar, content, and bottom panel

use rsc::prelude::*;
use @modules::context::{use_provider, use_environment};
use @modules::navigation::{ContextBar, Header};

/// Main workspace layout component
#[component]
pub fn WorkspaceLayout(
    feature: String,
    provider_color: Option<String>,
    env_color: Option<String>,
    children: Children,
) -> Element {
    let provider = use_provider();
    let environment = use_environment();

    let p_color = provider_color.unwrap_or_else(|| provider.color().unwrap_or("#666").to_string());
    let e_color = env_color.unwrap_or_else(|| environment.color().unwrap_or("#10B981").to_string());

    rsx! {
        div(
            class: format!("workspace-layout feature-{}", feature),
            style: format!("--provider-color: {}; --env-color: {}", p_color, e_color)
        ) {
            // Context bar (provider/environment switcher)
            ContextBar()

            // Header
            Header()

            // Main workspace body
            div(class: "workspace-body") {
                {children}
            }

            // Status bar
            StatusBar(feature: feature.clone())
        }
    }
}

/// Status bar component
#[component]
fn StatusBar(feature: String) -> Element {
    let provider = use_provider();
    let environment = use_environment();

    rsx! {
        footer(class: "status-bar") {
            div(class: "status-left") {
                span(class: "status-item feature") { {&feature} }
            }
            div(class: "status-center") {
                // Connection status, etc.
            }
            div(class: "status-right") {
                span(
                    class: "status-item provider",
                    style: format!("color: {}", provider.color().unwrap_or("#666"))
                ) {
                    {provider.current_provider().map(|p| p.label.as_str()).unwrap_or("Unknown")}
                }
                span(
                    class: "status-item environment",
                    style: format!("color: {}", environment.color().unwrap_or("#10B981"))
                ) {
                    {environment.current_environment().map(|e| e.label.as_str()).unwrap_or("Local")}
                }
            }
        }
    }
}
