// Unit tests for RequestTable component (CloudEmu feature)
// Tests request log table rendering and filtering

use rsc_test::prelude::*;
use crate::features::cloudemu::RequestTable;

#[test]
fn test_request_table_renders() {
    let ctx = TestContext::new();

    let rendered = ctx.render_component::<RequestTable>(Props::new());

    rendered.assert_exists();
    rendered.assert_has_element(".request-table");
}

#[test]
fn test_request_table_has_header() {
    let ctx = TestContext::new();

    let rendered = ctx.render_component::<RequestTable>(Props::new());

    rendered.assert_has_element("thead");
    rendered.assert_has_element("th");
}

#[test]
fn test_request_table_columns() {
    let ctx = TestContext::new();

    let rendered = ctx.render_component::<RequestTable>(Props::new());

    // Check expected column headers
    rendered.assert_text_contains("Method");
    rendered.assert_text_contains("Path");
    rendered.assert_text_contains("Status");
    rendered.assert_text_contains("Service");
    rendered.assert_text_contains("Time");
}

#[test]
fn test_request_table_shows_rows() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("requests", vec![
        RequestLog { id: "1", method: "GET", path: "/test", status: 200 },
        RequestLog { id: "2", method: "POST", path: "/data", status: 201 },
    ]);

    let rendered = ctx.render_component::<RequestTable>(props);

    let rows = rendered.query_all("tbody tr");
    assert_eq!(rows.len(), 2);
}

#[test]
fn test_request_table_method_badge() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("requests", vec![
        RequestLog { id: "1", method: "GET", path: "/test", status: 200 },
    ]);

    let rendered = ctx.render_component::<RequestTable>(props);

    rendered.assert_has_element(".method-badge");
    rendered.assert_has_element(".method-get");
}

#[test]
fn test_request_table_status_colors() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("requests", vec![
        RequestLog { id: "1", method: "GET", path: "/test", status: 200 },
        RequestLog { id: "2", method: "GET", path: "/error", status: 500 },
    ]);

    let rendered = ctx.render_component::<RequestTable>(props);

    rendered.assert_has_element(".status-success");
    rendered.assert_has_element(".status-error");
}

#[test]
fn test_request_table_empty_state() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("requests", Vec::<RequestLog>::new());

    let rendered = ctx.render_component::<RequestTable>(props);

    rendered.assert_has_element(".empty-state");
    rendered.assert_text_contains("No requests");
}

#[test]
fn test_request_table_filtering() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("filter", "GET".to_string());

    let rendered = ctx.render_component::<RequestTable>(props);

    rendered.assert_has_element(".filter-input");
}

#[test]
fn test_request_table_row_clickable() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("requests", vec![
        RequestLog { id: "1", method: "GET", path: "/test", status: 200 },
    ]);

    let rendered = ctx.render_component::<RequestTable>(props);

    let row = rendered.query("tbody tr").first();
    row.assert_has_attribute("onclick");
}

// =============================================================================
// Style Tests
// =============================================================================

#[test]
fn test_request_table_striped_rows() {
    let ctx = TestContext::new();

    let rendered = ctx.render_component::<RequestTable>(Props::new());

    let table = rendered.query(".request-table").first();
    table.assert_has_class("striped");
}

#[test]
fn test_request_table_hover_highlight() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("requests", vec![
        RequestLog { id: "1", method: "GET", path: "/test", status: 200 },
    ]);

    let rendered = ctx.render_component::<RequestTable>(props);

    let row = rendered.query("tbody tr").first();
    row.assert_has_style("cursor");
}

// Helper type for tests
struct RequestLog {
    id: &'static str,
    method: &'static str,
    path: &'static str,
    status: i32,
}
