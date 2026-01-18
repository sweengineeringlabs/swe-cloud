// CloudEmu Overview Page
use rsc::prelude::*;

#[page(route = "/cloudemu", title = "CloudEmu")]
pub fn CloudemuOverview() -> Element {
    rsx! {
        div(class: "cloudemu-overview") {
            h1 { "CloudEmu" }
            p { "Cloud service emulation for local development" }
        }
    }
}
