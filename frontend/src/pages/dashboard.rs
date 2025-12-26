use dioxus::prelude::*;
use crate::components::{service_list::*, log_viewer::*};

#[component]
pub fn DashboardPage() -> Element {
    let services = use_signal(|| vec![
        ServiceStatus {
            name: "S3".into(),
            service_type: "Object Storage".into(),
            state: ServiceState::Running,
            port: Some(4566),
        },
        ServiceStatus {
            name: "DynamoDB".into(),
            service_type: "Key-Value".into(),
            state: ServiceState::Running,
            port: Some(4567),
        },
        ServiceStatus {
            name: "SQS".into(),
            service_type: "Message Queue".into(),
            state: ServiceState::Starting,
            port: Some(4568),
        },
        ServiceStatus {
            name: "SNS".into(),
            service_type: "Pub/Sub".into(),
            state: ServiceState::Stopped,
            port: None,
        },
        ServiceStatus {
            name: "Lambda".into(),
            service_type: "Functions".into(),
            state: ServiceState::Stopped,
            port: None,
        },
    ]);

    let logs = use_signal(|| vec![
        LogEntry {
            timestamp: "12:03:45".into(),
            service: "S3".into(),
            level: LogLevel::Info,
            message: "Bucket 'test-bucket' created".into(),
        },
        LogEntry {
            timestamp: "12:03:46".into(),
            service: "S3".into(),
            level: LogLevel::Info,
            message: "PUT object 'data.json' (1.2 KB)".into(),
        },
        LogEntry {
            timestamp: "12:03:47".into(),
            service: "DynamoDB".into(),
            level: LogLevel::Info,
            message: "Table 'users' created".into(),
        },
    ]);

    rsx! {
        div { class: "page dashboard",
            header { class: "page-header",
                h1 { "CloudEmu" }
                div { class: "controls",
                    button { class: "btn-primary", "Start All" }
                    button { class: "btn-secondary", "Stop All" }
                }
            }

            section { class: "services-section",
                h2 { "Services" }
                ServiceList { services: services.read().clone() }
            }

            section { class: "logs-section",
                LogViewer { logs: logs.read().clone() }
            }
        }
    }
}
