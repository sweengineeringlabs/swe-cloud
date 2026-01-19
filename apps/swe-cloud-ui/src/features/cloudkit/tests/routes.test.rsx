//! CloudKit Route Tests
//!
//! Tests for CloudKit feature routing configuration.
//! Validates route definitions, parameters, and access control.

use rustscript::test::*;
use rustscript::router::*;

// ============================================================================
// ROUTE CONFIGURATION TESTS
// ============================================================================

#[test]
fn cloudkit_routes_are_valid() {
    let result = validate_routes("src/features/cloudkit/routes.yaml");
    assert!(result.is_ok(), "CloudKit routes should be valid: {:?}", result.err());
}

#[test]
fn cloudkit_has_overview_route() {
    let router = Router::from_feature("cloudkit");
    let route = router.find("/cloudkit");

    assert!(route.is_some(), "CloudKit overview route should exist");
    assert_eq!(route.unwrap().page, "cloudkit_overview");
}

#[test]
fn cloudkit_resources_route_exists() {
    let router = Router::from_feature("cloudkit");
    let route = router.find("/cloudkit_resources");

    assert!(route.is_some(), "Resources route should exist");
    assert_eq!(route.unwrap().page, "cloudkit_resources");
}

#[test]
fn cloudkit_resource_type_route_captures_type() {
    let router = Router::from_feature("cloudkit");
    let route = router.resolve("/cloudkit_resources/buckets");

    assert!(route.is_some(), "Resource type route should resolve");
    assert_eq!(route.unwrap().params.get("type"), Some(&"buckets".to_string()));
}

#[test]
fn cloudkit_resource_detail_route_captures_params() {
    let router = Router::from_feature("cloudkit");
    let route = router.resolve("/cloudkit_resources/buckets/my-bucket");

    assert!(route.is_some(), "Resource detail route should resolve");
    let matched = route.unwrap();
    assert_eq!(matched.params.get("type"), Some(&"buckets".to_string()));
    assert_eq!(matched.params.get("id"), Some(&"my-bucket".to_string()));
}

#[test]
fn cloudkit_operations_route_exists() {
    let router = Router::from_feature("cloudkit");
    let route = router.find("/cloudkit_operations");

    assert!(route.is_some(), "Operations route should exist");
    assert_eq!(route.unwrap().page, "cloudkit_operations");
}

#[test]
fn cloudkit_explorer_route_exists() {
    let router = Router::from_feature("cloudkit");
    let route = router.find("/cloudkit_explorer");

    assert!(route.is_some(), "Explorer route should exist");
    assert_eq!(route.unwrap().page, "cloudkit_explorer");
}

// ============================================================================
// ACCESS CONTROL TESTS
// ============================================================================

#[test]
fn cloudkit_requires_role() {
    let router = Router::from_feature("cloudkit");
    let route = router.find("/cloudkit");

    assert!(route.is_some());
    let roles = &route.unwrap().role_required;
    assert!(!roles.is_empty(), "CloudKit should require roles");
    assert!(roles.contains(&"developer".to_string()));
    assert!(roles.contains(&"devops".to_string()));
    assert!(roles.contains(&"admin".to_string()));
}

// ============================================================================
// CONTEXT AWARENESS TESTS
// ============================================================================

#[test]
fn cloudkit_explorer_is_context_aware() {
    let router = Router::from_feature("cloudkit");
    let route = router.find("/cloudkit_explorer");

    assert!(route.is_some());
    assert!(route.unwrap().context_aware, "Explorer should be context-aware");
}

// ============================================================================
// NO ROUTE CONFLICTS
// ============================================================================

#[test]
fn cloudkit_routes_have_no_conflicts() {
    let result = check_route_conflicts("src/features/cloudkit/routes.yaml");
    assert!(result.conflicts.is_empty(), "Routes should have no conflicts: {:?}", result.conflicts);
}

#[test]
fn cloudkit_routes_have_no_duplicates() {
    let result = check_route_conflicts("src/features/cloudkit/routes.yaml");
    assert!(result.duplicates.is_empty(), "Routes should have no duplicates: {:?}", result.duplicates);
}
