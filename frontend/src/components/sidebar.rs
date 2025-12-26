use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        nav { class: "sidebar",
            div { class: "logo", "SWE-Cloud" }

            NavItem { to: Route::Dashboard, icon: "dashboard", label: "Dashboard" }
            NavItem { to: Route::Explorer, icon: "folder", label: "Explorer" }
            NavItem { to: Route::Providers, icon: "cloud", label: "Providers" }
            NavItem { to: Route::Inspector, icon: "search", label: "Inspector" }
            NavItem { to: Route::Settings, icon: "settings", label: "Settings" }
        }
    }
}

#[component]
fn NavItem(to: Route, icon: &'static str, label: &'static str) -> Element {
    rsx! {
        Link { to: to, class: "nav-item",
            span { class: "icon", "{icon}" }
            span { class: "label", "{label}" }
        }
    }
}
