use dioxus::prelude::*;
use crate::components::request_table::*;

#[component]
pub fn InspectorPage() -> Element {
    let mut recording = use_signal(|| false);
    let mut selected_request = use_signal(|| None::<ApiRequest>);

    let requests = use_signal(|| vec![
        ApiRequest {
            id: "1".into(),
            timestamp: "12:05:01".into(),
            service: "S3".into(),
            operation: "PutObject".into(),
            status: 200,
            duration_ms: 45,
        },
        ApiRequest {
            id: "2".into(),
            timestamp: "12:05:02".into(),
            service: "DynamoDB".into(),
            operation: "GetItem".into(),
            status: 200,
            duration_ms: 12,
        },
        ApiRequest {
            id: "3".into(),
            timestamp: "12:05:03".into(),
            service: "SQS".into(),
            operation: "SendMessage".into(),
            status: 200,
            duration_ms: 23,
        },
        ApiRequest {
            id: "4".into(),
            timestamp: "12:05:04".into(),
            service: "S3".into(),
            operation: "GetObject".into(),
            status: 404,
            duration_ms: 8,
        },
    ]);

    rsx! {
        div { class: "page inspector",
            header { class: "page-header",
                h1 { "Request Inspector" }
                button {
                    class: if *recording.read() { "btn-recording" } else { "btn-record" },
                    onclick: move |_| { let current = *recording.read(); recording.set(!current); },
                    if *recording.read() { "Stop Recording" } else { "Start Recording" }
                }
            }

            div { class: "inspector-layout",
                section { class: "requests-section",
                    RequestTable {
                        requests: requests.read().clone(),
                        on_select: move |req| selected_request.set(Some(req)),
                    }
                }

                section { class: "details-section",
                    h2 { "Request Details" }
                    match selected_request.read().as_ref() {
                        Some(req) => rsx! {
                            div { class: "request-detail",
                                p { strong { "Service: " } "{req.service}" }
                                p { strong { "Operation: " } "{req.operation}" }
                                p { strong { "Status: " } "{req.status}" }
                                p { strong { "Duration: " } "{req.duration_ms}ms" }
                            }
                        },
                        None => rsx! {
                            p { class: "placeholder", "Select a request to view details" }
                        },
                    }
                }
            }
        }
    }
}
