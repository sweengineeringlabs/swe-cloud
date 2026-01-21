//! Auto-generated theme constants from theme.yaml
//! DO NOT EDIT - regenerate with `rsc build`

#![allow(dead_code)]

pub const DEFAULT_THEME: &str = "dark";

/// Provider brand colors
pub mod providers {
    /// AWS brand color
    pub const AWS_COLOR: &str = "#FF9900";
    pub const AWS_LABEL: &str = "AWS";
    pub const AWS_ICON: &str = "cloud";

    /// Azure brand color
    pub const AZURE_COLOR: &str = "#0078D4";
    pub const AZURE_LABEL: &str = "Azure";
    pub const AZURE_ICON: &str = "diamond";

    /// GCP brand color
    pub const GCP_COLOR: &str = "#4285F4";
    pub const GCP_LABEL: &str = "GCP";
    pub const GCP_ICON: &str = "circle";

    /// ZeroCloud brand color
    pub const ZEROCLOUD_COLOR: &str = "#6B7280";
    pub const ZEROCLOUD_LABEL: &str = "ZeroCloud";
    pub const ZEROCLOUD_ICON: &str = "circle-outline";

}

/// Environment colors
pub mod environments {
    /// Dev environment
    pub const DEV_COLOR: &str = "#3B82F6";
    pub const DEV_LABEL: &str = "Dev";
    pub const DEV_ICON: &str = "wrench";
    pub const DEV_WARNING: bool = false;

    /// Local environment
    pub const LOCAL_COLOR: &str = "#10B981";
    pub const LOCAL_LABEL: &str = "Local";
    pub const LOCAL_ICON: &str = "laptop";
    pub const LOCAL_WARNING: bool = false;

    /// Prod environment
    pub const PROD_COLOR: &str = "#EF4444";
    pub const PROD_LABEL: &str = "Prod";
    pub const PROD_ICON: &str = "rocket";
    pub const PROD_WARNING: bool = true;

    /// Staging environment
    pub const STAGING_COLOR: &str = "#F59E0B";
    pub const STAGING_LABEL: &str = "Staging";
    pub const STAGING_ICON: &str = "theater";
    pub const STAGING_WARNING: bool = false;

}

/// Spacing scale (in pixels)
pub mod spacing {
    pub const XS: i32 = 4;
    pub const SM: i32 = 8;
    pub const MD: i32 = 16;
    pub const LG: i32 = 24;
    pub const XL: i32 = 32;
    pub const SIZE_2XL: i32 = 48;
}

/// Border radius scale (in pixels)
pub mod radius {
    pub const SM: i32 = 4;
    pub const MD: i32 = 8;
    pub const LG: i32 = 12;
    pub const FULL: i32 = 9999;
}

/// Layout dimensions (in pixels)
pub mod layout {
    pub const CONTEXT_BAR_HEIGHT: i32 = 40;
    pub const HEADER_HEIGHT: i32 = 56;
    pub const SIDEBAR_COLLAPSED_WIDTH: i32 = 60;
    pub const SIDEBAR_WIDTH: i32 = 280;
    pub const STATUS_BAR_HEIGHT: i32 = 28;
}

/// Theme-specific colors
pub mod themes {
    /// Light theme colors
    pub mod light {
        pub const NAME: &str = "Light";

        pub const BG: &str = "#ffffff";
        pub const BG_SECONDARY: &str = "#f8fafc";
        pub const BG_TERTIARY: &str = "#f1f5f9";
        pub const TEXT: &str = "#0f172a";
        pub const TEXT_SECONDARY: &str = "#475569";
        pub const TEXT_MUTED: &str = "#94a3b8";
        pub const BORDER: &str = "#e2e8f0";
        pub const ACCENT: &str = "#2563eb";
        pub const SUCCESS: &str = "#059669";
        pub const WARNING: &str = "#d97706";
        pub const ERROR: &str = "#dc2626";
        pub const INFO: &str = "#2563eb";
    }

    /// Dark theme colors
    pub mod dark {
        pub const NAME: &str = "Dark";

        pub const BG: &str = "#0f172a";
        pub const BG_SECONDARY: &str = "#1e293b";
        pub const BG_TERTIARY: &str = "#334155";
        pub const TEXT: &str = "#f1f5f9";
        pub const TEXT_SECONDARY: &str = "#94a3b8";
        pub const TEXT_MUTED: &str = "#64748b";
        pub const BORDER: &str = "#334155";
        pub const ACCENT: &str = "#3b82f6";
        pub const SUCCESS: &str = "#10b981";
        pub const WARNING: &str = "#f59e0b";
        pub const ERROR: &str = "#ef4444";
        pub const INFO: &str = "#3b82f6";
    }

}
