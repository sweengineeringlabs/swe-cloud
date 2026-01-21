// CloudKit Feature Module
// Cloud resource exploration and management

// Layout component (non-page)
pub mod cloudkit_layout;
pub mod cloudkit_type;

pub use cloudkit_layout::CloudkitLayout;
pub use cloudkit_type::*;

// Note: .page.rsx files are auto-discovered by the router
// They don't need explicit module imports
