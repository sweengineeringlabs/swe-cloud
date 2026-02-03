// Stat Card Component
// Displays a statistic with icon, value, and optional trend

use rsc::prelude::*;

/// Statistics card component
#[component]
pub fn StatCard(
    title: String,
    value: String,
    icon: String,
    source: Option<String>,
    trend: Option<String>,
) -> Element {
    let trend_class = trend.as_ref().map(|t| {
        if t.starts_with('+') { "trend-up" }
        else if t.starts_with('-') { "trend-down" }
        else { "" }
    }).unwrap_or("");

    rsx! {
        div(class: "stat-card") {
            div(class: "stat-icon") { {&icon} }
            div(class: "stat-content") {
                span(class: "stat-value") { {&value} }
                span(class: "stat-title") { {&title} }
            }
            if let Some(t) = trend {
                span(class: format!("stat-trend {}", trend_class)) { {t} }
            }
        }
    }
}

/// Feature card for landing pages
#[component]
pub fn FeatureCard(
    id: String,
    title: String,
    description: String,
    icon: String,
    href: String,
) -> Element {
    rsx! {
        a(href: &href, class: "feature-card") {
            span(class: "feature-icon") { {&icon} }
            div(class: "feature-content") {
                h3 { {&title} }
                p { {&description} }
            }
            span(class: "feature-arrow") { "→" }
        }
    }
}

/// Action card for quick actions
#[component]
pub fn ActionCard(
    id: String,
    title: String,
    description: String,
    icon: String,
    href: String,
    primary: bool = false,
) -> Element {
    rsx! {
        a(
            href: &href,
            class: format!("action-card {}", if primary { "primary" } else { "" })
        ) {
            span(class: "action-icon") { {&icon} }
            div(class: "action-content") {
                h3 { {&title} }
                p { {&description} }
            }
        }
    }
}

/// Section header with optional action
#[component]
pub fn SectionHeader(
    title: String,
    action_label: Option<String>,
    action_href: Option<String>,
) -> Element {
    rsx! {
        div(class: "section-header") {
            h2 { {&title} }
            if let (Some(label), Some(href)) = (action_label, action_href) {
                a(href: &href, class: "section-action") { {label} " →" }
            }
        }
    }
}
