// Request Table Component
// Displays recent API request logs

use rsc::prelude::*;
use crate::cloudemu_type::{RequestLog, LogFilter};

/// Request log table
#[component]
pub fn RequestTable(
    limit: usize = 10,
    filter: Option<LogFilter>,
) -> Element {
    let (logs, _) = use_state::<Vec<RequestLog>>(get_mock_logs());

    rsx! {
        div(class: "request-table-wrapper") {
            table(class: "request-table") {
                thead {
                    tr {
                        th { "Method" }
                        th { "Path" }
                        th { "Provider" }
                        th { "Status" }
                        th { "Duration" }
                        th { "Time" }
                    }
                }
                tbody {
                    for log in logs.iter().take(limit) {
                        RequestRow(log: log.clone())
                    }
                }
            }

            if logs.is_empty() {
                div(class: "empty-state") {
                    p { "No requests recorded yet" }
                }
            }
        }
    }
}

/// Single request row
#[component]
fn RequestRow(log: RequestLog) -> Element {
    let status_class = match log.status {
        200..=299 => "status-success",
        300..=399 => "status-redirect",
        400..=499 => "status-client-error",
        _ => "status-server-error",
    };

    rsx! {
        tr(class: "request-row") {
            td {
                span(class: format!("method-badge method-{}", log.method.to_lowercase())) {
                    {&log.method}
                }
            }
            td(class: "path-cell") {
                code { {&log.path} }
            }
            td {
                span(class: "provider-tag") { {&log.provider} }
            }
            td {
                span(class: format!("status-badge {}", status_class)) {
                    {log.status.to_string()}
                }
            }
            td { {format!("{}ms", log.duration_ms)} }
            td(class: "time-cell") { {&log.timestamp} }
        }
    }
}

fn get_mock_logs() -> Vec<RequestLog> {
    vec![
        RequestLog {
            id: "1".to_string(),
            method: "PUT".to_string(),
            path: "/my-bucket/file.json".to_string(),
            provider: "AWS".to_string(),
            service: "s3".to_string(),
            status: 200,
            duration_ms: 12,
            timestamp: "2 min ago".to_string(),
            request_headers: HashMap::new(),
            request_body: None,
            response_headers: HashMap::new(),
            response_body: None,
        },
        RequestLog {
            id: "2".to_string(),
            method: "GET".to_string(),
            path: "/logs-bucket?list".to_string(),
            provider: "AWS".to_string(),
            service: "s3".to_string(),
            status: 200,
            duration_ms: 45,
            timestamp: "5 min ago".to_string(),
            request_headers: HashMap::new(),
            request_body: None,
            response_headers: HashMap::new(),
            response_body: None,
        },
        RequestLog {
            id: "3".to_string(),
            method: "POST".to_string(),
            path: "/my-table".to_string(),
            provider: "AWS".to_string(),
            service: "dynamodb".to_string(),
            status: 400,
            duration_ms: 8,
            timestamp: "10 min ago".to_string(),
            request_headers: HashMap::new(),
            request_body: None,
            response_headers: HashMap::new(),
            response_body: None,
        },
    ]
}
