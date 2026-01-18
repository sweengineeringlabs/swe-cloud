use rsc::prelude::*;

#[page(route = "/iac/deployments/:id", title = "Deployment Detail")]
pub fn IacDeploymentDetail() -> Element {
    rsx! { div(class: "iac-deployment-detail") { h1 { "Deployment Detail" } } }
}
