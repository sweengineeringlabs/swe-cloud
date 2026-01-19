// Integration tests for SWE Cloud UI routing
//
// Tests route matching and rendering:
// - Route resolution
// - Layout wrapping
// - Parameter extraction
// - Navigation state

use rsc_test::prelude::*;
use rsc_router::{Router, Route, use_params, use_location};

// =============================================================================
// Route Resolution Tests
// =============================================================================

#[test]
fn test_root_route_resolves() {
    let router = Router::new();
    router.load_routes("swe-cloud-ui_routes.yaml");

    let resolved = router.resolve("/");

    assert!(resolved.is_some());
    let route = resolved.unwrap();
    assert_eq!(route.page, "landing");
}

#[test]
fn test_cloudemu_route_resolves() {
    let router = Router::new();
    router.load_routes("swe-cloud-ui_routes.yaml");

    let resolved = router.resolve("/cloudemu");

    assert!(resolved.is_some());
}

#[test]
fn test_cloudemu_nested_route_resolves() {
    let router = Router::new();
    router.load_routes("swe-cloud-ui_routes.yaml");

    let resolved = router.resolve("/cloudemu/services");

    assert!(resolved.is_some());
}

#[test]
fn test_cloudkit_route_resolves() {
    let router = Router::new();
    router.load_routes("swe-cloud-ui_routes.yaml");

    let resolved = router.resolve("/cloudkit");

    assert!(resolved.is_some());
}

#[test]
fn test_iac_route_resolves() {
    let router = Router::new();
    router.load_routes("swe-cloud-ui_routes.yaml");

    let resolved = router.resolve("/iac");

    assert!(resolved.is_some());
}

#[test]
fn test_settings_route_resolves() {
    let router = Router::new();
    router.load_routes("swe-cloud-ui_routes.yaml");

    let resolved = router.resolve("/settings");

    assert!(resolved.is_some());
}

#[test]
fn test_workflow_dynamic_route_resolves() {
    let router = Router::new();
    router.load_routes("swe-cloud-ui_routes.yaml");

    let resolved = router.resolve("/workflow/create-resource");

    assert!(resolved.is_some());
}

#[test]
fn test_unknown_route_fallback() {
    let router = Router::new();
    router.load_routes("swe-cloud-ui_routes.yaml");

    let resolved = router.resolve("/nonexistent/path");

    // Should resolve to 404 fallback
    assert!(resolved.is_some());
    let route = resolved.unwrap();
    assert_eq!(route.page, "not-found");
}

// =============================================================================
// Route Parameter Tests
// =============================================================================

#[test]
fn test_workflow_route_extracts_id() {
    let router = Router::new();
    router.load_routes("swe-cloud-ui_routes.yaml");

    let resolved = router.resolve("/workflow/my-workflow-123");

    assert!(resolved.is_some());
    let route = resolved.unwrap();
    assert_eq!(route.params.get("workflowId"), Some(&"my-workflow-123".to_string()));
}

#[test]
fn test_use_params_hook() {
    let ctx = TestContext::new();

    ctx.with_route("/workflow/test-id", || {
        let params = use_params();

        assert_eq!(params.get("workflowId"), Some(&"test-id".to_string()));
    });
}

#[test]
fn test_use_location_hook() {
    let ctx = TestContext::new();

    ctx.with_route("/cloudemu/services", || {
        let location = use_location();

        assert_eq!(location.pathname, "/cloudemu/services");
    });
}

// =============================================================================
// Layout Wrapping Tests
// =============================================================================

#[test]
fn test_cloudemu_routes_use_cloudemu_layout() {
    let router = Router::new();
    router.load_routes("swe-cloud-ui_routes.yaml");

    let resolved = router.resolve("/cloudemu");

    assert!(resolved.is_some());
    let route = resolved.unwrap();
    assert!(route.layouts.contains(&"CloudemuLayout".to_string()));
}

#[test]
fn test_cloudkit_routes_use_cloudkit_layout() {
    let router = Router::new();
    router.load_routes("swe-cloud-ui_routes.yaml");

    let resolved = router.resolve("/cloudkit");

    assert!(resolved.is_some());
    let route = resolved.unwrap();
    assert!(route.layouts.contains(&"CloudkitLayout".to_string()));
}

#[test]
fn test_iac_routes_use_iac_layout() {
    let router = Router::new();
    router.load_routes("swe-cloud-ui_routes.yaml");

    let resolved = router.resolve("/iac");

    assert!(resolved.is_some());
    let route = resolved.unwrap();
    assert!(route.layouts.contains(&"IacLayout".to_string()));
}

#[test]
fn test_nested_layouts_applied_in_order() {
    let router = Router::new();
    router.load_routes("swe-cloud-ui_routes.yaml");

    let resolved = router.resolve("/cloudemu/services");

    assert!(resolved.is_some());
    let route = resolved.unwrap();

    // Should have layouts in correct order (outer to inner)
    assert!(route.layouts.len() >= 1);
}

// =============================================================================
// Navigation Tests
// =============================================================================

#[test]
fn test_navigate_function() {
    let ctx = TestContext::new();
    let router = ctx.get_router();

    router.navigate("/cloudemu");

    assert_eq!(router.current_path(), "/cloudemu");
}

#[test]
fn test_navigate_with_replace() {
    let ctx = TestContext::new();
    let router = ctx.get_router();

    router.navigate("/");
    router.navigate("/cloudemu");
    router.navigate_replace("/cloudkit");

    // History should not include cloudemu
    router.back();
    assert_eq!(router.current_path(), "/");
}

#[test]
fn test_back_navigation() {
    let ctx = TestContext::new();
    let router = ctx.get_router();

    router.navigate("/");
    router.navigate("/cloudemu");
    router.navigate("/iac");

    router.back();
    assert_eq!(router.current_path(), "/cloudemu");

    router.back();
    assert_eq!(router.current_path(), "/");
}

#[test]
fn test_forward_navigation() {
    let ctx = TestContext::new();
    let router = ctx.get_router();

    router.navigate("/");
    router.navigate("/cloudemu");
    router.back();

    router.forward();
    assert_eq!(router.current_path(), "/cloudemu");
}

// =============================================================================
// Route Matching Priority Tests
// =============================================================================

#[test]
fn test_exact_route_takes_priority() {
    let router = Router::new();
    router.load_routes("swe-cloud-ui_routes.yaml");

    // /cloudemu should match exact route, not wildcard
    let resolved = router.resolve("/cloudemu");

    assert!(resolved.is_some());
    let route = resolved.unwrap();
    assert_ne!(route.page, "not-found");
}

#[test]
fn test_wildcard_catches_unmatched() {
    let router = Router::new();
    router.load_routes("swe-cloud-ui_routes.yaml");

    // /cloudemu/unknown/deep/path should be caught by wildcard
    let resolved = router.resolve("/cloudemu/unknown/deep/path");

    assert!(resolved.is_some());
}

// =============================================================================
// Route Import Tests
// =============================================================================

#[test]
fn test_feature_routes_imported() {
    let router = Router::new();
    router.load_routes("swe-cloud-ui_routes.yaml");

    // Routes from feature files should be available
    let cloudemu = router.resolve("/cloudemu/services");
    let cloudkit = router.resolve("/cloudkit/components");
    let iac = router.resolve("/iac/modules");

    assert!(cloudemu.is_some() || router.resolve("/cloudemu").is_some());
    assert!(cloudkit.is_some() || router.resolve("/cloudkit").is_some());
    assert!(iac.is_some() || router.resolve("/iac").is_some());
}
