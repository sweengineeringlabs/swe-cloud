// Not Found Page
use rsc::prelude::*;

#[page(route = "/*", title = "Not Found")]
pub fn NotFound() -> Element {
    rsx! {
        div(class: "not-found-page") {
            h1 { "404 - Not Found" }
            p { "The page you're looking for doesn't exist." }
            a(href: "/") { "Back to Dashboard" }
        }
    }
}
