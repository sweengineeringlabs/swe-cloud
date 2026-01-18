use rsc::prelude::*;

#[page(route = "/cloudemu/:provider", title = "Provider")]
pub fn CloudemuProvider() -> Element {
    rsx! { div(class: "cloudemu-provider") { h1 { "Provider" } } }
}
