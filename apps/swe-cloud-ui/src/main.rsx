// SWE Cloud UI Entry Point
// Uses use_state hooks for reactive state management

use rsc::prelude::*;

#[component]
pub fn App() -> Element {
    // Provider state
    let (provider, set_provider) = use_state("aws".to_string());
    let (dropdown_open, set_dropdown_open) = use_state(false);

    // Environment state
    let (environment, set_environment) = use_state("local".to_string());

    // Provider label lookup
    let provider_label = match provider.as_str() {
        "aws" => "AWS",
        "azure" => "Azure",
        "gcp" => "GCP",
        "zerocloud" => "ZeroCloud",
        _ => "AWS"
    };

    rsx! {
        div(class: "app", data_testid: "app") {
            div(class: "context-bar", data_testid: "context-bar") {
                // Provider selector (left side)
                div(class: "context-left") {
                    div(class: "context-selector provider-selector", data_testid: "provider-selector") {
                        button(
                            class: "selector-button",
                            data_testid: "provider-button",
                            onclick: move |_| set_dropdown_open(!*dropdown_open)
                        ) {
                            span(class: "selector-icon") { "☁" }
                            span(class: "selector-label", data_testid: "provider-label") { {provider_label} }
                            span(class: "selector-arrow") { "▼" }
                        }
                        if *dropdown_open {
                            div(class: "selector-dropdown", data_testid: "provider-dropdown") {
                                button(
                                    class: format!("selector-option {}", if *provider == "aws" { "active" } else { "" }),
                                    data_testid: "provider-option-aws",
                                    onclick: move |_| { set_provider("aws".to_string()); set_dropdown_open(false); }
                                ) {
                                    span(class: "option-icon") { "☁" }
                                    span(class: "option-label") { "AWS" }
                                }
                                button(
                                    class: format!("selector-option {}", if *provider == "azure" { "active" } else { "" }),
                                    data_testid: "provider-option-azure",
                                    onclick: move |_| { set_provider("azure".to_string()); set_dropdown_open(false); }
                                ) {
                                    span(class: "option-icon") { "◈" }
                                    span(class: "option-label") { "Azure" }
                                }
                                button(
                                    class: format!("selector-option {}", if *provider == "gcp" { "active" } else { "" }),
                                    data_testid: "provider-option-gcp",
                                    onclick: move |_| { set_provider("gcp".to_string()); set_dropdown_open(false); }
                                ) {
                                    span(class: "option-icon") { "◉" }
                                    span(class: "option-label") { "GCP" }
                                }
                                button(
                                    class: format!("selector-option {}", if *provider == "zerocloud" { "active" } else { "" }),
                                    data_testid: "provider-option-zerocloud",
                                    onclick: move |_| { set_provider("zerocloud".to_string()); set_dropdown_open(false); }
                                ) {
                                    span(class: "option-icon") { "○" }
                                    span(class: "option-label") { "ZeroCloud" }
                                }
                            }
                        }
                    }
                }

                // Environment selector (right side)
                div(class: "context-right") {
                    div(class: "context-selector environment-selector", data_testid: "environment-selector") {
                        div(class: "env-pills", data_testid: "environment-pills") {
                            button(
                                class: format!("env-pill {}", if *environment == "local" { "active" } else { "" }),
                                data_testid: "env-option-local",
                                onclick: move |_| set_environment("local".to_string())
                            ) {
                                span(class: "env-icon") { "💻" }
                                span(class: "env-label", data_testid: "env-label-local") { "Local" }
                            }
                            button(
                                class: format!("env-pill {}", if *environment == "dev" { "active" } else { "" }),
                                data_testid: "env-option-dev",
                                onclick: move |_| set_environment("dev".to_string())
                            ) {
                                span(class: "env-icon") { "🔧" }
                                span(class: "env-label", data_testid: "env-label-dev") { "Dev" }
                            }
                            button(
                                class: format!("env-pill {}", if *environment == "staging" { "active" } else { "" }),
                                data_testid: "env-option-staging",
                                onclick: move |_| set_environment("staging".to_string())
                            ) {
                                span(class: "env-icon") { "🎭" }
                                span(class: "env-label", data_testid: "env-label-staging") { "Staging" }
                            }
                            button(
                                class: format!("env-pill {} production", if *environment == "prod" { "active" } else { "" }),
                                data_testid: "env-option-prod",
                                onclick: move |_| set_environment("prod".to_string())
                            ) {
                                span(class: "env-icon") { "🚀" }
                                span(class: "env-label", data_testid: "env-label-prod") { "Prod" }
                                span(class: "env-warning", data_testid: "env-warning") { "⚠" }
                            }
                        }
                    }
                }
            }
            main(class: "main-content") {
                h1 { "CloudEmu" }
                p { "Cloud service emulation" }
            }
        }
    }
}
