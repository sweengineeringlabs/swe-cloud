//! Not Found Route Tests
//!
//! Tests for 404 catch-all routing behavior.

use rustscript::test::*;
use rustscript::router::{TestRouter, RouteParser};

// ============================================================================
// CATCH-ALL ROUTE TESTS
// ============================================================================

#[test]
fn wildcard_route_exists() {
    let router = TestRouter::from_routes_yaml("routes.yaml");
    let route = router.find("*");

    assert!(route.is_some(), "Catch-all route should exist");
    assert_eq!(route.unwrap().page, "not-found");
}

#[test]
fn unknown_path_matches_catch_all() {
    let router = TestRouter::from_routes_yaml("routes.yaml");
    let result = router.resolve("/this-page-does-not-exist");

    assert!(result.is_some(), "Unknown path should match catch-all");
    assert_eq!(result.unwrap().route.page, "not-found");
}

#[test]
fn deeply_nested_unknown_path_matches_catch_all() {
    let router = TestRouter::from_routes_yaml("routes.yaml");
    let result = router.resolve("/some/deeply/nested/path/that/does/not/exist");

    assert!(result.is_some());
    assert_eq!(result.unwrap().route.page, "not-found");
}

#[test]
fn typo_in_known_path_matches_catch_all() {
    let router = TestRouter::from_routes_yaml("routes.yaml");

    // Typo in "cloudemu"
    let result = router.resolve("/cloudmu");
    assert!(result.is_some());
    assert_eq!(result.unwrap().route.page, "not-found");
}

// ============================================================================
// NOT FOUND CONFIG TESTS
// ============================================================================

#[test]
fn not_found_config_is_loaded() {
    let parser = RouteParser::from_file("routes.yaml").expect("Failed to parse routes");
    let config = parser.config();

    assert_eq!(config.not_found.page, "not-found");
}

#[test]
fn not_found_config_has_title() {
    let parser = RouteParser::from_file("routes.yaml").expect("Failed to parse routes");
    let config = parser.config();

    assert!(!config.not_found.title.is_empty());
}

#[test]
fn not_found_config_has_heading() {
    let parser = RouteParser::from_file("routes.yaml").expect("Failed to parse routes");
    let config = parser.config();

    assert!(!config.not_found.heading.is_empty());
}

#[test]
fn not_found_config_has_message() {
    let parser = RouteParser::from_file("routes.yaml").expect("Failed to parse routes");
    let config = parser.config();

    assert!(!config.not_found.message.is_empty());
}

#[test]
fn not_found_config_has_home_link() {
    let parser = RouteParser::from_file("routes.yaml").expect("Failed to parse routes");
    let config = parser.config();

    assert!(config.not_found.show_home_link);
    assert!(!config.not_found.home_link_text.is_empty());
    assert!(!config.not_found.home_link_path.is_empty());
}

// ============================================================================
// ROUTE PRIORITY TESTS
// ============================================================================

#[test]
fn known_routes_take_priority_over_catch_all() {
    let router = TestRouter::from_routes_yaml("routes.yaml");

    // Known route should not match catch-all
    let result = router.resolve("/cloudemu");
    assert!(result.is_some());
    assert_ne!(result.unwrap().route.page, "not-found");
}

#[test]
fn parameterized_routes_take_priority_over_catch_all() {
    let router = TestRouter::from_routes_yaml("routes.yaml");

    // Parameterized route should not match catch-all
    let result = router.resolve("/cloudemu/services/my-service");
    assert!(result.is_some());
    // Should match the service detail route, not catch-all
    assert_ne!(result.unwrap().route.page, "not-found");
}

#[test]
fn root_path_does_not_match_catch_all() {
    let router = TestRouter::from_routes_yaml("routes.yaml");

    let result = router.resolve("/");
    assert!(result.is_some());
    assert_eq!(result.unwrap().route.page, "dashboard");
}

// ============================================================================
// NAVIGATION TO 404 TESTS
// ============================================================================

#[test]
fn navigate_to_unknown_shows_not_found() {
    let router = TestRouter::from_routes_yaml("routes.yaml");
    router.navigate("/unknown-page");

    assert_eq!(router.current_page(), Some("not-found".to_string()));
}

#[test]
fn back_navigation_from_404_works() {
    let router = TestRouter::from_routes_yaml("routes.yaml");

    // Navigate to a known page first
    router.navigate("/cloudemu");
    assert_eq!(router.current_page(), Some("cloudemu-overview".to_string()));

    // Navigate to unknown (404)
    router.navigate("/unknown");
    assert_eq!(router.current_page(), Some("not-found".to_string()));

    // Go back
    router.back();
    assert_eq!(router.current_page(), Some("cloudemu-overview".to_string()));
}

// ============================================================================
// VALIDATION TESTS
// ============================================================================

#[test]
fn catch_all_route_passes_validation() {
    let result = validate_routes("routes.yaml");
    assert!(result.is_ok(), "Routes with catch-all should be valid");
}

#[test]
fn wildcard_path_is_valid() {
    use rustscript::router::validation;

    let result = validation::validate_path("*");
    assert!(result.valid, "Wildcard '*' should be a valid path");
}

#[test]
fn slash_wildcard_path_is_valid() {
    use rustscript::router::validation;

    let result = validation::validate_path("/*");
    assert!(result.valid, "Path '/*' should be valid");
}
