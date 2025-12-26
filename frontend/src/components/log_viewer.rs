use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LogEntry {
    pub timestamp: String,
    pub service: String,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

#[component]
pub fn LogViewer(logs: Vec<LogEntry>) -> Element {
    rsx! {
        div { class: "log-viewer",
            div { class: "log-header",
                h3 { "Logs" }
                button { class: "btn-clear", "Clear" }
            }
            div { class: "log-entries",
                for log in logs {
                    LogEntryRow { entry: log }
                }
            }
        }
    }
}

#[component]
fn LogEntryRow(entry: LogEntry) -> Element {
    let level_class = match entry.level {
        LogLevel::Info => "log-info",
        LogLevel::Warn => "log-warn",
        LogLevel::Error => "log-error",
        LogLevel::Debug => "log-debug",
    };

    rsx! {
        div { class: "log-entry {level_class}",
            span { class: "timestamp", "{entry.timestamp}" }
            span { class: "service", "[{entry.service}]" }
            span { class: "message", "{entry.message}" }
        }
    }
}
