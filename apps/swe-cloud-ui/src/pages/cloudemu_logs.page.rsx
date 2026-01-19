use rsc::prelude::*;

#[page(route = "/cloudemu/logs", title = "Request Logs")]
pub fn CloudemuLogs() -> Element {
    rsx! { div(class: "cloudemu-logs") { h1 { "Request Logs" } } }
}
