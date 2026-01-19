use rsc::prelude::*;

#[page(route = "/cloudkit/resources/:type/:id", title = "Resource Detail")]
pub fn CloudkitResourceDetail() -> Element {
    rsx! { div(class: "cloudkit-resource-detail") { h1 { "Resource Detail" } } }
}
