// IAC Landing Page
// Overview page for Infrastructure as Code feature

use rsc::prelude::*;
use crate::modules::layout::{StatCard, SectionHeader, ActionCard};
use crate::modules::context::environment::use_environment;
use crate::modules::context::use_role;

/// IAC landing page
#[page(route = "/iac", title = "Infrastructure")]
pub fn IacLanding() -> Element {
    let environment = use_environment();
    let role = use_role();

    rsx! {
        div(class: "landing-page iac-landing") {
            // Header
            section(class: "page-header") {
                h1 { "Infrastructure as Code" }
                p { "Manage and deploy infrastructure with Terraform" }

                if environment.is_production() {
                    div(class: "env-warning") {
                        span { "⚠" }
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

            // Recent deployments
            section(class: "deployments-section") {
                SectionHeader(
                    title: "Recent Deployments".to_string(),
                    action_label: Some("View All".to_string()),
                    action_href: Some("/iac/deployments".to_string()),
                )
                DeploymentsList()
            }

            // Pending plans
            section(class: "plans-section") {
                SectionHeader(
                    title: "Pending Plans".to_string(),
                    action_label: Some("View All".to_string()),
                    action_href: Some("/iac/plans".to_string()),
                )
                PendingPlansList()
            }
        }
    }
}

#[component]
fn DeploymentsList() -> Element {
    rsx! {
        div(class: "deployments-list") {
            DeploymentItem(
                id: "deploy-123".to_string(),
                module: "vpc-module".to_string(),
                environment: "staging".to_string(),
                status: "success".to_string(),
                changes: "+3 -1 ~0".to_string(),
                time: "1 hour ago".to_string(),
            )
            DeploymentItem(
                id: "deploy-122".to_string(),
                module: "s3-bucket".to_string(),
                environment: "dev".to_string(),
                status: "success".to_string(),
                changes: "+1 -0 ~0".to_string(),
                time: "3 hours ago".to_string(),
            )
            DeploymentItem(
                id: "deploy-121".to_string(),
                module: "lambda-function".to_string(),
                environment: "dev".to_string(),
                status: "failed".to_string(),
                changes: "+0 -0 ~1".to_string(),
                time: "5 hours ago".to_string(),
            )
        }
    }
}

#[component]
fn DeploymentItem(
    id: String,
    module: String,
    environment: String,
    status: String,
    changes: String,
    time: String,
) -> Element {
    let status_class = match status.as_str() {
        "success" => "status-success",
        "failed" => "status-error",
        _ => "status-pending",
    };

    rsx! {
        a(href: format!("/iac/deployments/{}", id), class: "deployment-item") {
            div(class: "deploy-info") {
                strong { {&module} }
                span(class: "env-badge") { {&environment} }
            }
            div(class: "deploy-meta") {
                span(class: "changes") { {&changes} }
                span(class: format!("status-badge {}", status_class)) { {&status} }
                span(class: "time") { {&time} }
            }
        }
    }
}

#[component]
fn PendingPlansList() -> Element {
    rsx! {
        div(class: "plans-list") {
            PlanItem(
                id: "plan-456".to_string(),
                module: "rds-instance".to_string(),
                environment: "staging".to_string(),
                changes: "+1 -0 ~2".to_string(),
                created_by: "john@example.com".to_string(),
            )
            PlanItem(
                id: "plan-455".to_string(),
                module: "vpc-module".to_string(),
                environment: "prod".to_string(),
                changes: "+0 -0 ~3".to_string(),
                created_by: "jane@example.com".to_string(),
            )
        }
    }
}

#[component]
fn PlanItem(
    id: String,
    module: String,
    environment: String,
    changes: String,
    created_by: String,
) -> Element {
    rsx! {
        div(class: "plan-item") {
            div(class: "plan-info") {
                strong { {&module} }
                span(class: "env-badge") { {&environment} }
                span(class: "changes") { {&changes} }
            }
            div(class: "plan-meta") {
                span { "by " {&created_by} }
            }
            div(class: "plan-actions") {
                a(href: format!("/iac/plans/{}", id), class: "btn-review") { "Review" }
                button(class: "btn-approve") { "Approve" }
            }
        }
    }
}
