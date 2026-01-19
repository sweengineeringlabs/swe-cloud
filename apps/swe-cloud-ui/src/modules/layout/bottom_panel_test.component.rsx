// Unit tests for BottomPanel component
// Tests bottom panel rendering, tabs, and visibility

use rsc_test::prelude::*;
use crate::modules::layout::BottomPanel;

#[test]
fn test_bottom_panel_renders() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", true);

    let rendered = ctx.render_component::<BottomPanel>(props);

    rendered.assert_exists();
    rendered.assert_has_element("[data-testid='bottom-panel']");
}

#[test]
fn test_bottom_panel_hidden_when_closed() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", false);

    let rendered = ctx.render_component::<BottomPanel>(props);

    let panel = rendered.query("[data-testid='bottom-panel']").first();
    panel.assert_hidden();
}

#[test]
fn test_bottom_panel_has_tabs() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", true);
    props.set("tabs", vec!["Console", "Output", "Problems"]);

    let rendered = ctx.render_component::<BottomPanel>(props);

    rendered.assert_has_element(".panel-tabs");
    let tabs = rendered.query_all(".panel-tab");
    assert!(tabs.len() >= 3);
}

#[test]
fn test_bottom_panel_tab_switching() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", true);
    props.set("tabs", vec!["Console", "Output", "Problems"]);
    props.set("active_tab", "Console".to_string());

    let rendered = ctx.render_component::<BottomPanel>(props);

    let console_tab = rendered.query("[data-tab='Console']").first();
    console_tab.assert_has_class("active");
}

#[test]
fn test_bottom_panel_content_area() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", true);

    let rendered = ctx.render_component::<BottomPanel>(props);

    rendered.assert_has_element(".panel-content");
}

#[test]
fn test_bottom_panel_resizable() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", true);
    props.set("resizable", true);

    let rendered = ctx.render_component::<BottomPanel>(props);

    rendered.assert_has_element(".resize-handle-top");
}

// =============================================================================
// Style Tests
// =============================================================================

#[test]
fn test_bottom_panel_height_style() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", true);
    props.set("height", 200i32);

    let rendered = ctx.render_component::<BottomPanel>(props);

    let panel = rendered.query("[data-testid='bottom-panel']").first();
    panel.assert_style("height", "200px");
}

#[test]
fn test_bottom_panel_flex_layout() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("is_open", true);

    let rendered = ctx.render_component::<BottomPanel>(props);

    let panel = rendered.query("[data-testid='bottom-panel']").first();
    panel.assert_style("display", "flex");
    panel.assert_style("flex-direction", "column");
}
