// IAC Layout
// Main layout for Infrastructure as Code feature pages

use rsc::prelude::*;
use crate::modules::layout::{WorkspaceLayout, Sidebar, BottomPanel};
use crate::modules::context::{use_environment, use_role};
use crate::modules::navigation::use_preset;

/// IAC feature layout
#[component]
pub fn IacLayout(children: Children) -> Element {
    let environment = use_environment();
    let role = use_role();
    let preset = use_preset("iac");

    rsx! {
        WorkspaceLayout(
            feature: "iac".to_string(),
            env_color: environment.color(),
        ) {
            // Environment warning banner for production
            if environment.is_production() {
                div(class: "env-warning-banner") {
                    span(class: "warning-icon") { "⚠" }
                    "Production environment - changes require approval"
                }
            }

            // Sidebar
            Sidebar(
                width: preset.sidebar.width,
                visible: preset.sidebar.visible,
            ) {
                // Modules panel
                div(class: "sidebar-panel") {
                    h3 { "Modules" }
                    ModuleTree()
                }

                // Environment panel
                div(class: "sidebar-panel") {
                    h3 { "Environment" }
                    EnvironmentInfo()
                }
            }

            // Main content
            main(class: "feature-content") {
                {children}
            }

            // Bottom panel
            if preset.bottom.visible {
                BottomPanel(
                    height: preset.bottom.height,
                    tabs: preset.bottom.tabs.clone(),
                )
            }
        }
    }
}

#[component]
fn ModuleTree() -> Element {
    rsx! {
        ul(class: "module-tree") {
            li(class: "module-node") {
                a(href: "/iac/modules/vpc") {
                    span(class: "module-icon") { "🌐" }
                    "VPC"
                }
            }
            li(class: "module-node") {
                a(href: "/iac/modules/s3-bucket") {
                    span(class: "module-icon") { "📦" }
                    "S3 Bucket"
                }
            }
            li(class: "module-node") {
                a(href: "/iac/modules/lambda-function") {
                    span(class: "module-icon") { "λ" }
                    "Lambda Function"
                }
            }
            li(class: "module-node") {
                a(href: "/iac/modules/rds-instance") {
                    span(class: "module-icon") { "🗄" }
                    "RDS Database"
                }
            }
        }
    }
}

#[component]
fn EnvironmentInfo() -> Element {
    let environment = use_environment();
    let current = environment.current_environment();

    match current {
        Some(env) => rsx! {
            div(class: "environment-info") {
                div(
                    class: "env-badge",
                    style: format!("background-color: {}", env.color)
                ) {
                    span(class: "env-icon") { {&env.icon} }
                    span(class: "env-label") { {&env.label} }
                }
                p(class: "env-description") { {&env.api_base} }
                if env.read_only_default {
                    span(class: "read-only-badge") { "Read Only" }
                }
            }
        },
        None => rsx! { div {} }
    }
}
