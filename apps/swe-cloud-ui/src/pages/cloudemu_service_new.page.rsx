use rsc::prelude::*;

#[page(route = "/cloudemu/:provider/:service/new", title = "New Service")]
pub fn CloudemuServiceNew() -> Element {
    rsx! { div(class: "cloudemu-service-new") { h1 { "New Service" } } }
}
