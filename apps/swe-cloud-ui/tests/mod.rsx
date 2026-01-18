// SWE Cloud UI Test Suite
//
// This module organizes all tests for the SWE Cloud UI application.
//
// Test categories:
// - feature/: Feature tests for individual components
// - e2e/: End-to-end browser tests for full application flows
// - integration/: Integration tests for compilation and runtime

pub mod feature {
    pub mod sidebar_test;
    pub mod bottom_panel_test;
    pub mod context_bar_test;
    pub mod header_test;
    pub mod stat_card_test;
    pub mod workspace_layout_test;
    pub mod provider_card_test;
    pub mod request_table_test;
}

pub mod e2e {
    pub mod layout_e2e;
    pub mod navigation_e2e;
    pub mod routing_e2e;
    pub mod context_switching_e2e;
}

pub mod integration {
    pub mod compilation;
    pub mod context_providers;
    pub mod routing;
}
