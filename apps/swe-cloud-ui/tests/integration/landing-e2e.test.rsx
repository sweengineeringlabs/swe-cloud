// Landing Page E2E Integration Tests
// Full integration tests for the landing page functionality

use rsc::test::*;
use rsc::test::e2e::*;

// ============================================================================
// PAGE LOAD TESTS
// ============================================================================

#[e2e_test]
async fn landing_page_loads_successfully() {
    let page = TestBrowser::new().navigate("/").await;

    assert!(page.is_loaded());
    assert_eq!(page.title(), "SWE Cloud");
}

#[e2e_test]
async fn landing_page_has_correct_url() {
    let page = TestBrowser::new().navigate("/").await;

    assert_eq!(page.url(), "/");
}

#[e2e_test]
async fn landing_page_renders_all_sections() {
    let page = TestBrowser::new().navigate("/").await;

    assert!(page.wait_for(".context-bar").await.is_ok());
    assert!(page.wait_for(".app-header").await.is_ok());
    assert!(page.wait_for(".welcome-section").await.is_ok());
    assert!(page.wait_for(".stats-section").await.is_ok());
    assert!(page.wait_for(".features-section").await.is_ok());
    assert!(page.wait_for(".status-bar").await.is_ok());
}

// ============================================================================
// PROVIDER SWITCHING TESTS
// ============================================================================

#[e2e_test]
async fn can_switch_provider_to_azure() {
    let page = TestBrowser::new().navigate("/").await;

    // Open provider dropdown
    page.click(".selector-button").await;

    // Wait for dropdown
    page.wait_for(".selector-dropdown").await.expect("Dropdown should open");

    // Click Azure option
    let azure_option = page.find_all(".selector-option")
        .iter()
        .find(|opt| opt.text().contains("Azure"))
        .expect("Azure option should exist");

    azure_option.click().await;

    // Verify provider changed in UI
    assert!(page.wait_for_text("Azure").await.is_ok());
}

#[e2e_test]
async fn provider_color_updates_in_welcome_message() {
    let page = TestBrowser::new().navigate("/").await;

    // Switch to Azure (blue)
    page.click(".selector-button").await;
    page.click_text("Azure").await;

    // Check that the provider highlight has Azure color
    let highlight = page.find(".provider-highlight");
    let style = highlight.style("color");

    // Azure blue is #0078D4 or rgb(0, 120, 212)
    assert!(style.contains("0078D4") || style.contains("rgb(0, 120, 212)"));
}

#[e2e_test]
async fn provider_dropdown_closes_after_selection() {
    let page = TestBrowser::new().navigate("/").await;

    page.click(".selector-button").await;
    assert!(page.exists(".selector-dropdown"));

    page.click_text("GCP").await;

    // Dropdown should close
    assert!(!page.exists(".selector-dropdown"));
}

// ============================================================================
// ENVIRONMENT SWITCHING TESTS
// ============================================================================

#[e2e_test]
async fn can_switch_environment_to_dev() {
    let page = TestBrowser::new().navigate("/").await;

    // Find and click Dev pill
    let dev_pill = page.find_all(".env-pill")
        .iter()
        .find(|pill| pill.text().contains("Dev"))
        .expect("Dev pill should exist");

    dev_pill.click().await;

    // Verify environment changed
    assert!(dev_pill.has_class("active"));
}

#[e2e_test]
async fn production_environment_shows_warning() {
    let page = TestBrowser::new().navigate("/").await;

    let prod_pill = page.find_all(".env-pill")
        .iter()
        .find(|pill| pill.text().contains("Prod"))
        .expect("Prod pill should exist");

    // Should have warning indicator
    assert!(prod_pill.exists(".env-warning"));
}

#[e2e_test]
async fn status_bar_shows_current_environment() {
    let page = TestBrowser::new().navigate("/").await;

    // Default is Local
    let status_bar = page.find(".status-bar");
    assert!(status_bar.text().contains("Local"));

    // Switch to Staging
    page.click_text("Staging").await;

    // Status bar should update
    assert!(status_bar.text().contains("Staging"));
}

// ============================================================================
// NAVIGATION TESTS
// ============================================================================

#[e2e_test]
async fn cloudemu_feature_card_navigates_to_cloudemu() {
    let page = TestBrowser::new().navigate("/").await;

    // Click CloudEmu feature card
    let cloudemu_card = page.find("a[href='/cloudemu']");
    cloudemu_card.click().await;

    // Should navigate to CloudEmu
    page.wait_for_navigation().await;
    assert!(page.url().starts_with("/cloudemu"));
}

#[e2e_test]
async fn cloudkit_feature_card_navigates_to_cloudkit() {
    let page = TestBrowser::new().navigate("/").await;

    let cloudkit_card = page.find("a[href='/cloudkit']");
    cloudkit_card.click().await;

    page.wait_for_navigation().await;
    assert!(page.url().starts_with("/cloudkit"));
}

#[e2e_test]
async fn iac_feature_card_navigates_to_iac() {
    let page = TestBrowser::new().navigate("/").await;

    let iac_card = page.find("a[href='/iac']");
    iac_card.click().await;

    page.wait_for_navigation().await;
    assert!(page.url().starts_with("/iac"));
}

#[e2e_test]
async fn brand_logo_navigates_to_home() {
    let page = TestBrowser::new().navigate("/cloudemu").await;

    // Click brand logo
    page.click(".brand").await;

    page.wait_for_navigation().await;
    assert_eq!(page.url(), "/");
}

// ============================================================================
// SEARCH FUNCTIONALITY TESTS
// ============================================================================

#[e2e_test]
async fn search_input_is_focusable() {
    let page = TestBrowser::new().navigate("/").await;

    let input = page.find("input[type='search']");
    input.focus().await;

    assert!(input.is_focused());
}

#[e2e_test]
async fn slash_key_focuses_search() {
    let page = TestBrowser::new().navigate("/").await;

    // Press '/' key
    page.keyboard().press("/").await;

    let input = page.find("input[type='search']");
    assert!(input.is_focused());
}

#[e2e_test]
async fn search_input_accepts_text() {
    let page = TestBrowser::new().navigate("/").await;

    let input = page.find("input[type='search']");
    input.type_text("s3 buckets").await;

    assert_eq!(input.value(), "s3 buckets");
}

// ============================================================================
// NOTIFICATION DROPDOWN TESTS
// ============================================================================

#[e2e_test]
async fn notification_dropdown_opens_on_click() {
    let page = TestBrowser::new().navigate("/").await;

    page.click(".bell-button").await;

    assert!(page.wait_for(".notification-dropdown").await.is_ok());
}

#[e2e_test]
async fn notification_dropdown_closes_on_outside_click() {
    let page = TestBrowser::new().navigate("/").await;

    page.click(".bell-button").await;
    assert!(page.exists(".notification-dropdown"));

    // Click outside
    page.click("body").await;

    assert!(!page.exists(".notification-dropdown"));
}

#[e2e_test]
async fn notification_badge_shows_count() {
    let page = TestBrowser::new().navigate("/").await;

    let badge = page.find(".badge");
    assert!(badge.exists());

    // Default is 3 unread
    assert!(badge.text().parse::<i32>().unwrap() > 0);
}

// ============================================================================
// USER MENU TESTS
// ============================================================================

#[e2e_test]
async fn user_menu_opens_on_click() {
    let page = TestBrowser::new().navigate("/").await;

    page.click(".user-button").await;

    assert!(page.wait_for(".user-dropdown").await.is_ok());
}

#[e2e_test]
async fn user_menu_shows_user_info() {
    let page = TestBrowser::new().navigate("/").await;

    page.click(".user-button").await;

    assert!(page.exists(".user-name"));
    assert!(page.exists(".user-email"));
}

#[e2e_test]
async fn user_menu_profile_link_works() {
    let page = TestBrowser::new().navigate("/").await;

    page.click(".user-button").await;
    page.click("a[href='/settings/profile']").await;

    page.wait_for_navigation().await;
    assert!(page.url().contains("/settings/profile"));
}

// ============================================================================
// RESPONSIVE DESIGN TESTS
// ============================================================================

#[e2e_test]
async fn landing_page_works_on_mobile() {
    let page = TestBrowser::new()
        .viewport(375, 667) // iPhone SE
        .navigate("/")
        .await;

    assert!(page.is_loaded());
    assert!(page.exists(".landing-page"));
}

#[e2e_test]
async fn stats_grid_is_responsive() {
    let page = TestBrowser::new()
        .viewport(375, 667)
        .navigate("/")
        .await;

    let stats_grid = page.find(".stats-grid");

    // On mobile, should stack vertically
    let computed_style = stats_grid.computed_style();
    assert!(computed_style.contains("grid") || computed_style.contains("flex"));
}

#[e2e_test]
async fn search_hidden_on_mobile() {
    let page = TestBrowser::new()
        .viewport(375, 667)
        .navigate("/")
        .await;

    // Search should be hidden on mobile
    let search = page.find(".header-center");
    assert!(search.is_hidden());
}

// ============================================================================
// ACCESSIBILITY TESTS
// ============================================================================

#[e2e_test]
async fn page_has_main_heading() {
    let page = TestBrowser::new().navigate("/").await;

    let h1 = page.find("h1");
    assert!(h1.exists());
    assert!(h1.text().contains("Welcome"));
}

#[e2e_test]
async fn all_links_are_accessible() {
    let page = TestBrowser::new().navigate("/").await;

    let links = page.find_all("a");

    for link in links {
        let href = link.attribute("href");
        assert!(href.is_some(), "All links should have href");
        assert!(!href.unwrap().is_empty(), "Links should not have empty href");
    }
}

#[e2e_test]
async fn feature_cards_are_keyboard_navigable() {
    let page = TestBrowser::new().navigate("/").await;

    // Tab to first feature card
    page.keyboard().press("Tab").await;
    page.keyboard().press("Tab").await; // Skip search
    page.keyboard().press("Tab").await; // Skip notification
    page.keyboard().press("Tab").await; // Skip user menu

    // Should be able to tab through feature cards
    let focused = page.focused_element();
    assert!(focused.has_class("feature-card"));
}

// ============================================================================
// PERFORMANCE TESTS
// ============================================================================

#[e2e_test]
async fn landing_page_loads_within_timeout() {
    let start = std::time::Instant::now();

    let page = TestBrowser::new()
        .navigate("/")
        .await;

    let duration = start.elapsed();

    assert!(duration.as_millis() < 3000, "Page should load within 3 seconds");
}

#[e2e_test]
async fn no_console_errors_on_load() {
    let page = TestBrowser::new().navigate("/").await;

    let errors = page.console_errors();
    assert!(errors.is_empty(), "No console errors on page load");
}
