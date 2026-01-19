//! CloudEmu Route Tests
//!
//! Tests for CloudEmu feature routing configuration.
//! Validates route definitions, parameters, and navigation.

use rustscript::test::*;
use rustscript::router::*;

// ============================================================================
// ROUTE CONFIGURATION TESTS
// ============================================================================

#[test]
fn cloudemu_routes_are_valid() {
    let result = validate_routes("src/features/cloudemu/routes.yaml");
    assert!(result.is_ok(), "CloudEmu routes should be valid: {:?}", result.err());
}

#[test]
fn cloudemu_has_overview_route() {
    let router = Router::from_feature("cloudemu");
    let route = router.find("/cloudemu");

    assert!(route.is_some(), "CloudEmu overview route should exist");
    assert_eq!(route.unwrap().page, "cloudemu-overview");
}

#[test]
fn cloudemu_provider_route_accepts_valid_providers() {
    let router = Router::from_feature("cloudemu");

    for provider in ["aws", "azure", "gcp", "zero"] {
        let path = format!("/cloudemu/{}", provider);
        let route = router.resolve(&path);

        assert!(route.is_some(), "Route for provider '{}' should resolve", provider);
        assert_eq!(route.unwrap().params.get("provider"), Some(&provider.to_string()));
    }
}

#[test]
fn cloudemu_service_route_accepts_valid_services() {
    let router = Router::from_feature("cloudemu");
    let services = ["s3", "dynamodb", "lambda", "sqs", "sns", "ec2"];

    for service in services {
        let path = format!("/cloudemu/aws/{}", service);
        let route = router.resolve(&path);

        assert!(route.is_some(), "Route for service '{}' should resolve", service);
        assert_eq!(route.unwrap().params.get("service"), Some(&service.to_string()));
    }
}

#[test]
fn cloudemu_resource_detail_route_captures_id() {
    let router = Router::from_feature("cloudemu");
    let route = router.resolve("/cloudemu/aws/s3/bucket-123");

    assert!(route.is_some(), "Resource detail route should resolve");
    let matched = route.unwrap();
    assert_eq!(matched.params.get("provider"), Some(&"aws".to_string()));
    assert_eq!(matched.params.get("service"), Some(&"s3".to_string()));
    assert_eq!(matched.params.get("id"), Some(&"bucket-123".to_string()));
}

#[test]
fn cloudemu_new_resource_route_resolves() {
    let router = Router::from_feature("cloudemu");
    let route = router.resolve("/cloudemu/aws/dynamodb/new");

    assert!(route.is_some(), "New resource route should resolve");
    assert_eq!(route.unwrap().page, "cloudemu-service-new");
}

#[test]
fn cloudemu_logs_route_exists() {
    let router = Router::from_feature("cloudemu");
    let route = router.find("/cloudemu/logs");

    assert!(route.is_some(), "Logs route should exist");
    assert_eq!(route.unwrap().page, "cloudemu-logs");
}

// ============================================================================
// ROUTE PARAMETER TESTS
// ============================================================================

#[test]
fn cloudemu_provider_param_is_required() {
    let router = Router::from_feature("cloudemu");
    let route = router.find("/cloudemu/:provider");

    assert!(route.is_some());
    let param = route.unwrap().params.iter().find(|p| p.name == "provider");
    assert!(param.is_some(), "Provider param should be defined");
}

#[test]
fn cloudemu_service_param_is_required() {
    let router = Router::from_feature("cloudemu");
    let route = router.find("/cloudemu/:provider/:service");

    assert!(route.is_some());
    let params = &route.unwrap().params;
    assert!(params.iter().any(|p| p.name == "provider"));
    assert!(params.iter().any(|p| p.name == "service"));
}

// ============================================================================
// ROUTE CONTEXT TESTS
// ============================================================================

#[test]
fn cloudemu_provider_route_sets_context() {
    let router = Router::from_feature("cloudemu");
    let route = router.find("/cloudemu/:provider");

    assert!(route.is_some());
    let context = &route.unwrap().context;
    assert!(context.is_some(), "Provider route should set context");
}

// ============================================================================
// NO ROUTE CONFLICTS
// ============================================================================

#[test]
fn cloudemu_routes_have_no_conflicts() {
    let result = check_route_conflicts("src/features/cloudemu/routes.yaml");
    assert!(result.conflicts.is_empty(), "Routes should have no conflicts: {:?}", result.conflicts);
}

#[test]
fn cloudemu_routes_have_no_duplicates() {
    let result = check_route_conflicts("src/features/cloudemu/routes.yaml");
    assert!(result.duplicates.is_empty(), "Routes should have no duplicates: {:?}", result.duplicates);
}
