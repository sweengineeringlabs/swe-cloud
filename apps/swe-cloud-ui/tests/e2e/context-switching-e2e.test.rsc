// E2E tests for SWE Cloud UI context switching
//
// Tests provider and environment switching:
// - Provider selection and persistence
// - Environment selection and warnings
// - Context bar interactions
// - State persistence across navigation

use rsc_test::prelude::*;

const BASE_URL: &str = "http://localhost:3000";

// =============================================================================
// Provider Switching Tests
// =============================================================================

#[e2e]
async fn provider_dropdown_shows_all_providers() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Open provider dropdown
    page.click(".provider-selector").await;
    page.wait_for(".provider-dropdown").await;

    // Should show all providers
    assert!(page.query("[data-provider='aws']").exists().await);
    assert!(page.query("[data-provider='azure']").exists().await);
    assert!(page.query("[data-provider='gcp']").exists().await);
    assert!(page.query("[data-provider='zerocloud']").exists().await);
}

#[e2e]
async fn switching_provider_updates_display() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Switch to Azure
    page.click(".provider-selector").await;
    page.wait_for(".provider-dropdown").await;
    page.click("[data-provider='azure']").await;

    // Wait for update
    page.wait_for_text(".provider-selector", "Azure").await;

    // Provider display should show Azure
    let text = page.query(".provider-selector").text().await;
    assert!(text.contains("Azure"));
}

#[e2e]
async fn provider_selection_persists_across_pages() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Switch to GCP
    page.click(".provider-selector").await;
    page.wait_for(".provider-dropdown").await;
    page.click("[data-provider='gcp']").await;
    page.wait_for_text(".provider-selector", "GCP").await;

    // Navigate to CloudEmu
    page.goto(&format!("{}/cloudemu", BASE_URL)).await;
    page.wait_for(".cloudemu-layout").await;

    // Provider should still be GCP
    let text = page.query(".provider-selector").text().await;
    assert!(text.contains("GCP"));
}

#[e2e]
async fn provider_selection_persists_after_refresh() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Switch to Azure
    page.click(".provider-selector").await;
    page.wait_for(".provider-dropdown").await;
    page.click("[data-provider='azure']").await;
    page.wait_for_text(".provider-selector", "Azure").await;

    // Refresh
    page.reload().await;
    page.wait_for("[data-testid='context-bar']").await;

    // Provider should still be Azure
    let text = page.query(".provider-selector").text().await;
    assert!(text.contains("Azure"));
}

// =============================================================================
// Environment Switching Tests
// =============================================================================

#[e2e]
async fn environment_dropdown_shows_all_environments() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Open environment dropdown
    page.click(".environment-selector").await;
    page.wait_for(".environment-dropdown").await;

    // Should show all environments
    assert!(page.query("[data-env='local']").exists().await);
    assert!(page.query("[data-env='dev']").exists().await);
    assert!(page.query("[data-env='staging']").exists().await);
    assert!(page.query("[data-env='prod']").exists().await);
}

#[e2e]
async fn switching_environment_updates_display() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Switch to Staging
    page.click(".environment-selector").await;
    page.wait_for(".environment-dropdown").await;
    page.click("[data-env='staging']").await;

    // Wait for update
    page.wait_for_text(".environment-selector", "Staging").await;

    // Environment display should show Staging
    let text = page.query(".environment-selector").text().await;
    assert!(text.contains("Staging"));
}

#[e2e]
async fn production_environment_shows_warning() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Switch to Production
    page.click(".environment-selector").await;
    page.wait_for(".environment-dropdown").await;
    page.click("[data-env='prod']").await;

    // Should show confirmation or warning
    // Wait for confirmation dialog or indicator
    let has_warning = page.query(".env-warning, .confirmation-dialog, .prod-indicator").exists().await;
    let has_red_indicator = page.query(".environment-indicator.prod, .environment-prod").exists().await;

    assert!(has_warning || has_red_indicator);
}

#[e2e]
async fn environment_color_indicator_changes() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Get initial indicator color (local = green)
    let indicator = page.query(".environment-indicator").await;
    let initial_color = indicator.css_value("background-color").await;

    // Switch to Production
    page.click(".environment-selector").await;
    page.wait_for(".environment-dropdown").await;
    page.click("[data-env='prod']").await;

    // Wait for environment to change
    page.wait_for_text(".environment-selector", "Production").await;

    // Indicator color should change (prod = red)
    let new_color = indicator.css_value("background-color").await;
    assert_ne!(initial_color, new_color);
}

#[e2e]
async fn environment_selection_persists() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Switch to Dev
    page.click(".environment-selector").await;
    page.wait_for(".environment-dropdown").await;
    page.click("[data-env='dev']").await;
    page.wait_for_text(".environment-selector", "Development").await;

    // Refresh
    page.reload().await;
    page.wait_for("[data-testid='context-bar']").await;

    // Environment should still be Dev
    let text = page.query(".environment-selector").text().await;
    assert!(text.contains("Development"));
}

// =============================================================================
// Combined Context Tests
// =============================================================================

#[e2e]
async fn both_contexts_persist_together() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Set provider to Azure
    page.click(".provider-selector").await;
    page.wait_for(".provider-dropdown").await;
    page.click("[data-provider='azure']").await;
    page.wait_for_text(".provider-selector", "Azure").await;

    // Set environment to Staging
    page.click(".environment-selector").await;
    page.wait_for(".environment-dropdown").await;
    page.click("[data-env='staging']").await;
    page.wait_for_text(".environment-selector", "Staging").await;

    // Navigate and return
    page.goto(&format!("{}/cloudemu", BASE_URL)).await;
    page.wait_for(".cloudemu-layout").await;
    page.goto(BASE_URL).await;
    page.wait_for(".landing-page").await;

    // Both should persist
    let provider_text = page.query(".provider-selector").text().await;
    let env_text = page.query(".environment-selector").text().await;

    assert!(provider_text.contains("Azure"));
    assert!(env_text.contains("Staging"));
}

// =============================================================================
// Dropdown Behavior Tests
// =============================================================================

#[e2e]
async fn clicking_outside_closes_provider_dropdown() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Open dropdown
    page.click(".provider-selector").await;
    page.wait_for(".provider-dropdown").await;
    assert!(page.query(".provider-dropdown").is_visible().await);

    // Click outside
    page.click(".workspace-main, .landing-page").await;

    // Dropdown should close
    page.wait_for_hidden(".provider-dropdown").await;
    assert!(!page.query(".provider-dropdown").is_visible().await);
}

#[e2e]
async fn escape_key_closes_dropdown() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Open dropdown
    page.click(".environment-selector").await;
    page.wait_for(".environment-dropdown").await;

    // Press Escape
    page.keyboard().press("Escape").await;

    // Dropdown should close
    page.wait_for_hidden(".environment-dropdown").await;
    assert!(!page.query(".environment-dropdown").is_visible().await);
}
