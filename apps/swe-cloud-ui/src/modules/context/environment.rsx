// Environment Context
// Target environment selection (Local, Dev, Staging, Prod)

use rsc::prelude::*;
use crate::generated::theme::environments;

/// Environment option from config
#[derive(Clone, Debug)]
pub struct EnvironmentOption {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub color: String,
    pub api_base: String,
    pub confirm_actions: bool,
    pub read_only_default: bool,
}

/// Environment context state
#[context(persist = true)]
pub struct EnvironmentContext {
    pub current: String,
    pub options: Vec<EnvironmentOption>,
}

impl Default for EnvironmentContext {
    fn default() -> Self {
        Self {
            current: "local".to_string(),
            options: load_environments_from_config(),
        }
    }
}

impl EnvironmentContext {
    pub fn current_environment(&self) -> Option<&EnvironmentOption> {
        self.options.iter().find(|e| e.id == self.current)
    }

    pub fn color(&self) -> Option<&str> {
        self.current_environment().map(|e| e.color.as_str())
    }

    pub fn api_base(&self) -> Option<&str> {
        self.current_environment().map(|e| e.api_base.as_str())
    }

    pub fn requires_confirmation(&self) -> bool {
        self.current_environment()
            .map(|e| e.confirm_actions)
            .unwrap_or(false)
    }

    pub fn is_read_only(&self) -> bool {
        self.current_environment()
            .map(|e| e.read_only_default)
            .unwrap_or(false)
    }

    pub fn is_production(&self) -> bool {
        self.current == "prod"
    }

    pub fn is_safe_environment(&self) -> bool {
        self.current == "local" || self.current == "dev"
    }

    pub fn switch(&mut self, env_id: &str) {
        if self.options.iter().any(|e| e.id == env_id) {
            self.current = env_id.to_string();
        }
    }
}

fn load_environments_from_config() -> Vec<EnvironmentOption> {
    // Use generated theme constants for colors
    vec![
        EnvironmentOption {
            id: "local".to_string(),
            label: environments::LOCAL_LABEL.to_string(),
            icon: environments::LOCAL_ICON.to_string(),
            color: environments::LOCAL_COLOR.to_string(),
            api_base: "http://localhost:8080".to_string(),
            confirm_actions: false,
            read_only_default: false,
        },
        EnvironmentOption {
            id: "dev".to_string(),
            label: environments::DEV_LABEL.to_string(),
            icon: environments::DEV_ICON.to_string(),
            color: environments::DEV_COLOR.to_string(),
            api_base: "https://dev-api.swe-cloud.io".to_string(),
            confirm_actions: false,
            read_only_default: false,
        },
        EnvironmentOption {
            id: "staging".to_string(),
            label: environments::STAGING_LABEL.to_string(),
            icon: environments::STAGING_ICON.to_string(),
            color: environments::STAGING_COLOR.to_string(),
            api_base: "https://staging-api.swe-cloud.io".to_string(),
            confirm_actions: true,
            read_only_default: false,
        },
        EnvironmentOption {
            id: "prod".to_string(),
            label: environments::PROD_LABEL.to_string(),
            icon: environments::PROD_ICON.to_string(),
            color: environments::PROD_COLOR.to_string(),
            api_base: "https://api.swe-cloud.io".to_string(),
            confirm_actions: environments::PROD_WARNING,
            read_only_default: true,
        },
    ]
}

#[component]
pub fn EnvironmentProvider(children: Children) -> Element {
    let context = use_context_state::<EnvironmentContext>();
    rsx! {
        context_provider(value: context) { {children} }
    }
}

pub fn use_environment() -> EnvironmentContext {
    use_context::<EnvironmentContext>()
}
