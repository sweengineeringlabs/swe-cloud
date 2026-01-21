// Theme Context
// Theme selection (dark, light, system)

use rsc::prelude::*;

/// Theme mode options
#[derive(Clone, Debug, PartialEq)]
pub enum ThemeMode {
    Dark,
    Light,
    System,
}

impl ThemeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeMode::Dark => "dark",
            ThemeMode::Light => "light",
            ThemeMode::System => "system",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "light" => ThemeMode::Light,
            "system" => ThemeMode::System,
            _ => ThemeMode::Dark,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ThemeMode::Dark => "Dark",
            ThemeMode::Light => "Light",
            ThemeMode::System => "System",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ThemeMode::Dark => "moon",
            ThemeMode::Light => "sun",
            ThemeMode::System => "monitor",
        }
    }
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::Dark
    }
}

/// Theme context state
#[context(persist = true)]
pub struct ThemeContext {
    pub mode: ThemeMode,
}

impl Default for ThemeContext {
    fn default() -> Self {
        // Try to load from localStorage, fallback to config default
        Self {
            mode: ThemeMode::Dark,
        }
    }
}

impl ThemeContext {
    pub fn is_dark(&self) -> bool {
        matches!(self.mode, ThemeMode::Dark)
    }

    pub fn is_light(&self) -> bool {
        matches!(self.mode, ThemeMode::Light)
    }

    pub fn is_system(&self) -> bool {
        matches!(self.mode, ThemeMode::System)
    }

    pub fn set_mode(&mut self, mode: ThemeMode) {
        self.mode = mode;
    }

    pub fn toggle(&mut self) {
        self.mode = match self.mode {
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::System,
            ThemeMode::System => ThemeMode::Dark,
        };
    }

    pub fn cycle_next(&mut self) {
        self.toggle();
    }

    /// Get the data-theme attribute value
    pub fn data_theme(&self) -> &'static str {
        self.mode.as_str()
    }
}

/// Theme provider component that applies theme to document
#[component]
pub fn ThemeProvider(children: Children) -> Element {
    let context = use_context_state::<ThemeContext>();
    let theme_attr = context.data_theme();

    rsx! {
        context_provider(value: context) {
            div(data_theme: theme_attr, class: "theme-root") {
                {children}
            }
        }
    }
}

/// Hook to access theme context
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>()
}

/// Theme toggle button component
#[component]
pub fn ThemeToggle() -> Element {
    let (theme, set_theme) = use_context_state::<ThemeContext>();

    let icon = match theme.mode {
        ThemeMode::Dark => "moon-icon",
        ThemeMode::Light => "sun-icon",
        ThemeMode::System => "monitor-icon",
    };

    let next_mode = match theme.mode {
        ThemeMode::Dark => ThemeMode::Light,
        ThemeMode::Light => ThemeMode::System,
        ThemeMode::System => ThemeMode::Dark,
    };

    let tooltip = format!("Switch to {} mode", next_mode.label());

    rsx! {
        button(
            class: "theme-toggle",
            data_testid: "theme-toggle",
            title: tooltip,
            onclick: move |_| {
                let mut ctx = theme.clone();
                ctx.toggle();
                set_theme(ctx);
            }
        ) {
            span(class: format!("theme-icon {}", icon), data_testid: "theme-icon") {
                {match theme.mode {
                    ThemeMode::Dark => "moon-symbol",
                    ThemeMode::Light => "sun-symbol",
                    ThemeMode::System => "monitor-symbol",
                }}
            }
        }
    }
}

/// Theme selector dropdown component
#[component]
pub fn ThemeSelector() -> Element {
    let (theme, set_theme) = use_context_state::<ThemeContext>();
    let (dropdown_open, set_dropdown_open) = use_state(false);

    let options = vec![
        ThemeMode::Dark,
        ThemeMode::Light,
        ThemeMode::System,
    ];

    rsx! {
        div(class: "theme-selector", data_testid: "theme-selector") {
            button(
                class: "selector-button",
                data_testid: "theme-button",
                onclick: move |_| set_dropdown_open(!*dropdown_open)
            ) {
                span(class: "selector-icon") {
                    {match theme.mode {
                        ThemeMode::Dark => "dark-icon",
                        ThemeMode::Light => "light-icon",
                        ThemeMode::System => "system-icon",
                    }}
                }
                span(class: "selector-label", data_testid: "theme-label") {
                    {theme.mode.label()}
                }
                span(class: "selector-arrow") { "arrow-down" }
            }

            if *dropdown_open {
                div(class: "selector-dropdown", data_testid: "theme-dropdown") {
                    for option in options {
                        button(
                            class: format!("selector-option {}", if theme.mode == option { "active" } else { "" }),
                            data_testid: format!("theme-option-{}", option.as_str()),
                            onclick: move |_| {
                                let mut ctx = theme.clone();
                                ctx.set_mode(option.clone());
                                set_theme(ctx);
                                set_dropdown_open(false);
                            }
                        ) {
                            span(class: "option-icon") {
                                {match option {
                                    ThemeMode::Dark => "dark-icon",
                                    ThemeMode::Light => "light-icon",
                                    ThemeMode::System => "system-icon",
                                }}
                            }
                            span(class: "option-label") { {option.label()} }
                        }
                    }
                }
            }
        }
    }
}
