// Integration tests for SWE Cloud UI compilation
//
// Tests the RustScript compilation pipeline:
// - Component compilation
// - Route generation
// - Config loading
// - Boundary enforcement

use rsc_test::prelude::*;
use rsc_compiler::Compiler;

// =============================================================================
// Component Compilation Tests
// =============================================================================

#[test]
fn test_main_app_compiles() {
    let compiler = Compiler::new();
    let result = compiler.compile_file("src/main.rsc");

    assert!(result.is_ok(), "Main app should compile without errors");
}

#[test]
fn test_cloudemu_feature_compiles() {
    let compiler = Compiler::new();
    let result = compiler.compile_directory("src/features/cloudemu");

    assert!(result.is_ok(), "CloudEmu feature should compile without errors");
}

#[test]
fn test_cloudkit_feature_compiles() {
    let compiler = Compiler::new();
    let result = compiler.compile_directory("src/features/cloudkit");

    assert!(result.is_ok(), "CloudKit feature should compile without errors");
}

#[test]
fn test_iac_feature_compiles() {
    let compiler = Compiler::new();
    let result = compiler.compile_directory("src/features/iac");

    assert!(result.is_ok(), "IAC feature should compile without errors");
}

#[test]
fn test_layout_module_compiles() {
    let compiler = Compiler::new();
    let result = compiler.compile_directory("src/modules/layout");

    assert!(result.is_ok(), "Layout module should compile without errors");
}

#[test]
fn test_navigation_module_compiles() {
    let compiler = Compiler::new();
    let result = compiler.compile_directory("src/modules/navigation");

    assert!(result.is_ok(), "Navigation module should compile without errors");
}

#[test]
fn test_context_module_compiles() {
    let compiler = Compiler::new();
    let result = compiler.compile_directory("src/modules/context");

    assert!(result.is_ok(), "Context module should compile without errors");
}

#[test]
fn test_workflow_module_compiles() {
    let compiler = Compiler::new();
    let result = compiler.compile_directory("src/modules/workflow");

    assert!(result.is_ok(), "Workflow module should compile without errors");
}

// =============================================================================
// Route Generation Tests
// =============================================================================

#[test]
fn test_routes_yaml_parses() {
    let routes = load_routes("swe-cloud-ui_routes.yaml");

    assert!(routes.is_ok(), "Root routes file should parse correctly");
    let routes = routes.unwrap();
    assert!(routes.len() > 0, "Should have at least one route");
}

#[test]
fn test_feature_routes_import() {
    let routes = load_routes("swe-cloud-ui_routes.yaml").unwrap();

    // Check feature routes are imported
    assert!(routes.iter().any(|r| r.path.starts_with("/cloudemu")));
    assert!(routes.iter().any(|r| r.path.starts_with("/cloudkit")));
    assert!(routes.iter().any(|r| r.path.starts_with("/iac")));
}

#[test]
fn test_cloudemu_routes_parse() {
    let routes = load_routes("src/features/cloudemu/cloudemu_routes.yaml");

    assert!(routes.is_ok(), "CloudEmu routes should parse correctly");
}

#[test]
fn test_cloudkit_routes_parse() {
    let routes = load_routes("src/features/cloudkit/cloudkit_routes.yaml");

    assert!(routes.is_ok(), "CloudKit routes should parse correctly");
}

#[test]
fn test_iac_routes_parse() {
    let routes = load_routes("src/features/iac/iac_routes.yaml");

    assert!(routes.is_ok(), "IAC routes should parse correctly");
}

// =============================================================================
// Config Loading Tests
// =============================================================================

#[test]
fn test_providers_config_loads() {
    let config = load_config("configs/providers.yaml");

    assert!(config.is_ok(), "Providers config should load");
    let providers = config.unwrap();
    assert!(providers.contains_key("providers"));
}

#[test]
fn test_environments_config_loads() {
    let config = load_config("configs/environments.yaml");

    assert!(config.is_ok(), "Environments config should load");
    let envs = config.unwrap();
    assert!(envs.contains_key("environments"));
}

#[test]
fn test_workflows_config_loads() {
    let config = load_config("configs/workflows.yaml");

    assert!(config.is_ok(), "Workflows config should load");
}

#[test]
fn test_activities_config_loads() {
    let config = load_config("configs/activities.yaml");

    assert!(config.is_ok(), "Activities config should load");
}

#[test]
fn test_services_config_loads() {
    let config = load_config("configs/services.yaml");

    assert!(config.is_ok(), "Services config should load");
}

// =============================================================================
// Boundary Enforcement Tests
// =============================================================================

#[test]
fn test_cloudemu_boundary_valid() {
    let boundary = load_boundary("src/features/cloudemu/_boundary.toml");

    assert!(boundary.is_ok(), "CloudEmu boundary should be valid");
    let boundary = boundary.unwrap();
    assert!(boundary.exports.len() > 0, "Should have exports");
}

#[test]
fn test_cloudkit_boundary_valid() {
    let boundary = load_boundary("src/features/cloudkit/_boundary.toml");

    assert!(boundary.is_ok(), "CloudKit boundary should be valid");
}

#[test]
fn test_iac_boundary_valid() {
    let boundary = load_boundary("src/features/iac/_boundary.toml");

    assert!(boundary.is_ok(), "IAC boundary should be valid");
}

#[test]
fn test_layout_module_boundary_valid() {
    let boundary = load_boundary("src/modules/layout/_boundary.toml");

    assert!(boundary.is_ok(), "Layout module boundary should be valid");
}

#[test]
fn test_navigation_module_boundary_valid() {
    let boundary = load_boundary("src/modules/navigation/_boundary.toml");

    assert!(boundary.is_ok(), "Navigation module boundary should be valid");
}

#[test]
fn test_context_module_boundary_valid() {
    let boundary = load_boundary("src/modules/context/_boundary.toml");

    assert!(boundary.is_ok(), "Context module boundary should be valid");
}

#[test]
fn test_workflow_module_boundary_valid() {
    let boundary = load_boundary("src/modules/workflow/_boundary.toml");

    assert!(boundary.is_ok(), "Workflow module boundary should be valid");
}

// =============================================================================
// Full Build Test
// =============================================================================

#[test]
fn test_full_project_builds() {
    let compiler = Compiler::new();
    let result = compiler.build_project(".");

    assert!(result.is_ok(), "Full project should build without errors");
}

// =============================================================================
// Helper Functions
// =============================================================================

fn load_routes(path: &str) -> Result<Vec<Route>, Error> {
    rsc_router::load_routes(path)
}

fn load_config(path: &str) -> Result<serde_yaml::Value, Error> {
    let content = std::fs::read_to_string(path)?;
    serde_yaml::from_str(&content).map_err(Into::into)
}

fn load_boundary(path: &str) -> Result<Boundary, Error> {
    rsc_boundaries::load_boundary(path)
}

struct Route {
    path: String,
}

struct Boundary {
    exports: Vec<String>,
}
