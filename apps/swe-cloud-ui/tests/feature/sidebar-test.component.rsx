// Unit tests for Sidebar component
// Tests sidebar rendering, visibility, and panel switching

use rsc_test::prelude::*;
use crate::modules::layout::Sidebar;

#[test]
fn test_sidebar_renders() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", true);

    let rendered = ctx.render_component::<Sidebar>(props);

    rendered.assert_exists();
    rendered.assert_has_element("[data-testid='sidebar']");
}

#[test]
fn test_sidebar_hidden_when_closed() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", false);

    let rendered = ctx.render_component::<Sidebar>(props);

    let sidebar = rendered.query("[data-testid='sidebar']").first();
    sidebar.assert_hidden();
}

#[test]
fn test_sidebar_has_header() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", true);
    props.set("title", "Explorer".to_string());

    let rendered = ctx.render_component::<Sidebar>(props);

    rendered.assert_has_element(".sidebar-header");
    rendered.assert_text_contains("Explorer");
}

#[test]
fn test_sidebar_has_close_button() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", true);

    let rendered = ctx.render_component::<Sidebar>(props);

    rendered.assert_has_element(".sidebar-close");
}

#[test]
fn test_sidebar_visible_when_open() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", true);

    let rendered = ctx.render_component::<Sidebar>(props);

    let sidebar = rendered.query("[data-testid='sidebar']").first();
    sidebar.assert_visible();
}

#[test]
fn test_sidebar_content_area() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", true);

    let rendered = ctx.render_component::<Sidebar>(props);

    rendered.assert_has_element(".sidebar-content");
}

#[test]
fn test_sidebar_resizable() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", true);
    props.set("resizable", true);

    let rendered = ctx.render_component::<Sidebar>(props);

    rendered.assert_has_element(".resize-handle");
}

// =============================================================================
// Style Tests
// =============================================================================

#[test]
fn test_sidebar_width_style() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", true);
    props.set("width", 280i32);

    let rendered = ctx.render_component::<Sidebar>(props);

    let sidebar = rendered.query("[data-testid='sidebar']").first();
    sidebar.assert_style("width", "280px");
}

#[test]
fn test_sidebar_flex_layout() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", true);

    let rendered = ctx.render_component::<Sidebar>(props);

    let sidebar = rendered.query("[data-testid='sidebar']").first();
    sidebar.assert_style("display", "flex");
    sidebar.assert_style("flex-direction", "column");
}

#[test]
fn test_sidebar_overflow_style() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", true);

    let rendered = ctx.render_component::<Sidebar>(props);

    let content = rendered.query(".sidebar-content").first();
    content.assert_style("overflow", "auto");
}
