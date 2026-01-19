use rsc::prelude::*;

#[page(route = "/cloudkit/explorer", title = "API Explorer")]
pub fn CloudkitExplorer() -> Element {
    rsx! { div(class: "cloudkit-explorer") { h1 { "API Explorer" } } }
}
