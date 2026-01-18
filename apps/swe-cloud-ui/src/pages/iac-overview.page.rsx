use rsc::prelude::*;

#[page(route = "/iac", title = "Infrastructure")]
pub fn IacOverview() -> Element {
    rsx! { div(class: "iac-overview") { h1 { "Infrastructure as Code" } } }
}
