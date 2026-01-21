use rsc::prelude::*;

#[page(route = "/cloudkit/operations/:id", title = "Operation Detail")]
pub fn CloudkitOperationDetail() -> Element {
    rsx! { div(class: "cloudkit-operation-detail") { h1 { "Operation Detail" } } }
}
