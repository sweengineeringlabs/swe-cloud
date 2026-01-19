use rsc::prelude::*;

#[page(route = "/iac/modules/:module", title = "Module Detail")]
pub fn IacModuleDetail() -> Element {
    rsx! { div(class: "iac-module-detail") { h1 { "Module Detail" } } }
}
