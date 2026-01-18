// Dashboard Page
use rsc::prelude::*;

#[page(route = "/", title = "Dashboard")]
pub fn Dashboard() -> Element {
    rsx! {
        div(class: "dashboard-page") {
            h1 { "Welcome to SWE Cloud" }
            p { "Your cloud management dashboard" }
        }
    }
}
