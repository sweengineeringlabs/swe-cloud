// Bottom Panel Component
// Resizable bottom panel with tabs

use rsc::prelude::*;

/// Bottom panel with tabs
#[component]
pub fn BottomPanel(
    height: Option<u32>,
    tabs: Vec<String>,
    children: Option<Children>,
) -> Element {
    let height = height.unwrap_or(200);
    let (active_tab, set_active_tab) = use_state(tabs.first().cloned().unwrap_or_default());
    let (is_expanded, set_expanded) = use_state(true);

    rsx! {
        div(
            class: format!("bottom-panel {}", if *is_expanded { "" } else { "collapsed" }),
            style: format!("--panel-height: {}px", height)
        ) {
            // Tab bar
            div(class: "panel-tabs") {
                for tab in &tabs {
                    button(
                        class: format!("panel-tab {}", if &*active_tab == tab { "active" } else { "" }),
                        onclick: move |_| {
                            set_active_tab(tab.clone());
                            set_expanded(true);
                        }
                    ) {
                        {tab}
                    }
                }

                // Toggle button
                button(
                    class: "panel-toggle",
                    onclick: move |_| set_expanded(!*is_expanded)
                ) {
                    if *is_expanded { "▼" } else { "▲" }
                }
            }

            // Tab content
            if *is_expanded {
                div(class: "panel-content") {
                    match active_tab.as_str() {
                        "logs" => rsx! { LogsPanel() },
                        "output" => rsx! { OutputPanel() },
                        "problems" => rsx! { ProblemsPanel() },
                        _ => rsx! { div { "Tab content" } }
                    }
                }
            }
        }
    }
}

#[component]
fn LogsPanel() -> Element {
    rsx! {
        div(class: "logs-panel") {
            pre(class: "log-output") {
                "[INFO] Server started on port 4566\n"
                "[INFO] AWS S3 emulator ready\n"
                "[INFO] AWS DynamoDB emulator ready\n"
            }
        }
    }
}

#[component]
fn OutputPanel() -> Element {
    rsx! {
        div(class: "output-panel") {
            pre(class: "output-content") {
                "Ready."
            }
        }
    }
}

#[component]
fn ProblemsPanel() -> Element {
    rsx! {
        div(class: "problems-panel") {
            p(class: "no-problems") { "No problems detected" }
        }
    }
}
