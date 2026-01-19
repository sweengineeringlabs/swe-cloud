use rsc::prelude::*;

#[page(route = "/cloudemu/:provider/:service", title = "Service")]
pub fn CloudemuService() -> Element {
    rsx! { div(class: "cloudemu-service") { h1 { "Service" } } }
}
