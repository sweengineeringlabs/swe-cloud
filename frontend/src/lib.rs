pub mod components;
pub mod pages;

use dioxus::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    Dashboard,
    #[route("/explorer")]
    Explorer,
    #[route("/providers")]
    Providers,
    #[route("/inspector")]
    Inspector,
    #[route("/settings")]
    Settings,
}

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Dashboard() -> Element {
    rsx! {
        pages::dashboard::DashboardPage {}
    }
}

#[component]
fn Explorer() -> Element {
    rsx! {
        pages::explorer::ExplorerPage {}
    }
}

#[component]
fn Providers() -> Element {
    rsx! {
        pages::providers::ProvidersPage {}
    }
}

#[component]
fn Inspector() -> Element {
    rsx! {
        pages::inspector::InspectorPage {}
    }
}

#[component]
fn Settings() -> Element {
    rsx! {
        pages::settings::SettingsPage {}
    }
}
