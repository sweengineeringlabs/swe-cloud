use rsc::prelude::*;

#[page(route = "/settings/users", title = "Users")]
pub fn SettingsUsers() -> Element {
    rsx! { div(class: "settings-users") { h1 { "Users" } } }
}
