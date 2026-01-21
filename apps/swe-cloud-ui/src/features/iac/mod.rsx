// IAC Feature Module
// Infrastructure as Code modules and deployments

// Layout component (non-page)
pub mod iac_layout;
pub mod iac_type;

pub use iac_layout::IacLayout;
pub use iac_type::*;

// Note: .page.rsx files are auto-discovered by the router
// They don't need explicit module imports
