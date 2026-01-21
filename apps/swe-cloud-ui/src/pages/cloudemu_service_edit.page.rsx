use rsc::prelude::*;

#[page(route = "/cloudemu/:provider/:service/:id/edit", title = "Edit Service")]
pub fn CloudemuServiceEdit() -> Element {
    rsx! { div(class: "cloudemu-service-edit") { h1 { "Edit Service" } } }
}
