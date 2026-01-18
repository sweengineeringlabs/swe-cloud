use rsc::prelude::*;

#[page(route = "/cloudkit", title = "CloudKit")]
pub fn CloudkitOverview() -> Element {
    rsx! { div(class: "cloudkit-overview") { h1 { "CloudKit" } p { "Infrastructure toolkit" } } }
}
