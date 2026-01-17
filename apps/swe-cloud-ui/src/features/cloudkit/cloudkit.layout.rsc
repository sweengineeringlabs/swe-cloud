// CloudKit Layout
// Main layout for CloudKit feature pages

use rsc::prelude::*;
use @modules::layout::{WorkspaceLayout, Sidebar, BottomPanel};
use @modules::context::{use_provider, use_environment};
use @modules::navigation::use_preset;

/// CloudKit feature layout
#[component]
pub fn CloudkitLayout(children: Children) -> Element {
    let provider = use_provider();
    let environment = use_environment();
    let preset = use_preset("cloudkit");

    rsx! {
        WorkspaceLayout(
            feature: "cloudkit".to_string(),
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
                    h3 { "Provider" }
                    ProviderSelector()
                }

                // Services panel
                div(class: "sidebar-panel") {
                    h3 { "Services" }
                    ServiceTree(provider_id: provider.current.clone())
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
fn ProviderSelector() -> Element {
    let mut provider = use_provider();

    rsx! {
        select(
            class: "provider-selector",
            value: &provider.current,
            onchange: move |e| provider.switch(&e.value)
        ) {
            for p in &provider.options {
                option(value: &p.id) { {&p.label} }
            }
        }
    }
}

#[component]
fn ServiceTree(provider_id: String) -> Element {
    let provider = use_provider();
    let services = provider.services();

    rsx! {
        ul(class: "service-tree") {
            for service in services {
                li(class: "service-node") {
                    span(class: "service-icon") { {&service.icon} }
                    span(class: "service-label") { {&service.label} }
                }
            }
        }
    }
}
