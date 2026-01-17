// Integration tests for SWE Cloud UI context providers
//
// Tests context state management:
// - Provider context initialization and updates
// - Environment context initialization and updates
// - Context persistence
// - Cross-component state sharing

use rsc_test::prelude::*;
use crate::modules::context::{
    ProviderContext, EnvironmentContext, AppContextProvider,
    use_provider, use_environment,
};

// =============================================================================
// Provider Context Tests
// =============================================================================

#[test]
fn test_provider_context_initializes_with_default() {
    let ctx = TestContext::new();

    ctx.with_provider::<AppContextProvider>(|| {
        let provider = use_provider();

        // Should default to AWS
        assert_eq!(provider.current, "aws");
    });
}

#[test]
fn test_provider_context_has_all_providers() {
    let ctx = TestContext::new();

    ctx.with_provider::<AppContextProvider>(|| {
        let provider = use_provider();

        let ids: Vec<&str> = provider.options.iter().map(|p| p.id.as_str()).collect();

        assert!(ids.contains(&"aws"));
        assert!(ids.contains(&"azure"));
        assert!(ids.contains(&"gcp"));
        assert!(ids.contains(&"zerocloud"));
    });
}

#[test]
fn test_provider_context_switch() {
    let ctx = TestContext::new();

    ctx.with_provider::<AppContextProvider>(|| {
        let mut provider = use_provider();

        provider.switch("azure");

        assert_eq!(provider.current, "azure");
    });
}

#[test]
fn test_provider_context_invalid_switch() {
    let ctx = TestContext::new();

    ctx.with_provider::<AppContextProvider>(|| {
        let mut provider = use_provider();
        let original = provider.current.clone();

        provider.switch("nonexistent");

        // Should not change
        assert_eq!(provider.current, original);
    });
}

#[test]
fn test_provider_context_current_provider_info() {
    let ctx = TestContext::new();

    ctx.with_provider::<AppContextProvider>(|| {
        let provider = use_provider();

        let current = provider.current_provider();

        assert!(current.is_some());
        let current = current.unwrap();
        assert_eq!(current.id, "aws");
        assert!(!current.label.is_empty());
    });
}

#[test]
fn test_provider_context_color() {
    let ctx = TestContext::new();

    ctx.with_provider::<AppContextProvider>(|| {
        let provider = use_provider();

        let color = provider.color();

        assert!(color.is_some());
        assert!(color.unwrap().starts_with("#"));
    });
}

// =============================================================================
// Environment Context Tests
// =============================================================================

#[test]
fn test_environment_context_initializes_with_local() {
    let ctx = TestContext::new();

    ctx.with_provider::<AppContextProvider>(|| {
        let env = use_environment();

        // Should default to local
        assert_eq!(env.current, "local");
    });
}

#[test]
fn test_environment_context_has_all_environments() {
    let ctx = TestContext::new();

    ctx.with_provider::<AppContextProvider>(|| {
        let env = use_environment();

        let ids: Vec<&str> = env.options.iter().map(|e| e.id.as_str()).collect();

        assert!(ids.contains(&"local"));
        assert!(ids.contains(&"dev"));
        assert!(ids.contains(&"staging"));
        assert!(ids.contains(&"prod"));
    });
}

#[test]
fn test_environment_context_switch() {
    let ctx = TestContext::new();

    ctx.with_provider::<AppContextProvider>(|| {
        let mut env = use_environment();

        env.switch("staging");

        assert_eq!(env.current, "staging");
    });
}

#[test]
fn test_environment_context_is_production() {
    let ctx = TestContext::new();

    ctx.with_provider::<AppContextProvider>(|| {
        let mut env = use_environment();

        assert!(!env.is_production());

        env.switch("prod");

        assert!(env.is_production());
    });
}

#[test]
fn test_environment_context_is_safe_environment() {
    let ctx = TestContext::new();

    ctx.with_provider::<AppContextProvider>(|| {
        let mut env = use_environment();

        // Local is safe
        assert!(env.is_safe_environment());

        // Dev is safe
        env.switch("dev");
        assert!(env.is_safe_environment());

        // Staging is not safe
        env.switch("staging");
        assert!(!env.is_safe_environment());

        // Prod is not safe
        env.switch("prod");
        assert!(!env.is_safe_environment());
    });
}

#[test]
fn test_environment_context_requires_confirmation() {
    let ctx = TestContext::new();

    ctx.with_provider::<AppContextProvider>(|| {
        let mut env = use_environment();

        // Local doesn't require confirmation
        assert!(!env.requires_confirmation());

        // Staging requires confirmation
        env.switch("staging");
        assert!(env.requires_confirmation());

        // Prod requires confirmation
        env.switch("prod");
        assert!(env.requires_confirmation());
    });
}

#[test]
fn test_environment_context_read_only_default() {
    let ctx = TestContext::new();

    ctx.with_provider::<AppContextProvider>(|| {
        let mut env = use_environment();

        // Local is not read-only
        assert!(!env.is_read_only());

        // Prod is read-only by default
        env.switch("prod");
        assert!(env.is_read_only());
    });
}

#[test]
fn test_environment_context_api_base() {
    let ctx = TestContext::new();

    ctx.with_provider::<AppContextProvider>(|| {
        let mut env = use_environment();

        let api = env.api_base();
        assert!(api.is_some());
        assert!(api.unwrap().contains("localhost"));

        env.switch("prod");
        let api = env.api_base();
        assert!(api.is_some());
        assert!(api.unwrap().contains("api.swe-cloud.io"));
    });
}

// =============================================================================
// Context Persistence Tests
// =============================================================================

#[test]
fn test_provider_context_persists() {
    let ctx = TestContext::new();

    // First session
    ctx.with_provider::<AppContextProvider>(|| {
        let mut provider = use_provider();
        provider.switch("gcp");
    });

    // Simulate reload
    ctx.clear_providers();

    // Second session - should restore from persistence
    ctx.with_provider::<AppContextProvider>(|| {
        let provider = use_provider();
        assert_eq!(provider.current, "gcp");
    });
}

#[test]
fn test_environment_context_persists() {
    let ctx = TestContext::new();

    // First session
    ctx.with_provider::<AppContextProvider>(|| {
        let mut env = use_environment();
        env.switch("staging");
    });

    // Simulate reload
    ctx.clear_providers();

    // Second session - should restore from persistence
    ctx.with_provider::<AppContextProvider>(|| {
        let env = use_environment();
        assert_eq!(env.current, "staging");
    });
}

// =============================================================================
// Cross-Component State Tests
// =============================================================================

#[test]
fn test_context_shared_across_components() {
    let ctx = TestContext::new();

    ctx.with_provider::<AppContextProvider>(|| {
        // Component A changes provider
        {
            let mut provider = use_provider();
            provider.switch("azure");
        }

        // Component B should see the change
        {
            let provider = use_provider();
            assert_eq!(provider.current, "azure");
        }
    });
}

#[test]
fn test_nested_context_access() {
    let ctx = TestContext::new();

    ctx.with_provider::<AppContextProvider>(|| {
        // Outer component
        let provider = use_provider();
        let env = use_environment();

        assert_eq!(provider.current, "aws");
        assert_eq!(env.current, "local");

        // Nested component would have same access
        ctx.with_nested_component(|| {
            let nested_provider = use_provider();
            let nested_env = use_environment();

            assert_eq!(nested_provider.current, "aws");
            assert_eq!(nested_env.current, "local");
        });
    });
}
