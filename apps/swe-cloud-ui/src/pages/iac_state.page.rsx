use rsc::prelude::*;

#[page(route = "/iac/state", title = "State")]
pub fn IacState() -> Element {
    rsx! { div(class: "iac-state") { h1 { "Infrastructure State" } } }
}
