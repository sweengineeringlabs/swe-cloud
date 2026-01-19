// E2E tests for SWE Cloud UI layout rendering
//
// Tests full page layout with real browser rendering:
// - Main layout structure (ContextBar, Header, Sidebar, BottomPanel)
// - Component visibility and positioning
// - Feature layouts

use rsc_test::prelude::*;

const BASE_URL: &str = "http://localhost:3000";

// =============================================================================
// Layout Structure Tests
// =============================================================================

#[e2e]
async fn app_loads_with_complete_layout() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;

    // Wait for app to fully load
    page.wait_for("[data-testid='app']").await;

    // All major layout components should be present
    assert!(page.query("[data-testid='context-bar']").is_visible().await);
    assert!(page.query(".app-header").is_visible().await);
    assert!(page.query(".status-bar").is_visible().await);
}

#[e2e]
async fn context_bar_renders_at_top() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    let context_bar = page.query("[data-testid='context-bar']").await;
    let rect = context_bar.bounding_box().await;

    // Should be at top
    assert_eq!(rect.y, 0.0);

    // Should span full width
    let viewport = page.viewport_size().await;
    assert_eq!(rect.width, viewport.width);
}

#[e2e]
async fn header_renders_below_context_bar() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for(".app-header").await;

    let context_bar = page.query("[data-testid='context-bar']").await;
    let header = page.query(".app-header").await;

    let cb_rect = context_bar.bounding_box().await;
    let h_rect = header.bounding_box().await;

    // Header should be below context bar
    assert!((h_rect.y - (cb_rect.y + cb_rect.height)).abs() < 1.0);

    // Should span full width
    let viewport = page.viewport_size().await;
    assert_eq!(h_rect.width, viewport.width);
}

#[e2e]
async fn status_bar_renders_at_bottom() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for(".status-bar").await;

    let status_bar = page.query(".status-bar").await;
    let rect = status_bar.bounding_box().await;

    // Should be at bottom
    let viewport = page.viewport_size().await;
    assert!((rect.y + rect.height - viewport.height).abs() < 1.0);

    // Should span full width
    assert_eq!(rect.width, viewport.width);
}

#[e2e]
async fn sidebar_renders_when_open() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/cloudemu", BASE_URL)).await;
    page.wait_for("[data-testid='sidebar']").await;

    let sidebar = page.query("[data-testid='sidebar']").await;
    assert!(sidebar.is_visible().await);
}

#[e2e]
async fn bottom_panel_renders_when_open() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/cloudemu", BASE_URL)).await;
    page.wait_for("[data-testid='app']").await;

    // Open bottom panel via keyboard shortcut
    page.keyboard().press("Control+`").await;
    page.wait_for("[data-testid='bottom-panel']").await;

    let bottom_panel = page.query("[data-testid='bottom-panel']").await;
    assert!(bottom_panel.is_visible().await);

    // Should be above status bar
    let status_bar = page.query(".status-bar").await;
    let bp_rect = bottom_panel.bounding_box().await;
    let sb_rect = status_bar.bounding_box().await;

    assert!((bp_rect.y + bp_rect.height - sb_rect.y).abs() < 1.0);
}

// =============================================================================
// Main Content Area Tests
// =============================================================================

#[e2e]
async fn main_content_fills_available_space() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for(".workspace-main").await;

    let main = page.query(".workspace-main").await;
    let context_bar = page.query("[data-testid='context-bar']").await;
    let header = page.query(".app-header").await;
    let status_bar = page.query(".status-bar").await;

    let main_rect = main.bounding_box().await;
    let cb_rect = context_bar.bounding_box().await;
    let h_rect = header.bounding_box().await;
    let sb_rect = status_bar.bounding_box().await;

    // Main content should be below header
    assert!(main_rect.y >= cb_rect.height + h_rect.height);

    // Main content should be above status bar
    assert!(main_rect.y + main_rect.height <= sb_rect.y);
}

#[e2e]
async fn landing_page_renders_on_startup() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for(".landing-page").await;

    // Landing page should be visible
    assert!(page.query(".landing-page").is_visible().await);

    // Should have feature cards
    assert!(page.query(".feature-card, .features-grid").exists().await);
}

// =============================================================================
// Theme and Styling Tests
// =============================================================================

#[e2e]
async fn dark_theme_applies_correctly() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='app']").await;

    // App should have dark theme by default
    let app = page.query("[data-testid='app']").await;
    let bg_color = app.css_value("background-color").await;

    // Dark theme background should be dark (low luminance)
    assert!(is_dark_color(&bg_color));
}

#[e2e]
async fn css_variables_are_applied() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Check that CSS variables are being used
    let computed = page.evaluate(r#"
        getComputedStyle(document.documentElement).getPropertyValue('--primary-color')
    "#).await;

    assert!(!computed.is_empty());
}

// =============================================================================
// Feature Layout Tests
// =============================================================================

#[e2e]
async fn cloudemu_layout_renders() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/cloudemu", BASE_URL)).await;
    page.wait_for(".cloudemu-layout").await;

    assert!(page.query(".cloudemu-layout").is_visible().await);
    assert!(page.query("[data-testid='sidebar']").is_visible().await);
}

#[e2e]
async fn cloudkit_layout_renders() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/cloudkit", BASE_URL)).await;
    page.wait_for(".cloudkit-layout").await;

    assert!(page.query(".cloudkit-layout").is_visible().await);
}

#[e2e]
async fn iac_layout_renders() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/iac", BASE_URL)).await;
    page.wait_for(".iac-layout").await;

    assert!(page.query(".iac-layout").is_visible().await);
}

// =============================================================================
// Helper Functions
// =============================================================================

fn is_dark_color(color: &str) -> bool {
    if let Some(values) = parse_rgb(color) {
        let (r, g, b) = values;
        let luminance = 0.2126 * (r as f64 / 255.0)
            + 0.7152 * (g as f64 / 255.0)
            + 0.0722 * (b as f64 / 255.0);
        luminance < 0.5
    } else {
        false
    }
}

fn parse_rgb(color: &str) -> Option<(u8, u8, u8)> {
    let color = color.trim();
    if color.starts_with("rgb") {
        let start = color.find('(')?;
        let end = color.find(')')?;
        let values: Vec<u8> = color[start + 1..end]
            .split(',')
            .take(3)
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        if values.len() == 3 {
            return Some((values[0], values[1], values[2]));
        }
    }
    None
}
