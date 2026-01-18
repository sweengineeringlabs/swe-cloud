use rsc::prelude::*;

#[page(route = "/settings/credentials", title = "Credentials")]
pub fn SettingsCredentials() -> Element {
    rsx! { div(class: "settings-credentials") { h1 { "Credentials" } } }
}
