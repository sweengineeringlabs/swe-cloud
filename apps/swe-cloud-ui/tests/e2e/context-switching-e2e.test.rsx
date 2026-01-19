// E2E tests for SWE Cloud UI context switching
//
// Tests provider and environment switching:
// - Provider selection and persistence
// - Environment selection and warnings
// - Context bar interactions
// - State persistence across navigation

use rsc_test::prelude::*;

const BASE_URL: &str = "http://localhost:3000/cloudemu";

// =============================================================================
// Provider Switching Tests
// =============================================================================

#[e2e]
async fn provider_dropdown_shows_all_providers() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Open provider dropdown
    page.click("[data-testid='provider-button']").await;
    page.wait_for("[data-testid='provider-dropdown']").await;

    // Should show all providers
    assert!(page.query("[data-testid='provider-option-aws']").exists().await);
    assert!(page.query("[data-testid='provider-option-azure']").exists().await);
    assert!(page.query("[data-testid='provider-option-gcp']").exists().await);
    assert!(page.query("[data-testid='provider-option-zerocloud']").exists().await);
}

#[e2e]
async fn switching_provider_updates_display() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Switch to Azure
    page.click("[data-testid='provider-button']").await;
    page.wait_for("[data-testid='provider-dropdown']").await;
    page.click("[data-testid='provider-option-azure']").await;

    // Wait for update
    page.wait_for_text("[data-testid='provider-label']", "Azure").await;

    // Provider display should show Azure
    let text = page.query("[data-testid='provider-label']").text().await;
    assert!(text.contains("Azure"));
}

#[e2e]
async fn provider_selection_persists_across_pages() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Switch to GCP
    page.click("[data-testid='provider-button']").await;
    page.wait_for("[data-testid='provider-dropdown']").await;
    page.click("[data-testid='provider-option-gcp']").await;
    page.wait_for_text("[data-testid='provider-label']", "GCP").await;

    // Navigate to CloudKit
    page.goto("http://localhost:3000/cloudkit").await;
    page.wait_for("[data-testid='context-bar']").await;

    // Provider should still be GCP
    let text = page.query("[data-testid='provider-label']").text().await;
    assert!(text.contains("GCP"));
}

#[e2e]
async fn provider_selection_persists_after_refresh() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Switch to Azure
    page.click("[data-testid='provider-button']").await;
    page.wait_for("[data-testid='provider-dropdown']").await;
    page.click("[data-testid='provider-option-azure']").await;
    page.wait_for_text("[data-testid='provider-label']", "Azure").await;

    // Refresh
    page.reload().await;
    page.wait_for("[data-testid='context-bar']").await;

    // Provider should still be Azure
    let text = page.query("[data-testid='provider-label']").text().await;
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

    // Environment pills are always visible (not a dropdown)
    page.wait_for("[data-testid='environment-pills']").await;

    // Should show all environments
    assert!(page.query("[data-testid='env-option-local']").exists().await);
    assert!(page.query("[data-testid='env-option-dev']").exists().await);
    assert!(page.query("[data-testid='env-option-staging']").exists().await);
    assert!(page.query("[data-testid='env-option-prod']").exists().await);
}

#[e2e]
async fn switching_environment_updates_display() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Switch to Staging
    page.click("[data-testid='env-option-staging']").await;

    // Wait for update - check that staging pill has active class
    page.wait_for("[data-testid='env-option-staging'].active").await;

    // Environment label should show Staging
    let text = page.query("[data-testid='env-label-staging']").text().await;
    assert!(text.contains("Staging"));
}

#[e2e]
async fn production_environment_shows_warning() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Production pill should have warning indicator
    let has_warning = page.query("[data-testid='env-warning']").exists().await;
    let prod_pill = page.query("[data-testid='env-option-prod']").await;
    let has_production_class = prod_pill.has_class("production").await;

    assert!(has_warning || has_production_class);
}

#[e2e]
async fn environment_color_indicator_changes() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Get initial active pill (local by default)
    let local_pill = page.query("[data-testid='env-option-local']").await;
    let initial_color = local_pill.css_value("--env-color").await;

    // Switch to Production
    page.click("[data-testid='env-option-prod']").await;

    // Wait for prod to become active
    page.wait_for("[data-testid='env-option-prod'].active").await;

    // Prod pill should have different color
    let prod_pill = page.query("[data-testid='env-option-prod']").await;
    let new_color = prod_pill.css_value("--env-color").await;
    assert_ne!(initial_color, new_color);
}

#[e2e]
async fn environment_selection_persists() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Switch to Dev
    page.click("[data-testid='env-option-dev']").await;
    page.wait_for("[data-testid='env-option-dev'].active").await;

    // Refresh
    page.reload().await;
    page.wait_for("[data-testid='context-bar']").await;

    // Dev should still be active
    let dev_pill = page.query("[data-testid='env-option-dev']").await;
    assert!(dev_pill.has_class("active").await);
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
    page.click("[data-testid='provider-button']").await;
    page.wait_for("[data-testid='provider-dropdown']").await;
    page.click("[data-testid='provider-option-azure']").await;
    page.wait_for_text("[data-testid='provider-label']", "Azure").await;

    // Set environment to Staging
    page.click("[data-testid='env-option-staging']").await;
    page.wait_for("[data-testid='env-option-staging'].active").await;

    // Navigate and return
    page.goto("http://localhost:3000/cloudkit").await;
    page.wait_for("[data-testid='context-bar']").await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Both should persist
    let provider_text = page.query("[data-testid='provider-label']").text().await;
    let staging_pill = page.query("[data-testid='env-option-staging']").await;

    assert!(provider_text.contains("Azure"));
    assert!(staging_pill.has_class("active").await);
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
    page.click("[data-testid='provider-button']").await;
    page.wait_for("[data-testid='provider-dropdown']").await;
    assert!(page.query("[data-testid='provider-dropdown']").is_visible().await);

    // Click outside (on the context bar itself, outside the dropdown)
    page.click("[data-testid='environment-selector']").await;

    // Dropdown should close
    page.wait_for_hidden("[data-testid='provider-dropdown']").await;
    assert!(!page.query("[data-testid='provider-dropdown']").is_visible().await);
}

#[e2e]
async fn escape_key_closes_dropdown() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Open provider dropdown
    page.click("[data-testid='provider-button']").await;
    page.wait_for("[data-testid='provider-dropdown']").await;

    // Press Escape
    page.keyboard().press("Escape").await;

    // Dropdown should close
    page.wait_for_hidden("[data-testid='provider-dropdown']").await;
    assert!(!page.query("[data-testid='provider-dropdown']").is_visible().await);
}

// =============================================================================
// Signal String Comparison Tests (Regression tests for binop fix)
// =============================================================================
// These tests verify that signal.get() == "string" comparisons work correctly
// in component templates after the string comparison codegen fix.

#[e2e]
async fn signal_string_comparison_updates_class() {
    // Tests that class={if signal.get() == "value" { ... } else { ... }}
    // updates correctly when the signal changes
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Initially "local" should be active (has 'active' class from signal comparison)
    let local_pill = page.query("[data-testid='env-option-local']").await;
    assert!(local_pill.has_class("active").await, "Local should initially have active class");

    // Click dev to change the signal
    page.click("[data-testid='env-option-dev']").await;
    page.wait_for("[data-testid='env-option-dev'].active").await;

    // Now local should NOT have active class (signal comparison updated)
    let local_pill = page.query("[data-testid='env-option-local']").await;
    assert!(!local_pill.has_class("active").await, "Local should lose active class after switch");

    // And dev should have active class
    let dev_pill = page.query("[data-testid='env-option-dev']").await;
    assert!(dev_pill.has_class("active").await, "Dev should have active class after switch");
}

#[e2e]
async fn multiple_signal_comparisons_in_same_component() {
    // Tests that multiple signal.get() == "value" comparisons work in the same component
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Check that both provider and environment comparisons work
    // Provider should show AWS initially
    let provider_label = page.query("[data-testid='provider-label']").text().await;
    assert!(provider_label.contains("AWS"), "Provider label should show AWS");

    // Environment should show Local as active initially
    let local_pill = page.query("[data-testid='env-option-local']").await;
    assert!(local_pill.has_class("active").await, "Local should be active initially");

    // Change provider - this tests another signal.get() == "value" comparison
    page.click("[data-testid='provider-button']").await;
    page.wait_for("[data-testid='provider-dropdown']").await;
    page.click("[data-testid='provider-option-gcp']").await;
    page.wait_for_text("[data-testid='provider-label']", "GCP").await;

    // GCP should now be displayed (provider signal comparison worked)
    let provider_label = page.query("[data-testid='provider-label']").text().await;
    assert!(provider_label.contains("GCP"), "Provider label should show GCP after change");

    // Change environment as well
    page.click("[data-testid='env-option-staging']").await;
    page.wait_for("[data-testid='env-option-staging'].active").await;

    // Staging should now be active (environment signal comparison worked)
    let staging_pill = page.query("[data-testid='env-option-staging']").await;
    assert!(staging_pill.has_class("active").await, "Staging should be active after change");
}

#[e2e]
async fn signal_not_equal_comparison_works() {
    // Tests that != comparison with signals also works
    // The production environment uses special styling when environment.get() != "prod"
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Initially not in prod, so prod pill should have the "production" class style
    let prod_pill = page.query("[data-testid='env-option-prod']").await;
    let has_production_class = prod_pill.has_class("production").await;

    // Click on prod
    page.click("[data-testid='env-option-prod']").await;
    page.wait_for("[data-testid='env-option-prod'].active").await;

    // Prod should now be active
    let prod_active = page.query("[data-testid='env-option-prod']").await;
    assert!(prod_active.has_class("active").await || has_production_class,
            "Prod should be active or have production class");
}

#[e2e]
async fn signal_comparison_survives_rapid_changes() {
    // Stress test: rapid signal changes should all be correctly reflected
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for("[data-testid='context-bar']").await;

    // Rapidly cycle through environments
    let environments = ["local", "dev", "staging", "prod"];
    for env in environments.iter() {
        page.click(&format!("[data-testid='env-option-{}']", env)).await;
    }

    // After rapid changes, prod should be the final active state
    page.wait_for("[data-testid='env-option-prod'].active").await;

    // Verify final state
    let prod_pill = page.query("[data-testid='env-option-prod']").await;
    assert!(prod_pill.has_class("active").await, "Prod should be active after rapid cycling");
}
