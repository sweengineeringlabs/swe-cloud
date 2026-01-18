use rsc::prelude::*;

#[page(route = "/settings", title = "Settings")]
pub fn SettingsIndex() -> Element {
    rsx! { div(class: "settings-index") { h1 { "Settings" } } }
}
