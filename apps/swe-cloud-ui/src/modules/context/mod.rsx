// Context Module
// Exports all context providers and hooks

pub mod provider;
pub mod environment;
pub mod theme;

#[cfg(test)]
mod tests;

// Explicitly re-export all public items from submodules
pub use provider::{
    ProviderOption, ProviderEndpoints, ServiceOption,
    ProviderContext, ProviderProvider, use_provider
};
pub use environment::{
    EnvironmentOption, EnvironmentContext, EnvironmentProvider, use_environment
};
pub use theme::{
    ThemeMode, ThemeContext, ThemeProvider, ThemeToggle, ThemeSelector, use_theme
};

use rsc::prelude::*;

/// Combined application context provider
#[component]
pub fn AppContextProvider(children: Children) -> Element {
    rsx! {
        ThemeProvider {
            ProviderProvider {
                EnvironmentProvider {
                    RoleProvider {
                        {children}
                    }
                }
            }
        }
    }
}

// Role context for IAC permissions
#[context]
pub struct RoleContext {
    pub current: String,
    pub can_deploy: bool,
    pub can_approve: bool,
}

impl Default for RoleContext {
    fn default() -> Self {
        Self {
            current: "developer".to_string(),
            can_deploy: true,
            can_approve: false,
        }
    }
}

#[component]
pub fn RoleProvider(children: Children) -> Element {
    let context = use_context_state::<RoleContext>();
    rsx! {
        context_provider(value: context) { {children} }
    }
}

pub fn use_role() -> RoleContext {
    use_context::<RoleContext>()
}
