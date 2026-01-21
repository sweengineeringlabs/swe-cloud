// Context Bar Component Tests
// Tests for the provider and environment selector bar

use rsc::test::*;
use crate::modules::navigation::ContextBar;
use crate::modules::context::{ProviderContext, EnvironmentContext};

// ============================================================================
// CONTEXT BAR RENDER TESTS
// ============================================================================

#[test]
fn context_bar_renders_without_error() {
    let result = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    assert!(result.is_ok());
}

#[test]
fn context_bar_has_correct_class() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    assert!(rendered.query_selector(".context-bar").is_some());
}

#[test]
fn context_bar_has_left_and_right_sections() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    assert!(rendered.query_selector(".context-left").is_some());
    assert!(rendered.query_selector(".context-right").is_some());
}

// ============================================================================
// PROVIDER SELECTOR TESTS
// ============================================================================

#[test]
fn provider_selector_renders() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    assert!(rendered.query_selector(".provider-selector").is_some());
}

#[test]
fn provider_selector_shows_current_provider() {
    let rendered = render_with_context! {
        provider: ProviderContext { current: "aws".to_string(), ..Default::default() },
        ContextBar()
    };

    assert!(rendered.contains_text("AWS"));
}

#[test]
fn provider_selector_button_exists() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    assert!(rendered.query_selector(".selector-button").is_some());
}

#[test]
fn provider_selector_dropdown_hidden_by_default() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    assert!(rendered.query_selector(".selector-dropdown").is_none());
}

#[test]
fn provider_selector_dropdown_shows_on_click() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    // Simulate click on selector button
    rendered.fire_event(".selector-button", "click");

    assert!(rendered.query_selector(".selector-dropdown").is_some());
}

#[test]
fn provider_selector_dropdown_has_all_providers() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    rendered.fire_event(".selector-button", "click");

    let options = rendered.query_selector_all(".selector-option");
    assert!(options.len() >= 4); // AWS, Azure, GCP, ZeroCloud
}

#[test]
fn provider_selector_highlights_current_provider() {
    let rendered = render_with_context! {
        provider: ProviderContext { current: "azure".to_string(), ..Default::default() },
        ContextBar()
    };

    rendered.fire_event(".selector-button", "click");

    let active_option = rendered.query_selector(".selector-option.active");
    assert!(active_option.is_some());
    assert!(active_option.unwrap().contains_text("Azure"));
}

#[test]
fn provider_selector_changes_provider_on_selection() {
    let mut context = TestProviderContext::new("aws");

    let rendered = render_with_context!(context, ContextBar());

    rendered.fire_event(".selector-button", "click");

    // Find and click Azure option
    let options = rendered.query_selector_all(".selector-option");
    for option in options {
        if option.contains_text("Azure") {
            option.fire_event("click");
            break;
        }
    }

    // Verify context was updated
    assert_eq!(context.current(), "azure");
}

#[test]
fn provider_selector_dropdown_closes_after_selection() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    rendered.fire_event(".selector-button", "click");
    assert!(rendered.query_selector(".selector-dropdown").is_some());

    // Click an option
    rendered.fire_event(".selector-option", "click");

    // Dropdown should close
    assert!(rendered.query_selector(".selector-dropdown").is_none());
}

#[test]
fn provider_selector_shows_provider_icon() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    assert!(rendered.query_selector(".selector-icon").is_some());
}

#[test]
fn provider_selector_shows_dropdown_arrow() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    assert!(rendered.query_selector(".selector-arrow").is_some());
    assert!(rendered.contains_text("▼"));
}

// ============================================================================
// ENVIRONMENT SELECTOR TESTS
// ============================================================================

#[test]
fn environment_selector_renders() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    assert!(rendered.query_selector(".environment-selector").is_some());
}

#[test]
fn environment_selector_shows_pills() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    assert!(rendered.query_selector(".env-pills").is_some());
}

#[test]
fn environment_selector_has_all_environments() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    let pills = rendered.query_selector_all(".env-pill");
    assert!(pills.len() >= 4); // Local, Dev, Staging, Prod
}

#[test]
fn environment_selector_highlights_current_environment() {
    let rendered = render_with_context! {
        environment: EnvironmentContext { current: "local".to_string(), ..Default::default() },
        ContextBar()
    };

    let active_pill = rendered.query_selector(".env-pill.active");
    assert!(active_pill.is_some());
    assert!(active_pill.unwrap().contains_text("Local"));
}

#[test]
fn environment_selector_changes_environment_on_click() {
    let mut context = TestEnvironmentContext::new("local");

    let rendered = render_with_context!(context, ContextBar());

    // Find and click Dev pill
    let pills = rendered.query_selector_all(".env-pill");
    for pill in pills {
        if pill.contains_text("Dev") {
            pill.fire_event("click");
            break;
        }
    }

    assert_eq!(context.current(), "dev");
}

#[test]
fn environment_selector_shows_warning_for_production() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    let prod_pill = rendered.query_selector(".env-pill.production");
    assert!(prod_pill.is_some());

    // Should have warning icon
    let warning = prod_pill.unwrap().query_selector(".env-warning");
    assert!(warning.is_some());
}

#[test]
fn environment_selector_applies_environment_colors() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    let active_pill = rendered.query_selector(".env-pill.active");
    assert!(active_pill.is_some());

    // Should have --env-color CSS variable set
    let style = active_pill.unwrap().attribute("style");
    assert!(style.is_some());
    assert!(style.unwrap().contains("--env-color"));
}

// ============================================================================
// KEYBOARD NAVIGATION TESTS
// ============================================================================

#[test]
fn provider_selector_opens_on_enter_key() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    rendered.fire_event(".selector-button", "keydown", |e| e.key = "Enter");

    assert!(rendered.query_selector(".selector-dropdown").is_some());
}

#[test]
fn provider_selector_closes_on_escape() {
    let rendered = render! {
        TestContextProvider {
            ContextBar()
        }
    };

    rendered.fire_event(".selector-button", "click");
    assert!(rendered.query_selector(".selector-dropdown").is_some());

    rendered.fire_event_global("keydown", |e| e.key = "Escape");

    assert!(rendered.query_selector(".selector-dropdown").is_none());
}

// ============================================================================
// HELPER: Test Context Providers
// ============================================================================

#[component]
fn TestContextProvider(children: Children) -> Element {
    use crate::modules::context::AppContextProvider;

    rsx! {
        AppContextProvider {
            {children}
        }
    }
}

struct TestProviderContext {
    current: String,
}

impl TestProviderContext {
    fn new(provider: &str) -> Self {
        Self { current: provider.to_string() }
    }

    fn current(&self) -> &str {
        &self.current
    }
}

struct TestEnvironmentContext {
    current: String,
}

impl TestEnvironmentContext {
    fn new(env: &str) -> Self {
        Self { current: env.to_string() }
    }

    fn current(&self) -> &str {
        &self.current
    }
}
