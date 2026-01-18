// Sidebar Component
// Collapsible sidebar for navigation and panels

use rsc::prelude::*;

/// Sidebar component
#[component]
pub fn Sidebar(
    width: Option<u32>,
    visible: Option<bool>,
    children: Children,
) -> Element {
    let (collapsed, set_collapsed) = use_state(false);
    let width = width.unwrap_or(280);
    let visible = visible.unwrap_or(true);

    if !visible {
        return rsx! { div {} };
    }

    rsx! {
        aside(
            class: format!("sidebar {}", if *collapsed { "collapsed" } else { "" }),
            style: format!("--sidebar-width: {}px", width)
        ) {
            div(class: "sidebar-content") {
                {children}
            }

            button(
                class: "sidebar-toggle",
                onclick: move |_| set_collapsed(!*collapsed)
            ) {
                if *collapsed { "→" } else { "←" }
            }
        }
    }
}

/// Sidebar panel wrapper
#[component]
pub fn SidebarPanel(
    title: String,
    collapsible: Option<bool>,
    children: Children,
) -> Element {
    let collapsible = collapsible.unwrap_or(true);
    let (collapsed, set_collapsed) = use_state(false);

    rsx! {
        div(class: format!("sidebar-panel {}", if *collapsed { "collapsed" } else { "" })) {
            div(
                class: "panel-header",
                onclick: if collapsible { Some(move |_| set_collapsed(!*collapsed)) } else { None }
            ) {
                h3 { {&title} }
                if collapsible {
                    span(class: "collapse-icon") {
                        if *collapsed { "▶" } else { "▼" }
                    }
                }
            }
            if !*collapsed {
                div(class: "panel-content") {
                    {children}
                }
            }
        }
    }
}
