use rsc::prelude::*;

#[page(route = "/iac/deployments", title = "Deployments")]
pub fn IacDeployments() -> Element {
    rsx! { div(class: "iac-deployments") { h1 { "Deployments" } } }
}
