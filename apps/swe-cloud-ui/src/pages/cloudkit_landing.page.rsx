//! CloudkitLanding page component.
//!
//! Route: /cloudkit
//! Main landing page for CloudKit feature.

use rsc::prelude::*;
use crate::modules::context::{ProviderProvider, EnvironmentProvider};
use crate::modules::navigation::ContextBar;
use crate::modules::layout::{StatCard, SectionHeader, FeatureCard};

#[page]
pub fn CloudkitLanding() -> Element {
    rsx! {
        ProviderProvider {
            EnvironmentProvider {
                div(class: "cloudkit-page") {
                    ContextBar()

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
                    }
                }
            }
        }
    }
}
