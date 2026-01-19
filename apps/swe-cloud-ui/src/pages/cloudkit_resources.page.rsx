use rsc::prelude::*;

#[page(route = "/cloudkit/resources", title = "Resources")]
pub fn CloudkitResources() -> Element {
    rsx! { div(class: "cloudkit-resources") { h1 { "Resources" } } }
}
