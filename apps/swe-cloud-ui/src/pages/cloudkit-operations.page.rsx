use rsc::prelude::*;

#[page(route = "/cloudkit/operations", title = "Operations")]
pub fn CloudkitOperations() -> Element {
    rsx! { div(class: "cloudkit-operations") { h1 { "Operations" } } }
}
