use rsc::prelude::*;

#[page(route = "/iac/plans", title = "Plans")]
pub fn IacPlans() -> Element {
    rsx! { div(class: "iac-plans") { h1 { "IAC Plans" } } }
}
