//! IAC Route Tests
//!
//! Tests for Infrastructure as Code feature routing configuration.
//! Validates route definitions, parameters, workflows, and access control.

use rustscript::test::*;
use rustscript::router::*;

// ============================================================================
// ROUTE CONFIGURATION TESTS
// ============================================================================

#[test]
fn iac_routes_are_valid() {
    let result = validate_routes("src/features/iac/routes.yaml");
    assert!(result.is_ok(), "IAC routes should be valid: {:?}", result.err());
}

#[test]
fn iac_has_overview_route() {
    let router = Router::from_feature("iac");
    let route = router.find("/iac");

    assert!(route.is_some(), "IAC overview route should exist");
    assert_eq!(route.unwrap().page, "iac-overview");
}

// ============================================================================
// MODULE ROUTES
// ============================================================================

#[test]
fn iac_modules_route_exists() {
    let router = Router::from_feature("iac");
    let route = router.find("/iac-modules");

    assert!(route.is_some(), "Modules route should exist");
    assert_eq!(route.unwrap().page, "iac-modules");
}

#[test]
fn iac_module_detail_route_captures_module() {
    let router = Router::from_feature("iac");
    let route = router.resolve("/iac-modules/vpc-network");

    assert!(route.is_some(), "Module detail route should resolve");
    assert_eq!(route.unwrap().params.get("module"), Some(&"vpc-network".to_string()));
}

// ============================================================================
// DEPLOYMENT ROUTES
// ============================================================================

#[test]
fn iac_deploy_route_exists() {
    let router = Router::from_feature("iac");
    let route = router.find("/iac-deploy");

    assert!(route.is_some(), "Deploy route should exist");
    assert_eq!(route.unwrap().page, "iac-deploy");
}

#[test]
fn iac_deploy_has_workflow() {
    let router = Router::from_feature("iac");
    let route = router.find("/iac-deploy");

    assert!(route.is_some());
    assert_eq!(route.unwrap().workflow, Some("deploy-infrastructure".to_string()));
}

#[test]
fn iac_deployments_route_exists() {
    let router = Router::from_feature("iac");
    let route = router.find("/iac-deployments");

    assert!(route.is_some(), "Deployments route should exist");
    assert_eq!(route.unwrap().page, "iac-deployments");
}

#[test]
fn iac_deployment_detail_route_captures_id() {
    let router = Router::from_feature("iac");
    let route = router.resolve("/iac-deployments/deploy-456");

    assert!(route.is_some(), "Deployment detail route should resolve");
    assert_eq!(route.unwrap().params.get("id"), Some(&"deploy-456".to_string()));
}

// ============================================================================
// STATE & PLANS ROUTES
// ============================================================================

#[test]
fn iac_state_route_exists() {
    let router = Router::from_feature("iac");
    let route = router.find("/iac-state");

    assert!(route.is_some(), "State route should exist");
    assert_eq!(route.unwrap().page, "iac-state");
}

#[test]
fn iac_plans_route_exists() {
    let router = Router::from_feature("iac");
    let route = router.find("/iac-plans");

    assert!(route.is_some(), "Plans route should exist");
    assert_eq!(route.unwrap().page, "iac-plans");
}

#[test]
fn iac_plan_detail_route_captures_id() {
    let router = Router::from_feature("iac");
    let route = router.resolve("/iac-plans/plan-789");

    assert!(route.is_some(), "Plan detail route should resolve");
    assert_eq!(route.unwrap().params.get("id"), Some(&"plan-789".to_string()));
}

// ============================================================================
// ACCESS CONTROL TESTS
// ============================================================================

#[test]
fn iac_requires_role() {
    let router = Router::from_feature("iac");
    let route = router.find("/iac");

    assert!(route.is_some());
    let roles = &route.unwrap().role_required;
    assert!(!roles.is_empty(), "IAC should require roles");
    assert!(roles.contains(&"devops".to_string()));
    assert!(roles.contains(&"admin".to_string()));
}

#[test]
fn iac_deploy_inherits_role_requirement() {
    let router = Router::from_feature("iac");
    let route = router.find("/iac-deploy");

    assert!(route.is_some());
    // Child routes should inherit role requirements
    let roles = &route.unwrap().role_required;
    assert!(roles.contains(&"devops".to_string()) || roles.contains(&"admin".to_string()));
}

// ============================================================================
// NO ROUTE CONFLICTS
// ============================================================================

#[test]
fn iac_routes_have_no_conflicts() {
    let result = check_route_conflicts("src/features/iac/routes.yaml");
    assert!(result.conflicts.is_empty(), "Routes should have no conflicts: {:?}", result.conflicts);
}

#[test]
fn iac_routes_have_no_duplicates() {
    let result = check_route_conflicts("src/features/iac/routes.yaml");
    assert!(result.duplicates.is_empty(), "Routes should have no duplicates: {:?}", result.duplicates);
}
