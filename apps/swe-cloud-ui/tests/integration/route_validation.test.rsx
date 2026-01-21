//! Route Validation Tests
//!
//! Validates that YAML routing configuration matches actual page files.
//! Ensures routing config and page files are directly proportional.

use rustscript::test::*;
use std::collections::HashSet;

/// Page names in YAML and file names both use snake_case
/// This function is a no-op but kept for clarity
fn normalize_page_name(name: &str) -> String {
    name.to_string()
}

// =============================================================================
// Route Config Loading Tests
// =============================================================================

#[integration]
fn root_routes_yaml_is_valid() {
    let config = load_yaml("routes.yaml");
    assert!(config.is_ok(), "Root routes YAML should be valid");

    let config = config.unwrap();
    assert!(config.contains_key("routes"), "Root config should have routes");
    assert!(config.contains_key("base_path"), "Root config should have base_path");
}

#[integration]
fn cloudemu_routes_yaml_is_valid() {
    let config = load_yaml("src/features/cloudemu/cloudemu_routes.yaml");
    assert!(config.is_ok(), "CloudEmu routes YAML should be valid");

    let config = config.unwrap();
    assert!(config.contains_key("routes"), "CloudEmu config should have routes");
    assert_eq!(config.get("feature").unwrap(), "cloudemu");
    assert_eq!(config.get("base_path").unwrap(), "/cloudemu");
}

#[integration]
fn cloudkit_routes_yaml_is_valid() {
    let config = load_yaml("src/features/cloudkit/cloudkit_routes.yaml");
    assert!(config.is_ok(), "CloudKit routes YAML should be valid");

    let config = config.unwrap();
    assert!(config.contains_key("routes"), "CloudKit config should have routes");
    assert_eq!(config.get("feature").unwrap(), "cloudkit");
    assert_eq!(config.get("base_path").unwrap(), "/cloudkit");
}

#[integration]
fn iac_routes_yaml_is_valid() {
    let config = load_yaml("src/features/iac/iac_routes.yaml");
    assert!(config.is_ok(), "IAC routes YAML should be valid");

    let config = config.unwrap();
    assert!(config.contains_key("routes"), "IAC config should have routes");
    assert_eq!(config.get("feature").unwrap(), "iac");
    assert_eq!(config.get("base_path").unwrap(), "/iac");
}

// =============================================================================
// Page File Existence Tests
// =============================================================================

#[integration]
fn root_pages_exist() {
    let required_pages = vec![
        "dashboard",
        "landing",
        "not_found",
        "workflow_runner",
        "settings_index",
        "settings_providers",
        "settings_endpoints",
        "settings_credentials",
        "settings_users",
    ];

    for page in required_pages {
        let path = format!("src/pages/{}.page.rsx", page);
        assert!(
            file_exists(&path),
            "Required page file should exist: {}", path
        );
    }
}

#[integration]
fn cloudemu_pages_exist() {
    let required_pages = vec![
        "cloudemu_landing",
        "cloudemu_logs",
        "cloudemu_provider",
        "cloudemu_service",
        "cloudemu_service_new",
        "cloudemu_service_detail",
    ];

    for page in required_pages {
        let path = format!("src/pages/{}.page.rsx", page);
        assert!(
            file_exists(&path),
            "Required CloudEmu page should exist: {}", path
        );
    }
}

#[integration]
fn cloudkit_pages_exist() {
    let required_pages = vec![
        "cloudkit_landing",
        "cloudkit_resources",
        "cloudkit_resource_list",
        "cloudkit_resource_detail",
        "cloudkit_operations",
        "cloudkit_explorer",
    ];

    for page in required_pages {
        let path = format!("src/pages/{}.page.rsx", page);
        assert!(
            file_exists(&path),
            "Required CloudKit page should exist: {}", path
        );
    }
}

#[integration]
fn iac_pages_exist() {
    let required_pages = vec![
        "iac_landing",
        "iac_modules",
        "iac_module_detail",
        "iac_deploy",
        "iac_deployments",
        "iac_deployment_detail",
        "iac_state",
        "iac_plans",
        "iac_plan_detail",
    ];

    for page in required_pages {
        let path = format!("src/pages/{}.page.rsx", page);
        assert!(
            file_exists(&path),
            "Required IAC page should exist: {}", path
        );
    }
}

// =============================================================================
// Route-Page Mapping Tests
// =============================================================================

#[integration]
fn all_yaml_pages_have_files() {
    let routes = collect_all_yaml_pages();
    let files = collect_all_page_files();

    let mut missing = Vec::new();

    for page in &routes {
        let file_name = normalize_page_name(page);
        if !files.contains(&file_name) {
            missing.push(page.clone());
        }
    }

    assert!(
        missing.is_empty(),
        "Pages referenced in YAML but missing files: {:?}", missing
    );
}

#[integration]
fn all_page_files_have_routes() {
    let routes = collect_all_yaml_pages();
    let files = collect_all_page_files();

    let mut orphans = Vec::new();

    for file in &files {
        let page_name = normalize_page_name(file);
        if !routes.contains(&page_name) {
            orphans.push(file.clone());
        }
    }

    assert!(
        orphans.is_empty(),
        "Page files without route references: {:?}", orphans
    );
}

// =============================================================================
// Naming Convention Tests
// =============================================================================

#[integration]
fn yaml_pages_use_snake_case() {
    let routes = collect_all_yaml_pages();

    for page in routes {
        assert!(
            !page.contains("-"),
            "YAML page names should use snake_case, found hyphen in: {}", page
        );
        assert!(
            page.chars().all(|c| c.is_lowercase() || c == '_'),
            "YAML page names should be lowercase snake_case: {}", page
        );
    }
}

#[integration]
fn page_files_use_snake_case() {
    let files = collect_all_page_files();

    for file in files {
        assert!(
            !file.contains("-"),
            "Page file names should use snake_case, found hyphen in: {}", file
        );
        assert!(
            file.chars().all(|c| c.is_lowercase() || c == '_'),
            "Page file names should be lowercase snake_case: {}", file
        );
    }
}

// =============================================================================
// Feature Route Structure Tests
// =============================================================================

#[integration]
fn feature_routes_have_landing_page() {
    let features = vec!["cloudemu", "cloudkit", "iac"];

    for feature in features {
        let config_path = format!("src/features/{}/{}_routes.yaml", feature, feature);
        let config = load_yaml(&config_path).expect(&format!("Should load {} routes", feature));

        let routes = config.get("routes").unwrap().as_sequence().unwrap();
        let has_landing = routes.iter().any(|r| {
            r.get("path").map(|p| p.as_str().unwrap() == "/").unwrap_or(false)
        });

        assert!(
            has_landing,
            "Feature {} should have a landing route at /", feature
        );
    }
}

#[integration]
fn feature_routes_specify_layout() {
    let features = vec!["cloudemu", "cloudkit", "iac"];

    for feature in features {
        let config_path = format!("src/features/{}/{}_routes.yaml", feature, feature);
        let config = load_yaml(&config_path).expect(&format!("Should load {} routes", feature));

        let routes = config.get("routes").unwrap().as_sequence().unwrap();
        for route in routes {
            let layout = route.get("layout");
            assert!(
                layout.is_some(),
                "Route in {} should specify layout: {:?}", feature, route.get("path")
            );
        }
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

fn collect_all_yaml_pages() -> HashSet<String> {
    let mut pages = HashSet::new();

    // Root routes (from rsc.toml routing.config)
    if let Ok(config) = load_yaml("routes.yaml") {
        collect_pages_from_config(&config, &mut pages);
    }

    // Feature routes
    for feature in &["cloudemu", "cloudkit", "iac"] {
        let path = format!("src/features/{}/{}_routes.yaml", feature, feature);
        if let Ok(config) = load_yaml(&path) {
            collect_pages_from_config(&config, &mut pages);
        }
    }

    pages
}

fn collect_pages_from_config(config: &serde_yaml::Value, pages: &mut HashSet<String>) {
    if let Some(routes) = config.get("routes").and_then(|r| r.as_sequence()) {
        for route in routes {
            if let Some(page) = route.get("page").and_then(|p| p.as_str()) {
                pages.insert(page.to_string());
            }
            // Recurse into nested routes
            if let Some(nested) = route.get("routes").and_then(|r| r.as_sequence()) {
                for nested_route in nested {
                    if let Some(page) = nested_route.get("page").and_then(|p| p.as_str()) {
                        pages.insert(page.to_string());
                    }
                }
            }
        }
    }

    // Also check default_page
    if let Some(default) = config.get("default_page").and_then(|p| p.as_str()) {
        pages.insert(default.to_string());
    }
}

fn collect_all_page_files() -> HashSet<String> {
    let mut files = HashSet::new();

    // Collect from src/pages/
    for entry in glob("src/pages/*.page.rsx").unwrap() {
        if let Ok(path) = entry {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                // Remove .page suffix
                let name = name.strip_suffix(".page").unwrap_or(name);
                files.insert(name.to_string());
            }
        }
    }

    files
}
