// Header Component
// Main application header with branding, search, and user menu

use rsc::prelude::*;

/// Main header component
#[component]
pub fn Header() -> Element {
    rsx! {
        header(class: "app-header") {
            // Left section - Brand
            div(class: "header-left") {
                a(href: "/", class: "brand", data_testid: "brand-link") {
                    span(class: "brand-icon") { "☁" }
                    span(class: "brand-name") { "SWE Cloud" }
                }
            }

            // Center - Search
            div(class: "header-center") {
                SearchInput()
            }

            // Right section - Actions
            div(class: "header-right") {
                NotificationBell()
                UserMenu()
            }
        }
    }
}

/// Search input
#[component]
fn SearchInput() -> Element {
    let (search, set_search) = use_state(String::new());

    rsx! {
        div(class: "search-wrapper") {
            span(class: "search-icon") { "🔍" }
            input(
                type: "search",
                placeholder: "Search resources... (/)",
                value: "{search}",
                oninput: move |e| set_search(e.value.clone())
            )
            span(class: "search-shortcut") { "/" }
        }
    }
}

/// Notification bell
#[component]
fn NotificationBell() -> Element {
    let (unread, _) = use_state(3);
    let (open, set_open) = use_state(false);

    rsx! {
        div(class: "notification-bell") {
            button(class: "bell-button", onclick: move |_| set_open(!*open)) {
                span(class: "bell-icon") { "🔔" }
                if *unread > 0 {
                    span(class: "badge") { {unread.to_string()} }
                }
            }

            if *open {
                div(class: "notification-dropdown") {
                    div(class: "dropdown-header") {
                        h4 { "Notifications" }
                    }
                    div(class: "notification-list") {
                        div(class: "notification-item unread") {
                            strong { "Deployment completed" }
                            p { "vpc-module deployed successfully" }
                            span(class: "time") { "5 min ago" }
                        }
                    }
                    a(href: "/notifications", class: "view-all") { "View all" }
                }
            }
        }
    }
}

/// User menu
#[component]
fn UserMenu() -> Element {
    let (open, set_open) = use_state(false);

    rsx! {
        div(class: "user-menu") {
            button(class: "user-button", onclick: move |_| set_open(!*open)) {
                span(class: "user-avatar") { "👤" }
            }

            if *open {
                div(class: "user-dropdown") {
                    div(class: "user-info") {
                        span(class: "user-name") { "John Doe" }
                        span(class: "user-email") { "john@example.com" }
                    }
                    div(class: "dropdown-divider") {}
                    a(href: "/settings/profile") { "Profile" }
                    a(href: "/settings/preferences") { "Preferences" }
                    div(class: "dropdown-divider") {}
                    button(class: "logout-btn") { "Sign Out" }
                }
            }
        }
    }
}
