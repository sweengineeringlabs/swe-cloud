use rsc::prelude::*;

#[page(route = "/iac/deploy", title = "Deploy")]
pub fn IacDeploy() -> Element {
    rsx! { div(class: "iac-deploy") { h1 { "Deploy Infrastructure" } } }
}
