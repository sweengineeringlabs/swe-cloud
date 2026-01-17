// E2E tests for SWE Cloud UI navigation
//
// Tests navigation flows and feature transitions:
// - Feature card navigation
// - Header navigation
// - Breadcrumb navigation
// - Back/forward browser navigation

use rsc_test::prelude::*;

const BASE_URL: &str = "http://localhost:3000";

// =============================================================================
// Feature Navigation Tests
// =============================================================================

#[e2e]
async fn navigate_to_cloudemu_from_landing() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for(".landing-page").await;

    // Click CloudEmu feature card
    page.click("[data-feature='cloudemu'] a, .feature-card[href='/cloudemu']").await;
    page.wait_for(".cloudemu-layout").await;

    // Should be on CloudEmu page
    assert!(page.url().await.contains("/cloudemu"));
    assert!(page.query(".cloudemu-layout").is_visible().await);
}

#[e2e]
async fn navigate_to_cloudkit_from_landing() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for(".landing-page").await;

    // Click CloudKit feature card
    page.click("[data-feature='cloudkit'] a, .feature-card[href='/cloudkit']").await;
    page.wait_for(".cloudkit-layout").await;

    // Should be on CloudKit page
    assert!(page.url().await.contains("/cloudkit"));
    assert!(page.query(".cloudkit-layout").is_visible().await);
}

#[e2e]
async fn navigate_to_iac_from_landing() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for(".landing-page").await;

    // Click IAC feature card
    page.click("[data-feature='iac'] a, .feature-card[href='/iac']").await;
    page.wait_for(".iac-layout").await;

    // Should be on IAC page
    assert!(page.url().await.contains("/iac"));
    assert!(page.query(".iac-layout").is_visible().await);
}

// =============================================================================
// Header Navigation Tests
// =============================================================================

#[e2e]
async fn brand_click_returns_to_landing() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/cloudemu", BASE_URL)).await;
    page.wait_for(".cloudemu-layout").await;

    // Click brand logo/text
    page.click(".brand").await;
    page.wait_for(".landing-page").await;

    // Should be back on landing page
    assert_eq!(page.url().await, format!("{}/", BASE_URL));
    assert!(page.query(".landing-page").is_visible().await);
}

#[e2e]
async fn search_input_is_focusable() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for(".search-wrapper").await;

    // Click search input
    page.click("input[type='search']").await;

    // Input should be focused
    let focused = page.evaluate(r#"
        document.activeElement.matches("input[type='search']")
    "#).await;
    assert!(focused);
}

#[e2e]
async fn search_shortcut_focuses_search() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for(".search-wrapper").await;

    // Press '/' shortcut
    page.keyboard().press("/").await;

    // Search input should be focused
    let focused = page.evaluate(r#"
        document.activeElement.matches("input[type='search']")
    "#).await;
    assert!(focused);
}

// =============================================================================
// User Menu Navigation Tests
// =============================================================================

#[e2e]
async fn user_menu_dropdown_opens() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for(".user-menu").await;

    // Click user button
    page.click(".user-button").await;
    page.wait_for(".user-dropdown").await;

    assert!(page.query(".user-dropdown").is_visible().await);
}

#[e2e]
async fn user_menu_navigates_to_profile() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for(".user-menu").await;

    // Open user menu and click profile
    page.click(".user-button").await;
    page.wait_for(".user-dropdown").await;
    page.click("a[href='/settings/profile']").await;

    // Should navigate to settings profile
    page.wait_for(".settings-page").await;
    assert!(page.url().await.contains("/settings/profile"));
}

#[e2e]
async fn notification_dropdown_opens() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for(".notification-bell").await;

    // Click notification bell
    page.click(".bell-button").await;
    page.wait_for(".notification-dropdown").await;

    assert!(page.query(".notification-dropdown").is_visible().await);
}

// =============================================================================
// Browser Navigation Tests
// =============================================================================

#[e2e]
async fn browser_back_button_works() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for(".landing-page").await;

    // Navigate to CloudEmu
    page.click("[data-feature='cloudemu'] a, .feature-card[href='/cloudemu']").await;
    page.wait_for(".cloudemu-layout").await;

    // Press browser back
    page.go_back().await;
    page.wait_for(".landing-page").await;

    // Should be back on landing page
    assert!(page.query(".landing-page").is_visible().await);
}

#[e2e]
async fn browser_forward_button_works() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for(".landing-page").await;

    // Navigate to CloudEmu then back
    page.click("[data-feature='cloudemu'] a, .feature-card[href='/cloudemu']").await;
    page.wait_for(".cloudemu-layout").await;
    page.go_back().await;
    page.wait_for(".landing-page").await;

    // Press browser forward
    page.go_forward().await;
    page.wait_for(".cloudemu-layout").await;

    // Should be on CloudEmu page
    assert!(page.query(".cloudemu-layout").is_visible().await);
}

// =============================================================================
// 404 Page Tests
// =============================================================================

#[e2e]
async fn unknown_route_shows_404() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/nonexistent-page", BASE_URL)).await;
    page.wait_for(".not-found-page").await;

    assert!(page.query(".not-found-page").is_visible().await);
    assert!(page.query("h1").text().await.contains("404"));
}

#[e2e]
async fn 404_page_has_back_link() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/nonexistent-page", BASE_URL)).await;
    page.wait_for(".not-found-page").await;

    // Should have link back to dashboard
    assert!(page.query("a[href='/']").exists().await);
}

#[e2e]
async fn 404_back_link_returns_home() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/nonexistent-page", BASE_URL)).await;
    page.wait_for(".not-found-page").await;

    page.click("a[href='/']").await;
    page.wait_for(".landing-page").await;

    assert!(page.query(".landing-page").is_visible().await);
}
