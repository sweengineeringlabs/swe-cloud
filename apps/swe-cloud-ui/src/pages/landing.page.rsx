//! Landing page component.
//!
//! Route: /
//! Main landing page for SWE Cloud platform.

use rsc::prelude::*;

#[page]
pub fn Landing() -> Element {
    rsx! {
        div(class: "landing-page") {
            header(class: "landing-header") {
                div(class: "brand") {
                    h1 { "SWE Cloud" }
                }
                nav(class: "main-nav") {
                    a(href: "/", class: "nav-link active") { "Dashboard" }
                    a(href: "/cloudemu", class: "nav-link") { "CloudEmu" }
                    a(href: "/cloudkit", class: "nav-link") { "CloudKit" }
                    a(href: "/iac", class: "nav-link") { "IAC" }
                    a(href: "/settings", class: "nav-link") { "Settings" }
                }
            }

            main(class: "landing-content") {
                section(class: "hero") {
                    h1(class: "hero-title") { "Software Engineering Cloud" }
                    p(class: "hero-subtitle") {
                        "Build, deploy, and scale your applications with the power of RustScript and WebAssembly"
                    }
                }

                section(class: "features-grid") {
                    FeatureCard(
                        icon: "cloud",
                        title: "CloudEmu",
                        description: "Cloud service emulation for local development",
                        href: "/cloudemu",
                    )
                    FeatureCard(
                        icon: "box",
                        title: "CloudKit",
                        description: "Cloud infrastructure management toolkit",
                        href: "/cloudkit",
                    )
                    FeatureCard(
                        icon: "server",
                        title: "IAC",
                        description: "Infrastructure as Code with Terraform",
                        href: "/iac",
                    )
                }

                section(class: "stats-grid") {
                    StatCard(title: "Avg Response", value: "2.5ms")
                    StatCard(title: "Uptime SLA", value: "99.9%")
                    StatCard(title: "Edge Locations", value: "150+")
                    StatCard(title: "Active Projects", value: "10K+")
                }
            }

            footer(class: "landing-footer") {
                p { "Built with RustScript • Powered by WebAssembly" }
            }
        }
    }
}

#[component]
fn FeatureCard(icon: &str, title: &str, description: &str, href: &str) -> Element {
    rsx! {
        a(href: href, class: "feature-card") {
            div(class: "feature-icon") {
                span(class: format!("icon-{}", icon)) {}
            }
            h3(class: "feature-title") { {title} }
            p(class: "feature-desc") { {description} }
        }
    }
}

#[component]
fn StatCard(title: &str, value: &str) -> Element {
    rsx! {
        div(class: "stat-card") {
            div(class: "stat-value") { {value} }
            div(class: "stat-label") { {title} }
        }
    }
}
