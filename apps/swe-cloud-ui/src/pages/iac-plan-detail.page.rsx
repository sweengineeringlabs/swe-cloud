use rsc::prelude::*;

#[page(route = "/iac/plans/:id", title = "Plan Detail")]
pub fn IacPlanDetail() -> Element {
    rsx! { div(class: "iac-plan-detail") { h1 { "Plan Detail" } } }
}
