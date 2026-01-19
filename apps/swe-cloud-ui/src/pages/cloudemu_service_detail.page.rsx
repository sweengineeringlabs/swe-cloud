use rsc::prelude::*;

#[page(route = "/cloudemu/:provider/:service/:id", title = "Service Detail")]
pub fn CloudemuServiceDetail() -> Element {
    rsx! { div(class: "cloudemu-service-detail") { h1 { "Service Detail" } } }
}
