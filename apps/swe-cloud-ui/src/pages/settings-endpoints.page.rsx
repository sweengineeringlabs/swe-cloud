use rsc::prelude::*;

#[page(route = "/settings/endpoints", title = "Endpoints")]
pub fn SettingsEndpoints() -> Element {
    rsx! { div(class: "settings-endpoints") { h1 { "Endpoints" } } }
}
