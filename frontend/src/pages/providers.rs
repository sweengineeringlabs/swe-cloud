use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CloudProvider {
    pub id: String,
    pub name: String,
    pub provider_type: ProviderType,
    pub connected: bool,
    pub endpoint: Option<String>,
    pub region: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ProviderType {
    CloudEmu,
    AWS,
    Azure,
    GCP,
    Oracle,
}

#[component]
pub fn ProvidersPage() -> Element {
    let providers = use_signal(|| vec![
        CloudProvider {
            id: "local".into(),
            name: "CloudEmu (Local)".into(),
            provider_type: ProviderType::CloudEmu,
            connected: true,
            endpoint: Some("http://localhost:4566".into()),
            region: None,
        },
        CloudProvider {
            id: "aws-prod".into(),
            name: "AWS (Production)".into(),
            provider_type: ProviderType::AWS,
            connected: false,
            endpoint: None,
            region: Some("us-east-1".into()),
        },
        CloudProvider {
            id: "azure-staging".into(),
            name: "Azure (Staging)".into(),
            provider_type: ProviderType::Azure,
            connected: false,
            endpoint: None,
            region: Some("eastus".into()),
        },
    ]);

    rsx! {
        div { class: "page providers",
            header { class: "page-header",
                h1 { "Cloud Providers" }
                button { class: "btn-primary", "+ Add Provider" }
            }

            div { class: "provider-list",
                for provider in providers.read().iter() {
                    ProviderCard { provider: provider.clone() }
                }
            }
        }
    }
}

#[component]
fn ProviderCard(provider: CloudProvider) -> Element {
    let status_icon = if provider.connected { "connected" } else { "disconnected" };
    let status_class = if provider.connected { "active" } else { "inactive" };

    rsx! {
        div { class: "provider-card {status_class}",
            div { class: "provider-header",
                span { class: "status-icon {status_icon}" }
                h3 { "{provider.name}" }
                button { class: "btn-connect",
                    if provider.connected { "Active" } else { "Connect" }
                }
            }
            div { class: "provider-details",
                if let Some(endpoint) = &provider.endpoint {
                    p { "Endpoint: {endpoint}" }
                }
                if let Some(region) = &provider.region {
                    p { "Region: {region}" }
                }
            }
        }
    }
}
