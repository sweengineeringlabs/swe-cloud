// CloudKit Landing Page
// Overview page for cloud toolkit feature

use rsc::prelude::*;
use @modules::layout::{StatCard, SectionHeader, FeatureCard};

/// CloudKit landing page
#[page(route = "/cloudkit", title = "CloudKit")]
pub fn CloudkitLanding() -> Element {
    rsx! {
        div(class: "landing-page cloudkit-landing") {
            // Header
            section(class: "page-header") {
                h1 { "CloudKit" }
                p { "Cloud infrastructure management toolkit" }
            }

            // Stats
            section(class: "stats-section") {
                div(class: "stats-grid") {
                    StatCard(
                        title: "Total Resources".to_string(),
                        value: "47".to_string(),
                        icon: "database".to_string(),
                        source: "/api/cloudkit/stats/resources".to_string(),
                    )
                    StatCard(
                        title: "Operations".to_string(),
                        value: "12".to_string(),
                        icon: "activity".to_string(),
                        source: "/api/cloudkit/stats/operations".to_string(),
                    )
                    StatCard(
                        title: "API Calls".to_string(),
                        value: "2.4k".to_string(),
                        icon: "zap".to_string(),
                        source: "/api/cloudkit/stats/api-calls".to_string(),
                    )
                }
            }

            // Features
            section(class: "features-section") {
                SectionHeader(title: "Features".to_string())
                div(class: "features-grid") {
                    FeatureCard(
                        id: "resources".to_string(),
                        title: "Resources".to_string(),
                        description: "Browse and manage cloud resources".to_string(),
                        icon: "database".to_string(),
                        href: "/cloudkit/resources".to_string(),
                    )
                    FeatureCard(
                        id: "operations".to_string(),
                        title: "Operations".to_string(),
                        description: "Execute and monitor operations".to_string(),
                        icon: "activity".to_string(),
                        href: "/cloudkit/operations".to_string(),
                    )
                    FeatureCard(
                        id: "explorer".to_string(),
                        title: "API Explorer".to_string(),
                        description: "Explore and test cloud APIs".to_string(),
                        icon: "compass".to_string(),
                        href: "/cloudkit/explorer".to_string(),
                    )
                }
            }

            // Recent operations
            section(class: "operations-section") {
                SectionHeader(
                    title: "Recent Operations".to_string(),
                    action_label: Some("View All".to_string()),
                    action_href: Some("/cloudkit/operations".to_string()),
                )
                RecentOperationsList()
            }
        }
    }
}

#[component]
fn RecentOperationsList() -> Element {
    rsx! {
        div(class: "operations-list") {
            OperationItem(
                name: "CreateBucket".to_string(),
                resource: "s3://my-new-bucket".to_string(),
                status: "success".to_string(),
                time: "5 min ago".to_string(),
            )
            OperationItem(
                name: "PutItem".to_string(),
                resource: "dynamodb://users".to_string(),
                status: "success".to_string(),
                time: "12 min ago".to_string(),
            )
            OperationItem(
                name: "InvokeFunction".to_string(),
                resource: "lambda://api-handler".to_string(),
                status: "error".to_string(),
                time: "20 min ago".to_string(),
            )
        }
    }
}

#[component]
fn OperationItem(name: String, resource: String, status: String, time: String) -> Element {
    let status_class = match status.as_str() {
        "success" => "status-success",
        "error" => "status-error",
        _ => "status-pending",
    };

    rsx! {
        div(class: "operation-item") {
            div(class: "op-info") {
                strong { {&name} }
                code { {&resource} }
            }
            div(class: "op-meta") {
                span(class: format!("status-badge {}", status_class)) { {&status} }
                span(class: "time") { {&time} }
            }
        }
    }
}
