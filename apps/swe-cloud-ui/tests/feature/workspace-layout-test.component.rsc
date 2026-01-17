// Unit tests for WorkspaceLayout component
// Tests main workspace layout structure

use rsc_test::prelude::*;
use crate::modules::layout::WorkspaceLayout;

#[test]
fn test_workspace_layout_renders() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("feature", "dashboard".to_string());

    let rendered = ctx.render_component::<WorkspaceLayout>(props);

    rendered.assert_exists();
    rendered.assert_has_element(".workspace-layout");
}

#[test]
fn test_workspace_layout_has_context_bar() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("feature", "dashboard".to_string());

    let rendered = ctx.render_component::<WorkspaceLayout>(props);

    rendered.assert_has_element("[data-testid='context-bar']");
}

#[test]
fn test_workspace_layout_has_header() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("feature", "dashboard".to_string());

    let rendered = ctx.render_component::<WorkspaceLayout>(props);

    rendered.assert_has_element(".app-header");
}

#[test]
fn test_workspace_layout_has_main_area() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("feature", "dashboard".to_string());

    let rendered = ctx.render_component::<WorkspaceLayout>(props);

    rendered.assert_has_element(".workspace-main");
}

#[test]
fn test_workspace_layout_has_status_bar() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("feature", "dashboard".to_string());

    let rendered = ctx.render_component::<WorkspaceLayout>(props);

    rendered.assert_has_element(".status-bar");
}

#[test]
fn test_workspace_layout_sidebar_when_enabled() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("feature", "cloudemu".to_string());
    props.set("sidebar_open", true);

    let rendered = ctx.render_component::<WorkspaceLayout>(props);

    rendered.assert_has_element("[data-testid='sidebar']");
}

#[test]
fn test_workspace_layout_bottom_panel_when_enabled() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("feature", "cloudemu".to_string());
    props.set("bottom_panel_open", true);

    let rendered = ctx.render_component::<WorkspaceLayout>(props);

    rendered.assert_has_element("[data-testid='bottom-panel']");
}

#[test]
fn test_workspace_layout_renders_children() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("feature", "dashboard".to_string());

    let rendered = ctx.render_component_with_children::<WorkspaceLayout>(
        props,
        rsx! { div(class: "test-child") { "Test Content" } }
    );

    rendered.assert_has_element(".test-child");
    rendered.assert_text_contains("Test Content");
}

// =============================================================================
// Style Tests
// =============================================================================

#[test]
fn test_workspace_layout_grid_structure() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("feature", "dashboard".to_string());

    let rendered = ctx.render_component::<WorkspaceLayout>(props);

    let layout = rendered.query(".workspace-layout").first();
    layout.assert_style("display", "grid");
}

#[test]
fn test_workspace_layout_full_height() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("feature", "dashboard".to_string());

    let rendered = ctx.render_component::<WorkspaceLayout>(props);

    let layout = rendered.query(".workspace-layout").first();
    layout.assert_style("height", "100vh");
}

#[test]
fn test_main_area_overflow() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("feature", "dashboard".to_string());

    let rendered = ctx.render_component::<WorkspaceLayout>(props);

    let main = rendered.query(".workspace-main").first();
    main.assert_style("overflow", "auto");
}
