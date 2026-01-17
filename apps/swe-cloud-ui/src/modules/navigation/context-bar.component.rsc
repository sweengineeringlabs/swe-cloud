// Context Bar Component
// Provider and environment switcher bar

use rsc::prelude::*;
use @modules::context::{use_provider, use_environment};

/// Context bar at the top of the application
#[component]
pub fn ContextBar() -> Element {
    rsx! {
        div(class: "context-bar") {
            // Provider selector (left)
            div(class: "context-left") {
                ProviderSelector()
            }

            // Environment selector (right)
            div(class: "context-right") {
                EnvironmentSelector()
            }
        }
    }
}

/// Provider dropdown selector
#[component]
fn ProviderSelector() -> Element {
    let mut provider = use_provider();
    let (open, set_open) = use_state(false);
    let current = provider.current_provider();

    rsx! {
        div(class: "context-selector provider-selector") {
            button(
                class: "selector-button",
                onclick: move |_| set_open(!*open),
                style: format!("--selector-color: {}", current.map(|p| p.color.as_str()).unwrap_or("#666"))
            ) {
                if let Some(p) = current {
                    span(class: "selector-icon") { {&p.icon} }
                    span(class: "selector-label") { {&p.label} }
                }
                span(class: "selector-arrow") { "▼" }
            }

            if *open {
                div(class: "selector-dropdown") {
                    for option in &provider.options {
                        button(
                            class: format!("selector-option {}", if option.id == provider.current { "active" } else { "" }),
                            style: format!("--option-color: {}", option.color),
                            onclick: move |_| {
                                provider.switch(&option.id);
                                set_open(false);
                            }
                        ) {
                            span(class: "option-icon") { {&option.icon} }
                            span(class: "option-label") { {&option.label} }
                        }
                    }
                }
            }
        }
    }
}

/// Environment pills selector
#[component]
fn EnvironmentSelector() -> Element {
    let mut environment = use_environment();

    rsx! {
        div(class: "context-selector environment-selector") {
            div(class: "env-pills") {
                for option in &environment.options {
                    button(
                        class: format!(
                            "env-pill {} {}",
                            if option.id == environment.current { "active" } else { "" },
                            if option.id == "prod" { "production" } else { "" }
                        ),
                        style: format!("--env-color: {}", option.color),
                        onclick: move |_| environment.switch(&option.id)
                    ) {
                        span(class: "env-icon") { {&option.icon} }
                        span(class: "env-label") { {&option.label} }
                        if option.read_only_default {
                            span(class: "env-warning") { "⚠" }
                        }
                    }
                }
            }
        }
    }
}
