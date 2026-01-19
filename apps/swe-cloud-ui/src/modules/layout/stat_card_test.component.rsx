// Unit tests for StatCard, FeatureCard, ActionCard components
// Tests card rendering and content display

use rsc_test::prelude::*;
use crate::modules::layout::{StatCard, FeatureCard, ActionCard, SectionHeader};

// =============================================================================
// StatCard Tests
// =============================================================================

#[test]
fn test_stat_card_renders() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("title", "Active Services".to_string());
    props.set("value", "12".to_string());

    let rendered = ctx.render_component::<StatCard>(props);

    rendered.assert_exists();
    rendered.assert_has_element(".stat-card");
}

#[test]
fn test_stat_card_shows_title() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("title", "Active Services".to_string());
    props.set("value", "12".to_string());

    let rendered = ctx.render_component::<StatCard>(props);

    rendered.assert_text_contains("Active Services");
}

#[test]
fn test_stat_card_shows_value() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("title", "Active Services".to_string());
    props.set("value", "12".to_string());

    let rendered = ctx.render_component::<StatCard>(props);

    rendered.assert_text_contains("12");
}

#[test]
fn test_stat_card_shows_icon() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("title", "Active Services".to_string());
    props.set("value", "12".to_string());
    props.set("icon", "cloud".to_string());

    let rendered = ctx.render_component::<StatCard>(props);

    rendered.assert_has_element(".stat-icon");
}

#[test]
fn test_stat_card_shows_trend() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("title", "Active Services".to_string());
    props.set("value", "12".to_string());
    props.set("trend", "+2".to_string());

    let rendered = ctx.render_component::<StatCard>(props);

    rendered.assert_has_element(".stat-trend");
    rendered.assert_text_contains("+2");
}

// =============================================================================
// FeatureCard Tests
// =============================================================================

#[test]
fn test_feature_card_renders() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("id", "cloudemu".to_string());
    props.set("title", "CloudEmu".to_string());
    props.set("description", "Cloud service emulation".to_string());

    let rendered = ctx.render_component::<FeatureCard>(props);

    rendered.assert_exists();
    rendered.assert_has_element(".feature-card");
}

#[test]
fn test_feature_card_has_link() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("id", "cloudemu".to_string());
    props.set("title", "CloudEmu".to_string());
    props.set("href", "/cloudemu".to_string());

    let rendered = ctx.render_component::<FeatureCard>(props);

    let link = rendered.query("a").first();
    link.assert_attribute("href", "/cloudemu");
}

#[test]
fn test_feature_card_shows_title() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("title", "CloudEmu".to_string());

    let rendered = ctx.render_component::<FeatureCard>(props);

    rendered.assert_text_contains("CloudEmu");
}

#[test]
fn test_feature_card_shows_description() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("title", "CloudEmu".to_string());
    props.set("description", "Cloud service emulation".to_string());

    let rendered = ctx.render_component::<FeatureCard>(props);

    rendered.assert_text_contains("Cloud service emulation");
}

// =============================================================================
// ActionCard Tests
// =============================================================================

#[test]
fn test_action_card_renders() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("title", "Create Resource".to_string());
    props.set("action", "create".to_string());

    let rendered = ctx.render_component::<ActionCard>(props);

    rendered.assert_exists();
    rendered.assert_has_element(".action-card");
}

#[test]
fn test_action_card_is_clickable() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("title", "Create Resource".to_string());

    let rendered = ctx.render_component::<ActionCard>(props);

    let card = rendered.query(".action-card").first();
    card.assert_has_attribute("onclick");
}

// =============================================================================
// SectionHeader Tests
// =============================================================================

#[test]
fn test_section_header_renders() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("title", "Overview".to_string());

    let rendered = ctx.render_component::<SectionHeader>(props);

    rendered.assert_exists();
    rendered.assert_has_element(".section-header");
}

#[test]
fn test_section_header_shows_title() {
    let ctx = TestContext::new();

    let mut props = Props::new();
    props.set("title", "Overview".to_string());

    let rendered = ctx.render_component::<SectionHeader>(props);

    rendered.assert_text_contains("Overview");
}
