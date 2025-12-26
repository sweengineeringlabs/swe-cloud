use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ServiceStatus {
    pub name: String,
    pub service_type: String,
    pub state: ServiceState,
    pub port: Option<u16>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ServiceState {
    Running,
    Starting,
    Stopping,
    Stopped,
    Error(String),
}

#[component]
pub fn ServiceList(services: Vec<ServiceStatus>) -> Element {
    rsx! {
        table { class: "service-table",
            thead {
                tr {
                    th { "Service" }
                    th { "Status" }
                    th { "Port" }
                    th { "Actions" }
                }
            }
            tbody {
                for service in services {
                    ServiceRow { service }
                }
            }
        }
    }
}

#[component]
fn ServiceRow(service: ServiceStatus) -> Element {
    let status_class = match &service.state {
        ServiceState::Running => "status-running",
        ServiceState::Starting | ServiceState::Stopping => "status-pending",
        ServiceState::Stopped => "status-stopped",
        ServiceState::Error(_) => "status-error",
    };

    rsx! {
        tr {
            td { "{service.name}" }
            td { class: status_class,
                match &service.state {
                    ServiceState::Running => "Running",
                    ServiceState::Starting => "Starting",
                    ServiceState::Stopping => "Stopping",
                    ServiceState::Stopped => "Stopped",
                    ServiceState::Error(e) => "Error",
                }
            }
            td {
                match service.port {
                    Some(p) => rsx! { "{p}" },
                    None => rsx! { "-" },
                }
            }
            td { class: "actions",
                match &service.state {
                    ServiceState::Running => rsx! {
                        button { class: "btn-stop", "Stop" }
                    },
                    ServiceState::Stopped => rsx! {
                        button { class: "btn-start", "Start" }
                    },
                    _ => rsx! { span { "..." } },
                }
            }
        }
    }
}
