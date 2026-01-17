// SWE Cloud Platform UI - Main Entry Point
// CloudEmu + CloudKit + IAC Management

use rsc::prelude::*;

// Module declarations
mod features {
    pub mod cloudemu;
    pub mod cloudkit;
    pub mod iac;
}

mod modules {
    pub mod layout;
    pub mod navigation;
    pub mod context;
    pub mod workflow;
    pub mod config;
}

mod pages;

use modules::context::AppContextProvider;
use modules::layout::WorkspaceLayout;

/// Main application component
#[app]
pub fn App() -> Element {
    rsx! {
        AppContextProvider {
            Router {
                // Landing
                Route(path: "/") {
                    LandingPage()
                }

                // CloudEmu routes
                Route(path: "/cloudemu/*") {
                    features::cloudemu::CloudemuLayout {
                        Outlet()
                    }
                }

                // CloudKit routes
                Route(path: "/cloudkit/*") {
                    features::cloudkit::CloudkitLayout {
                        Outlet()
                    }
                }

                // IAC routes
                Route(path: "/iac/*") {
                    features::iac::IacLayout {
                        Outlet()
                    }
                }

                // Workflow routes
                Route(path: "/workflow/:workflowId") {
                    WorkflowRunnerPage()
                }

                // Settings routes
                Route(path: "/settings/*") {
                    SettingsPage()
                }

                // Fallback
                Route(path: "*") {
                    NotFoundPage()
                }
            }
        }
    }
}

#[page(route = "/", title = "SWE Cloud")]
fn LandingPage() -> Element {
    use modules::context::{use_provider, use_environment};
    use modules::layout::{StatCard, SectionHeader, FeatureCard};

    let provider = use_provider();
    let environment = use_environment();

    rsx! {
        WorkspaceLayout(feature: "dashboard".to_string()) {
            div(class: "landing-page dashboard") {
                section(class: "welcome-section") {
                    h1 { "Welcome to SWE Cloud" }
                    p {
                        "Manage your cloud resources across "
                        span(class: "provider-highlight", style: format!("color: {}", provider.color().unwrap_or("#666"))) {
                            {provider.current_provider().map(|p| p.label.as_str()).unwrap_or("Unknown")}
                        }
                    }
                }

                section(class: "stats-section") {
                    SectionHeader(title: "Overview".to_string())
                    div(class: "stats-grid") {
                        StatCard(title: "Active Services".to_string(), value: "12".to_string(), icon: "☁".to_string(), source: None, trend: Some("+2".to_string()))
                        StatCard(title: "Resources".to_string(), value: "47".to_string(), icon: "📦".to_string(), source: None, trend: Some("+5".to_string()))
                        StatCard(title: "Requests Today".to_string(), value: "1,234".to_string(), icon: "📊".to_string(), source: None, trend: None)
                        StatCard(title: "Deployments".to_string(), value: "8".to_string(), icon: "🚀".to_string(), source: None, trend: Some("+1".to_string()))
                    }
                }

                section(class: "features-section") {
                    SectionHeader(title: "Features".to_string())
                    div(class: "features-grid") {
                        FeatureCard(
                            id: "cloudemu".to_string(),
                            title: "CloudEmu".to_string(),
                            description: "Cloud service emulation".to_string(),
                            icon: "☁".to_string(),
                            href: "/cloudemu".to_string(),
                        )
                        FeatureCard(
                            id: "cloudkit".to_string(),
                            title: "CloudKit".to_string(),
                            description: "Infrastructure toolkit".to_string(),
                            icon: "📦".to_string(),
                            href: "/cloudkit".to_string(),
                        )
                        FeatureCard(
                            id: "iac".to_string(),
                            title: "Infrastructure".to_string(),
                            description: "Infrastructure as Code".to_string(),
                            icon: "🔧".to_string(),
                            href: "/iac".to_string(),
                        )
                    }
                }
            }
        }
    }
}

fn WorkflowRunnerPage() -> Element {
    rsx! {
        div(class: "workflow-runner") {
            h1 { "Workflow Runner" }
            p { "Workflow execution page" }
        }
    }
}

fn SettingsPage() -> Element {
    rsx! {
        div(class: "settings-page") {
            h1 { "Settings" }
        }
    }
}

fn NotFoundPage() -> Element {
    rsx! {
        div(class: "not-found-page") {
            h1 { "404 - Not Found" }
            p { "The page you're looking for doesn't exist." }
            a(href: "/") { "← Back to Dashboard" }
        }
    }
}
