// Unit tests for ContextBar component
// Tests provider/environment switcher

use rsc_test::prelude::*;
use crate::modules::navigation::ContextBar;

#[test]
fn test_context_bar_renders() {
    let ctx = TestContext::new();
    let rendered = ctx.render_component::<ContextBar>(Props::new());

    rendered.assert_exists();
    rendered.assert_has_element("[data-testid='context-bar']");
}

#[test]
fn test_context_bar_has_provider_section() {
    let ctx = TestContext::new();
    let rendered = ctx.render_component::<ContextBar>(Props::new());

    rendered.assert_has_element(".provider-section");
}

#[test]
fn test_context_bar_has_environment_section() {
    let ctx = TestContext::new();
    let rendered = ctx.render_component::<ContextBar>(Props::new());

    rendered.assert_has_element(".environment-section");
}

#[test]
fn test_context_bar_provider_dropdown() {
    let ctx = TestContext::new();
    let rendered = ctx.render_component::<ContextBar>(Props::new());

    // Click provider selector
    let selector = rendered.query(".provider-selector").first();
    selector.click();

    // Dropdown should appear
    rendered.assert_has_element(".provider-dropdown");
}

#[test]
fn test_context_bar_environment_dropdown() {
    let ctx = TestContext::new();
    let rendered = ctx.render_component::<ContextBar>(Props::new());

    // Click environment selector
    let selector = rendered.query(".environment-selector").first();
    selector.click();

    // Dropdown should appear
    rendered.assert_has_element(".environment-dropdown");
}

#[test]
fn test_context_bar_shows_current_provider() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("provider", "aws".to_string());

    let rendered = ctx.render_component::<ContextBar>(props);

    rendered.assert_text_contains("AWS");
}

#[test]
fn test_context_bar_shows_current_environment() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("environment", "local".to_string());

    let rendered = ctx.render_component::<ContextBar>(props);

    rendered.assert_text_contains("Local");
}

#[test]
fn test_context_bar_environment_color_indicator() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("environment", "prod".to_string());

    let rendered = ctx.render_component::<ContextBar>(props);

    // Production should show red indicator
    let indicator = rendered.query(".environment-indicator").first();
    indicator.assert_has_style("background-color");
}

// =============================================================================
// Style Tests
// =============================================================================

#[test]
fn test_context_bar_flex_layout() {
    let ctx = TestContext::new();
    let rendered = ctx.render_component::<ContextBar>(Props::new());

    let bar = rendered.query("[data-testid='context-bar']").first();
    bar.assert_style("display", "flex");
    bar.assert_style("align-items", "center");
}

#[test]
fn test_context_bar_gap_between_sections() {
    let ctx = TestContext::new();
    let rendered = ctx.render_component::<ContextBar>(Props::new());

    let bar = rendered.query("[data-testid='context-bar']").first();
    bar.assert_has_style("gap");
}
