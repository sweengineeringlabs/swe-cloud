// Context Module
// Exports all context providers and hooks

pub mod provider;
pub mod environment;

pub use provider::*;
pub use environment::*;

use rsc::prelude::*;

/// Combined application context provider
#[component]
pub fn AppContextProvider(children: Children) -> Element {
    rsx! {
        ProviderProvider {
            EnvironmentProvider {
                {children}
            }
        }
    }
}
