// Layout Module
// Re-exports all layout components

// RustScript maps foo.component.rsc -> foo module, foo-bar.component.rsc -> foo_bar module
pub mod workspace_layout;
pub mod sidebar;
pub mod bottom_panel;
pub mod stat_card;

// Re-export components
pub use workspace_layout::WorkspaceLayout;
pub use sidebar::{Sidebar, SidebarPanel};
pub use bottom_panel::BottomPanel;
pub use stat_card::{StatCard, SectionHeader, FeatureCard, ActionCard};
