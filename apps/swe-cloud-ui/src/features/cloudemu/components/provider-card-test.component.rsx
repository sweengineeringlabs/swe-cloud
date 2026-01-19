// Unit tests for ProviderCard component (CloudEmu feature)
// Tests provider card rendering and interactions

use rsc_test::prelude::*;
use crate::features::cloudemu::ProviderCard;

#[test]
fn test_provider_card_renders() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("id", "aws".to_string());
    props.set("name", "AWS".to_string());
    props.set("icon", "aws".to_string());

    let rendered = ctx.render_component::<ProviderCard>(props);

    rendered.assert_exists();
    rendered.assert_has_element(".provider-card");
}

#[test]
fn test_provider_card_shows_name() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("id", "aws".to_string());
    props.set("name", "AWS".to_string());

    let rendered = ctx.render_component::<ProviderCard>(props);

    rendered.assert_text_contains("AWS");
}

#[test]
fn test_provider_card_shows_icon() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("id", "aws".to_string());
    props.set("name", "AWS".to_string());
    props.set("icon", "aws".to_string());

    let rendered = ctx.render_component::<ProviderCard>(props);

    rendered.assert_has_element(".provider-icon");
}

#[test]
fn test_provider_card_shows_service_count() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("id", "aws".to_string());
    props.set("name", "AWS".to_string());
    props.set("service_count", 15i32);

    let rendered = ctx.render_component::<ProviderCard>(props);

    rendered.assert_text_contains("15");
    rendered.assert_text_contains("services");
}

#[test]
fn test_provider_card_shows_status() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("id", "aws".to_string());
    props.set("name", "AWS".to_string());
    props.set("status", "running".to_string());

    let rendered = ctx.render_component::<ProviderCard>(props);

    rendered.assert_has_element(".provider-status");
    rendered.assert_has_element(".status-running");
}

#[test]
fn test_provider_card_active_state() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("id", "aws".to_string());
    props.set("name", "AWS".to_string());
    props.set("active", true);

    let rendered = ctx.render_component::<ProviderCard>(props);

    let card = rendered.query(".provider-card").first();
    card.assert_has_class("active");
}

#[test]
fn test_provider_card_clickable() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("id", "aws".to_string());
    props.set("name", "AWS".to_string());

    let rendered = ctx.render_component::<ProviderCard>(props);

    let card = rendered.query(".provider-card").first();
    card.assert_has_attribute("onclick");
}

// =============================================================================
// Style Tests
// =============================================================================

#[test]
fn test_provider_card_hover_effect() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("id", "aws".to_string());
    props.set("name", "AWS".to_string());

    let rendered = ctx.render_component::<ProviderCard>(props);

    let card = rendered.query(".provider-card").first();
    card.assert_has_style("cursor");
}

#[test]
fn test_provider_card_border_radius() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("id", "aws".to_string());
    props.set("name", "AWS".to_string());

    let rendered = ctx.render_component::<ProviderCard>(props);

    let card = rendered.query(".provider-card").first();
    card.assert_has_style("border-radius");
}
