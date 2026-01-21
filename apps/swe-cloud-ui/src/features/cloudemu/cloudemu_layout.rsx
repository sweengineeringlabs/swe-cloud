// CloudEmu Layout
// Main layout for CloudEmu feature pages

use rsc::prelude::*;
use crate::modules::layout::{WorkspaceLayout, Sidebar, BottomPanel};
use crate::modules::context::{use_provider, use_environment};
use crate::modules::navigation::use_preset;

use crate::modules::context::{ProviderProvider, EnvironmentProvider};

/// CloudEmu feature layout
#[layout]
pub fn CloudemuLayout(children: Children) -> Element {
    rsx! {
        div(data_testid: "context-bar") { "Debug context bar" }
        ProviderProvider {
            EnvironmentProvider {
                CloudemuLayoutInner { {children} }
            }
        }
    }
}

#[component]
fn CloudemuLayoutInner(children: Children) -> Element {
    let provider = use_provider();
    let environment = use_environment();
    let preset = use_preset("cloudemu");

    rsx! {
        WorkspaceLayout(
            feature: "cloudemu".to_string(),
            provider_color: provider.color(),
            env_color: environment.color(),
        ) {
            // Sidebar
            Sidebar(
                width: preset.sidebar.width,
                visible: preset.sidebar.visible,
            ) {
                // Provider panel
                div(class: "sidebar-panel") {
                    h3 { "Providers" }
                    ProviderList()
                }

                // Services panel
                div(class: "sidebar-panel") {
                    h3 { "Services" }
                    ServiceList(provider_id: provider.current.clone())
                }
            }

            // Main content
            main(class: "feature-content") {
                {children}
            }

            // Bottom panel (conditional)
            if preset.bottom.visible {
                BottomPanel(
                    height: preset.bottom.height,
                    tabs: preset.bottom.tabs.clone(),
                )
            }
        }
    }
}

/// Provider list sidebar component
#[component]
fn ProviderList() -> Element {
    let provider_ctx = use_provider();

    rsx! {
        ul(class: "provider-list") {
            for p in &provider_ctx.options {
                if p.id != "multi" {
                    li(
                        class: format!("provider-item {}", if p.id == provider_ctx.current { "active" } else { "" }),
                        style: format!("--provider-color: {}", p.color)
                    ) {
                        a(href: format!("/cloudemu/{}", p.id)) {
                            span(class: "provider-icon") { {&p.icon} }
                            span(class: "provider-label") { {&p.label} }
                        }
                    }
                }
            }
        }
    }
}

/// Service list sidebar component
#[component]
fn ServiceList(provider_id: String) -> Element {
    let provider_ctx = use_provider();
    let services = provider_ctx.services();

    rsx! {
        ul(class: "service-list") {
            for service in services {
                li(class: "service-item") {
                    a(href: format!("/cloudemu/{}/{}", provider_id, service.id)) {
                        span(class: "service-icon") { {&service.icon} }
                        span(class: "service-label") { {&service.label} }
                    }
                }
            }
        }
    }
}
