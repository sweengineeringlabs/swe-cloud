use rsc::prelude::*;

#[page(route = "/settings/providers", title = "Cloud Providers")]
pub fn SettingsProviders() -> Element {
    rsx! { div(class: "settings-providers") { h1 { "Cloud Providers" } } }
}
