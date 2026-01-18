// Environment Context
// Target environment selection (Local, Dev, Staging, Prod)

use rsc::prelude::*;

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
    vec![
        EnvironmentOption {
            id: "local".to_string(),
            label: "Local".to_string(),
            icon: "laptop".to_string(),
            color: "#10B981".to_string(),
            api_base: "http://localhost:8080".to_string(),
            confirm_actions: false,
            read_only_default: false,
        },
        EnvironmentOption {
            id: "dev".to_string(),
            label: "Development".to_string(),
            icon: "code".to_string(),
            color: "#3B82F6".to_string(),
            api_base: "https://dev-api.swe-cloud.io".to_string(),
            confirm_actions: false,
            read_only_default: false,
        },
        EnvironmentOption {
            id: "staging".to_string(),
            label: "Staging".to_string(),
            icon: "flask".to_string(),
            color: "#F59E0B".to_string(),
            api_base: "https://staging-api.swe-cloud.io".to_string(),
            confirm_actions: true,
            read_only_default: false,
        },
        EnvironmentOption {
            id: "prod".to_string(),
            label: "Production".to_string(),
            icon: "shield".to_string(),
            color: "#EF4444".to_string(),
            api_base: "https://api.swe-cloud.io".to_string(),
            confirm_actions: true,
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
