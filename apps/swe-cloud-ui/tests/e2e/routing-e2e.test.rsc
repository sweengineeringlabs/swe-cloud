// E2E tests for SWE Cloud UI routing
//
// Tests route matching and parameter handling:
// - Static routes
// - Dynamic routes with parameters
// - Nested routes
// - Route guards and redirects

use rsc_test::prelude::*;

const BASE_URL: &str = "http://localhost:3000";

// =============================================================================
// Static Route Tests
// =============================================================================

#[e2e]
async fn root_route_loads_landing() {
    let page = browser.new_page().await;
    page.goto(BASE_URL).await;
    page.wait_for(".landing-page").await;

    assert_eq!(page.url().await, format!("{}/", BASE_URL));
    assert!(page.query(".landing-page").is_visible().await);
}

#[e2e]
async fn cloudemu_base_route_loads() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/cloudemu", BASE_URL)).await;
    page.wait_for(".cloudemu-layout").await;

    assert!(page.query(".cloudemu-layout").is_visible().await);
}

#[e2e]
async fn cloudkit_base_route_loads() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/cloudkit", BASE_URL)).await;
    page.wait_for(".cloudkit-layout").await;

    assert!(page.query(".cloudkit-layout").is_visible().await);
}

#[e2e]
async fn iac_base_route_loads() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/iac", BASE_URL)).await;
    page.wait_for(".iac-layout").await;

    assert!(page.query(".iac-layout").is_visible().await);
}

#[e2e]
async fn settings_route_loads() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/settings", BASE_URL)).await;
    page.wait_for(".settings-page").await;

    assert!(page.query(".settings-page").is_visible().await);
}

// =============================================================================
// Dynamic Route Tests
// =============================================================================

#[e2e]
async fn workflow_route_with_id_loads() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/workflow/create-resource", BASE_URL)).await;
    page.wait_for(".workflow-runner").await;

    assert!(page.query(".workflow-runner").is_visible().await);
}

#[e2e]
async fn workflow_route_receives_params() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/workflow/test-workflow-123", BASE_URL)).await;
    page.wait_for(".workflow-runner").await;

    // Workflow ID should be accessible
    let workflow_id = page.evaluate(r#"
        window.__ROUTE_PARAMS__?.workflowId ||
        document.querySelector('[data-workflow-id]')?.dataset.workflowId
    "#).await;

    assert_eq!(workflow_id, "test-workflow-123");
}

// =============================================================================
// Nested Route Tests
// =============================================================================

#[e2e]
async fn cloudemu_nested_services_route() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/cloudemu/services", BASE_URL)).await;
    page.wait_for(".cloudemu-layout").await;

    // Should be in cloudemu layout with services view
    assert!(page.query(".cloudemu-layout").is_visible().await);
    assert!(page.url().await.contains("/cloudemu/services"));
}

#[e2e]
async fn cloudemu_nested_requests_route() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/cloudemu/requests", BASE_URL)).await;
    page.wait_for(".cloudemu-layout").await;

    assert!(page.query(".cloudemu-layout").is_visible().await);
    assert!(page.url().await.contains("/cloudemu/requests"));
}

#[e2e]
async fn iac_nested_modules_route() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/iac/modules", BASE_URL)).await;
    page.wait_for(".iac-layout").await;

    assert!(page.query(".iac-layout").is_visible().await);
    assert!(page.url().await.contains("/iac/modules"));
}

#[e2e]
async fn iac_nested_deployments_route() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/iac/deployments", BASE_URL)).await;
    page.wait_for(".iac-layout").await;

    assert!(page.query(".iac-layout").is_visible().await);
    assert!(page.url().await.contains("/iac/deployments"));
}

#[e2e]
async fn settings_nested_profile_route() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/settings/profile", BASE_URL)).await;
    page.wait_for(".settings-page").await;

    assert!(page.query(".settings-page").is_visible().await);
    assert!(page.url().await.contains("/settings/profile"));
}

#[e2e]
async fn settings_nested_preferences_route() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/settings/preferences", BASE_URL)).await;
    page.wait_for(".settings-page").await;

    assert!(page.query(".settings-page").is_visible().await);
    assert!(page.url().await.contains("/settings/preferences"));
}

// =============================================================================
// Wildcard Route Tests
// =============================================================================

#[e2e]
async fn cloudemu_wildcard_catches_unknown() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/cloudemu/unknown/path", BASE_URL)).await;
    page.wait_for(".cloudemu-layout").await;

    // Should still be in cloudemu layout (wildcard catch)
    assert!(page.query(".cloudemu-layout").is_visible().await);
}

#[e2e]
async fn unknown_route_falls_to_404() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/completely/unknown/route", BASE_URL)).await;
    page.wait_for(".not-found-page").await;

    assert!(page.query(".not-found-page").is_visible().await);
}

// =============================================================================
// Direct URL Access Tests
// =============================================================================

#[e2e]
async fn direct_url_access_preserves_state() {
    let page = browser.new_page().await;

    // Navigate to nested route directly
    page.goto(&format!("{}/cloudemu/services", BASE_URL)).await;
    page.wait_for(".cloudemu-layout").await;

    // Layout should be fully rendered
    assert!(page.query("[data-testid='context-bar']").is_visible().await);
    assert!(page.query(".app-header").is_visible().await);
    assert!(page.query(".cloudemu-layout").is_visible().await);
}

#[e2e]
async fn refresh_preserves_route() {
    let page = browser.new_page().await;
    page.goto(&format!("{}/cloudkit", BASE_URL)).await;
    page.wait_for(".cloudkit-layout").await;

    // Refresh page
    page.reload().await;
    page.wait_for(".cloudkit-layout").await;

    // Should still be on cloudkit
    assert!(page.url().await.contains("/cloudkit"));
    assert!(page.query(".cloudkit-layout").is_visible().await);
}
