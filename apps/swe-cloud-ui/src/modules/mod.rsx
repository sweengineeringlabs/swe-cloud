// Modules Root
// Re-exports all module components and utilities

pub mod context;
pub mod layout;
pub mod navigation;

// Re-export commonly used items
pub use context::{
    AppContextProvider, use_provider, use_environment, use_role,
    use_theme, ThemeMode, ThemeToggle, ThemeSelector
};
pub use layout::{WorkspaceLayout, Sidebar, SidebarPanel, BottomPanel, StatCard, SectionHeader, ActionCard};
pub use navigation::{Header, ContextBar, use_preset};
