// Provider Context
// Cloud provider selection (AWS, Azure, GCP, Zero)

use rsc::prelude::*;

/// Provider option from config
#[derive(Clone, Debug)]
pub struct ProviderOption {
    pub id: String,
    pub label: String,
    pub icon: String,
    pub color: String,
    pub endpoints: ProviderEndpoints,
    pub services: Vec<ServiceOption>,
}

#[derive(Clone, Debug)]
pub struct ProviderEndpoints {
    pub api: String,
    pub console: String,
}

#[derive(Clone, Debug)]
pub struct ServiceOption {
    pub id: String,
    pub label: String,
    pub icon: String,
}

/// Provider context state
#[context(persist = true)]
pub struct ProviderContext {
    pub current: String,
    pub options: Vec<ProviderOption>,
}

impl Default for ProviderContext {
    fn default() -> Self {
        Self {
            current: "aws".to_string(),
            options: load_providers_from_config(),
        }
    }
}

impl ProviderContext {
    pub fn current_provider(&self) -> Option<&ProviderOption> {
        self.options.iter().find(|p| p.id == self.current)
    }

    pub fn color(&self) -> Option<&str> {
        self.current_provider().map(|p| p.color.as_str())
    }

    pub fn api_endpoint(&self) -> Option<&str> {
        self.current_provider().map(|p| p.endpoints.api.as_str())
    }

    pub fn services(&self) -> Vec<&ServiceOption> {
        self.current_provider()
            .map(|p| p.services.iter().collect())
            .unwrap_or_default()
    }

    pub fn switch(&mut self, provider_id: &str) {
        if self.options.iter().any(|p| p.id == provider_id) {
            self.current = provider_id.to_string();
        }
    }

    pub fn is_multi_cloud(&self) -> bool {
        self.current == "multi"
    }
}

fn load_providers_from_config() -> Vec<ProviderOption> {
    // Load from configs/providers.yaml at runtime
    // For now, return defaults
    vec![
        ProviderOption {
            id: "aws".to_string(),
            label: "AWS".to_string(),
            icon: "aws".to_string(),
            color: "#FF9900".to_string(),
            endpoints: ProviderEndpoints {
                api: "http://localhost:4566".to_string(),
                console: "/cloudemu/aws".to_string(),
            },
            services: vec![
                ServiceOption { id: "s3".to_string(), label: "S3".to_string(), icon: "bucket".to_string() },
                ServiceOption { id: "dynamodb".to_string(), label: "DynamoDB".to_string(), icon: "database".to_string() },
                ServiceOption { id: "lambda".to_string(), label: "Lambda".to_string(), icon: "function".to_string() },
            ],
        },
        ProviderOption {
            id: "azure".to_string(),
            label: "Azure".to_string(),
            icon: "azure".to_string(),
            color: "#0078D4".to_string(),
            endpoints: ProviderEndpoints {
                api: "http://localhost:4567".to_string(),
                console: "/cloudemu/azure".to_string(),
            },
            services: vec![
                ServiceOption { id: "blobs".to_string(), label: "Blob Storage".to_string(), icon: "bucket".to_string() },
                ServiceOption { id: "functions".to_string(), label: "Functions".to_string(), icon: "function".to_string() },
            ],
        },
        ProviderOption {
            id: "gcp".to_string(),
            label: "GCP".to_string(),
            icon: "gcp".to_string(),
            color: "#4285F4".to_string(),
            endpoints: ProviderEndpoints {
                api: "http://localhost:4568".to_string(),
                console: "/cloudemu/gcp".to_string(),
            },
            services: vec![
                ServiceOption { id: "storage".to_string(), label: "Cloud Storage".to_string(), icon: "bucket".to_string() },
                ServiceOption { id: "functions".to_string(), label: "Cloud Functions".to_string(), icon: "function".to_string() },
            ],
        },
        ProviderOption {
            id: "zero".to_string(),
            label: "ZeroCloud".to_string(),
            icon: "server".to_string(),
            color: "#6B7280".to_string(),
            endpoints: ProviderEndpoints {
                api: "http://localhost:4569".to_string(),
                console: "/cloudemu/zero".to_string(),
            },
            services: vec![
                ServiceOption { id: "storage".to_string(), label: "Storage".to_string(), icon: "bucket".to_string() },
                ServiceOption { id: "compute".to_string(), label: "Compute".to_string(), icon: "server".to_string() },
            ],
        },
    ]
}

#[component]
pub fn ProviderProvider(children: Children) -> Element {
    let context = use_context_state::<ProviderContext>();
    rsx! {
        context_provider(value: context) { {children} }
    }
}

pub fn use_provider() -> ProviderContext {
    use_context::<ProviderContext>()
}
