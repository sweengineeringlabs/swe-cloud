// CloudEmu Feature Module
// Cloud service emulation for local development

// Layout component (non-page)
pub mod cloudemu_layout;
pub mod cloudemu_type;
pub mod components;
pub mod hooks;

pub use cloudemu_layout::CloudemuLayout;
pub use cloudemu_type::*;

// Note: .page.rsx files are auto-discovered by the router
// They don't need explicit module imports
