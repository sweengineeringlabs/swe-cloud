use dioxus::prelude::*;

#[component]
pub fn SettingsPage() -> Element {
    rsx! {
        div { class: "page settings",
            header { class: "page-header",
                h1 { "Settings" }
            }

            section { class: "settings-section",
                h2 { "CloudEmu Configuration" }

                div { class: "setting-group",
                    label { "Default Port Range" }
                    input { r#type: "text", value: "4566-4600" }
                }

                div { class: "setting-group",
                    label { "Data Directory" }
                    input { r#type: "text", value: "~/.cloudemu/data" }
                }

                div { class: "setting-group",
                    label { "Log Level" }
                    select {
                        option { "Info" }
                        option { "Debug" }
                        option { "Warn" }
                        option { "Error" }
                    }
                }
            }

            section { class: "settings-section",
                h2 { "UI Preferences" }

                div { class: "setting-group",
                    label { "Theme" }
                    select {
                        option { "Dark" }
                        option { "Light" }
                        option { "System" }
                    }
                }

                div { class: "setting-group",
                    label { "Auto-refresh interval" }
                    select {
                        option { "1 second" }
                        option { "5 seconds" }
                        option { "10 seconds" }
                        option { "Manual" }
                    }
                }
            }

            div { class: "settings-actions",
                button { class: "btn-primary", "Save Settings" }
                button { class: "btn-secondary", "Reset to Defaults" }
            }
        }
    }
}
