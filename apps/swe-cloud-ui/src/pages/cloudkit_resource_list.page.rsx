use rsc::prelude::*;

#[page(route = "/cloudkit/resources/:type", title = "Resource List")]
pub fn CloudkitResourceList() -> Element {
    rsx! { div(class: "cloudkit-resource-list") { h1 { "Resource List" } } }
}
