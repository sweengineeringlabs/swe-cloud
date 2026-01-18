use rsc::prelude::*;

#[page(route = "/iac/modules", title = "Modules")]
pub fn IacModules() -> Element {
    rsx! { div(class: "iac-modules") { h1 { "IAC Modules" } } }
}
