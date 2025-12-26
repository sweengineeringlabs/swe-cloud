use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ApiRequest {
    pub id: String,
    pub timestamp: String,
    pub service: String,
    pub operation: String,
    pub status: u16,
    pub duration_ms: u64,
}

#[component]
pub fn RequestTable(requests: Vec<ApiRequest>, on_select: EventHandler<ApiRequest>) -> Element {
    rsx! {
        table { class: "request-table",
            thead {
                tr {
                    th { "Time" }
                    th { "Service" }
                    th { "Operation" }
                    th { "Status" }
                    th { "Duration" }
                }
            }
            tbody {
                for req in requests {
                    RequestRow { request: req, on_select }
                }
            }
        }
    }
}

#[component]
fn RequestRow(request: ApiRequest, on_select: EventHandler<ApiRequest>) -> Element {
    let status_class = if request.status >= 200 && request.status < 300 {
        "status-ok"
    } else if request.status >= 400 {
        "status-error"
    } else {
        "status-other"
    };

    let req = request.clone();

    rsx! {
        tr {
            onclick: move |_| on_select.call(req.clone()),
            td { "{request.timestamp}" }
            td { "{request.service}" }
            td { "{request.operation}" }
            td { class: status_class, "{request.status}" }
            td { "{request.duration_ms}ms" }
        }
    }
}
