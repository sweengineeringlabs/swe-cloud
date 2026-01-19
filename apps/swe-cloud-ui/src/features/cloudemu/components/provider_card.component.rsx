// Provider Card Component
// Displays a cloud provider with service count and endpoint

use rsc::prelude::*;

/// Provider overview card
#[component]
pub fn ProviderCard(
    id: String,
    label: String,
    icon: String,
    color: String,
    endpoint: String,
    service_count: usize,
) -> Element {
    rsx! {
        a(
            href: format!("/cloudemu/{}", id),
            class: "provider-card",
            style: format!("--provider-color: {}", color)
        ) {
            div(class: "card-header") {
                span(class: "provider-icon") { {&icon} }
                h3 { {&label} }
            }
            div(class: "card-body") {
                div(class: "endpoint") {
                    span(class: "label") { "Endpoint:" }
                    code { {&endpoint} }
                }
                div(class: "services") {
                    span(class: "count") { {service_count.to_string()} }
                    span(class: "label") { " services" }
                }
            }
            div(class: "card-footer") {
                span(class: "arrow") { "→" }
            }
        }
    }
}

/// Small provider badge
#[component]
pub fn ProviderBadge(
    id: String,
    label: String,
    color: String,
) -> Element {
    rsx! {
        span(
            class: "provider-badge",
            style: format!("background-color: {}", color)
        ) {
            {&label}
        }
    }
}
