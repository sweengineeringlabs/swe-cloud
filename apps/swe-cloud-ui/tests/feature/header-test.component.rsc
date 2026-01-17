// Unit tests for Header component
// Tests header rendering, search, notifications, user menu

use rsc_test::prelude::*;
use crate::modules::navigation::Header;

#[test]
fn test_header_renders() {
    let ctx = TestContext::new();
    let rendered = ctx.render_component::<Header>(Props::new());

    rendered.assert_exists();
    rendered.assert_has_element(".app-header");
}

#[test]
fn test_header_has_brand() {
    let ctx = TestContext::new();
    let rendered = ctx.render_component::<Header>(Props::new());

    rendered.assert_has_element(".brand");
    rendered.assert_text_contains("SWE Cloud");
}

#[test]
fn test_header_has_search() {
    let ctx = TestContext::new();
    let rendered = ctx.render_component::<Header>(Props::new());

    rendered.assert_has_element(".search-wrapper");
    rendered.assert_has_element("input[type='search']");
}

#[test]
fn test_header_has_notification_bell() {
    let ctx = TestContext::new();
    let rendered = ctx.render_component::<Header>(Props::new());

    rendered.assert_has_element(".notification-bell");
    rendered.assert_has_element(".bell-button");
}

#[test]
fn test_header_has_user_menu() {
    let ctx = TestContext::new();
    let rendered = ctx.render_component::<Header>(Props::new());

    rendered.assert_has_element(".user-menu");
    rendered.assert_has_element(".user-button");
}

#[test]
fn test_notification_dropdown_opens() {
    let ctx = TestContext::new();
    let rendered = ctx.render_component::<Header>(Props::new());

    let bell = rendered.query(".bell-button").first();
    bell.click();

    rendered.assert_has_element(".notification-dropdown");
}

#[test]
fn test_user_dropdown_opens() {
    let ctx = TestContext::new();
    let rendered = ctx.render_component::<Header>(Props::new());

    let user_btn = rendered.query(".user-button").first();
    user_btn.click();

    rendered.assert_has_element(".user-dropdown");
}

#[test]
fn test_notification_badge_shows_count() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("unread_notifications", 5i32);

    let rendered = ctx.render_component::<Header>(props);

    rendered.assert_has_element(".badge");
    rendered.assert_text_contains("5");
}

// =============================================================================
// Style Tests
// =============================================================================

#[test]
fn test_header_flex_layout() {
    let ctx = TestContext::new();
    let rendered = ctx.render_component::<Header>(Props::new());

    let header = rendered.query(".app-header").first();
    header.assert_style("display", "flex");
    header.assert_style("justify-content", "space-between");
    header.assert_style("align-items", "center");
}

#[test]
fn test_header_fixed_height() {
    let ctx = TestContext::new();
    let rendered = ctx.render_component::<Header>(Props::new());

    let header = rendered.query(".app-header").first();
    header.assert_has_style("height");
}
