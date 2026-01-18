// Navigation Module
// Header, context bar, and navigation components

// Re-export from component files
pub mod header;
pub mod context_bar;

pub use header::Header;
pub use context_bar::ContextBar;

use rsc::prelude::*;

/// Layout preset configuration
#[derive(Clone, Debug)]
pub struct LayoutPreset {
    pub sidebar: SidebarPreset,
    pub bottom: BottomPreset,
}

#[derive(Clone, Debug)]
pub struct SidebarPreset {
    pub width: Option<u32>,
    pub visible: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct BottomPreset {
    pub height: Option<u32>,
    pub visible: bool,
    pub tabs: Vec<String>,
}

/// Get layout preset for a feature
pub fn use_preset(feature: &str) -> LayoutPreset {
    match feature {
        "cloudemu" => LayoutPreset {
            sidebar: SidebarPreset { width: Some(280), visible: Some(true) },
            bottom: BottomPreset { height: Some(200), visible: true, tabs: vec!["Logs".to_string(), "Requests".to_string()] },
        },
        "cloudkit" => LayoutPreset {
            sidebar: SidebarPreset { width: Some(300), visible: Some(true) },
            bottom: BottomPreset { height: Some(180), visible: true, tabs: vec!["Output".to_string(), "History".to_string()] },
        },
        "iac" => LayoutPreset {
            sidebar: SidebarPreset { width: Some(260), visible: Some(true) },
            bottom: BottomPreset { height: Some(220), visible: true, tabs: vec!["Plan".to_string(), "State".to_string(), "Logs".to_string()] },
        },
        _ => LayoutPreset {
            sidebar: SidebarPreset { width: Some(280), visible: Some(true) },
            bottom: BottomPreset { height: Some(200), visible: false, tabs: vec![] },
        },
    }
}
