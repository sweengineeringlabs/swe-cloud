// Landing Page Tests
// Tests for the main dashboard landing page

use rsc::test::*;
use @modules::context::{ProviderContext, EnvironmentContext};
use @modules::layout::{StatCard, SectionHeader, FeatureCard, WorkspaceLayout};

// ============================================================================
// LANDING PAGE RENDER TESTS
// ============================================================================

#[test]
fn landing_page_renders_without_error() {
    let result = render! {
        TestContextProvider {
            LandingPage()
        }
    };

    assert!(result.is_ok());
}

#[test]
fn landing_page_displays_welcome_section() {
    let rendered = render! {
        TestContextProvider {
            LandingPage()
        }
    };

    assert!(rendered.contains_text("Welcome to SWE Cloud"));
    assert!(rendered.query_selector(".welcome-section").is_some());
}

#[test]
fn landing_page_displays_current_provider() {
    let rendered = render_with_context! {
        provider: ProviderContext { current: "aws".to_string(), ..Default::default() },
        LandingPage()
    };

    assert!(rendered.contains_text("AWS"));
}

#[test]
fn landing_page_displays_stats_section() {
    let rendered = render! {
        TestContextProvider {
            LandingPage()
        }
    };

    assert!(rendered.query_selector(".stats-section").is_some());
    assert!(rendered.query_selector(".stats-grid").is_some());
}

#[test]
fn landing_page_displays_four_stat_cards() {
    let rendered = render! {
        TestContextProvider {
            LandingPage()
        }
    };

    let stat_cards = rendered.query_selector_all(".stat-card");
    assert_eq!(stat_cards.len(), 4);
}

#[test]
fn landing_page_displays_features_section() {
    let rendered = render! {
        TestContextProvider {
            LandingPage()
        }
    };

    assert!(rendered.query_selector(".features-section").is_some());
    assert!(rendered.query_selector(".features-grid").is_some());
}

#[test]
fn landing_page_displays_three_feature_cards() {
    let rendered = render! {
        TestContextProvider {
            LandingPage()
        }
    };

    let feature_cards = rendered.query_selector_all(".feature-card");
    assert_eq!(feature_cards.len(), 3);
}

#[test]
fn landing_page_feature_cards_have_correct_links() {
    let rendered = render! {
        TestContextProvider {
            LandingPage()
        }
    };

    let cloudemu_link = rendered.query_selector("a[href='/cloudemu']");
    let cloudkit_link = rendered.query_selector("a[href='/cloudkit']");
    let iac_link = rendered.query_selector("a[href='/iac']");

    assert!(cloudemu_link.is_some());
    assert!(cloudkit_link.is_some());
    assert!(iac_link.is_some());
}

// ============================================================================
// STAT CARD TESTS
// ============================================================================

#[test]
fn stat_card_renders_title_and_value() {
    let rendered = render! {
        StatCard(
            title: "Active Services".to_string(),
            value: "12".to_string(),
            icon: "☁".to_string(),
            source: None,
            trend: None,
        )
    };

    assert!(rendered.contains_text("Active Services"));
    assert!(rendered.contains_text("12"));
}

#[test]
fn stat_card_renders_icon() {
    let rendered = render! {
        StatCard(
            title: "Resources".to_string(),
            value: "47".to_string(),
            icon: "📦".to_string(),
            source: None,
            trend: None,
        )
    };

    assert!(rendered.query_selector(".stat-icon").is_some());
    assert!(rendered.contains_text("📦"));
}

#[test]
fn stat_card_renders_positive_trend() {
    let rendered = render! {
        StatCard(
            title: "Deployments".to_string(),
            value: "8".to_string(),
            icon: "🚀".to_string(),
            source: None,
            trend: Some("+3".to_string()),
        )
    };

    let trend = rendered.query_selector(".stat-trend");
    assert!(trend.is_some());
    assert!(rendered.contains_text("+3"));
    assert!(trend.unwrap().has_class("trend-up"));
}

#[test]
fn stat_card_renders_negative_trend() {
    let rendered = render! {
        StatCard(
            title: "Errors".to_string(),
            value: "2".to_string(),
            icon: "⚠".to_string(),
            source: None,
            trend: Some("-1".to_string()),
        )
    };

    let trend = rendered.query_selector(".stat-trend");
    assert!(trend.is_some());
    assert!(trend.unwrap().has_class("trend-down"));
}

#[test]
fn stat_card_omits_trend_when_none() {
    let rendered = render! {
        StatCard(
            title: "Requests".to_string(),
            value: "1234".to_string(),
            icon: "📊".to_string(),
            source: None,
            trend: None,
        )
    };

    assert!(rendered.query_selector(".stat-trend").is_none());
}

// ============================================================================
// FEATURE CARD TESTS
// ============================================================================

#[test]
fn feature_card_renders_title_and_description() {
    let rendered = render! {
        FeatureCard(
            id: "cloudemu".to_string(),
            title: "CloudEmu".to_string(),
            description: "Cloud service emulation".to_string(),
            icon: "☁".to_string(),
            href: "/cloudemu".to_string(),
        )
    };

    assert!(rendered.contains_text("CloudEmu"));
    assert!(rendered.contains_text("Cloud service emulation"));
}

#[test]
fn feature_card_is_a_link() {
    let rendered = render! {
        FeatureCard(
            id: "cloudkit".to_string(),
            title: "CloudKit".to_string(),
            description: "Infrastructure toolkit".to_string(),
            icon: "📦".to_string(),
            href: "/cloudkit".to_string(),
        )
    };

    let link = rendered.query_selector("a.feature-card");
    assert!(link.is_some());
    assert_eq!(link.unwrap().attribute("href"), Some("/cloudkit"));
}

#[test]
fn feature_card_renders_icon() {
    let rendered = render! {
        FeatureCard(
            id: "iac".to_string(),
            title: "Infrastructure".to_string(),
            description: "Infrastructure as Code".to_string(),
            icon: "🔧".to_string(),
            href: "/iac".to_string(),
        )
    };

    assert!(rendered.query_selector(".feature-icon").is_some());
    assert!(rendered.contains_text("🔧"));
}

#[test]
fn feature_card_renders_arrow() {
    let rendered = render! {
        FeatureCard(
            id: "test".to_string(),
            title: "Test".to_string(),
            description: "Test feature".to_string(),
            icon: "🧪".to_string(),
            href: "/test".to_string(),
        )
    };

    assert!(rendered.query_selector(".feature-arrow").is_some());
    assert!(rendered.contains_text("→"));
}

// ============================================================================
// SECTION HEADER TESTS
// ============================================================================

#[test]
fn section_header_renders_title() {
    let rendered = render! {
        SectionHeader(
            title: "Overview".to_string(),
            action_label: None,
            action_href: None,
        )
    };

    assert!(rendered.contains_text("Overview"));
    assert!(rendered.query_selector("h2").is_some());
}

#[test]
fn section_header_renders_action_link_when_provided() {
    let rendered = render! {
        SectionHeader(
            title: "Features".to_string(),
            action_label: Some("View All".to_string()),
            action_href: Some("/features".to_string()),
        )
    };

    let action = rendered.query_selector(".section-action");
    assert!(action.is_some());
    assert!(rendered.contains_text("View All"));
    assert_eq!(action.unwrap().attribute("href"), Some("/features"));
}

#[test]
fn section_header_omits_action_when_not_provided() {
    let rendered = render! {
        SectionHeader(
            title: "Stats".to_string(),
            action_label: None,
            action_href: None,
        )
    };

    assert!(rendered.query_selector(".section-action").is_none());
}

// ============================================================================
// WORKSPACE LAYOUT TESTS
// ============================================================================

#[test]
fn workspace_layout_renders_context_bar() {
    let rendered = render! {
        TestContextProvider {
            WorkspaceLayout(feature: "dashboard".to_string()) {
                div { "Content" }
            }
        }
    };

    assert!(rendered.query_selector(".context-bar").is_some());
}

#[test]
fn workspace_layout_renders_header() {
    let rendered = render! {
        TestContextProvider {
            WorkspaceLayout(feature: "dashboard".to_string()) {
                div { "Content" }
            }
        }
    };

    assert!(rendered.query_selector(".app-header").is_some());
}

#[test]
fn workspace_layout_renders_status_bar() {
    let rendered = render! {
        TestContextProvider {
            WorkspaceLayout(feature: "dashboard".to_string()) {
                div { "Content" }
            }
        }
    };

    assert!(rendered.query_selector(".status-bar").is_some());
}

#[test]
fn workspace_layout_renders_children() {
    let rendered = render! {
        TestContextProvider {
            WorkspaceLayout(feature: "dashboard".to_string()) {
                div(class: "test-content") { "Test Content" }
            }
        }
    };

    assert!(rendered.query_selector(".test-content").is_some());
    assert!(rendered.contains_text("Test Content"));
}

#[test]
fn workspace_layout_applies_feature_class() {
    let rendered = render! {
        TestContextProvider {
            WorkspaceLayout(feature: "cloudemu".to_string()) {
                div { "Content" }
            }
        }
    };

    assert!(rendered.query_selector(".feature-cloudemu").is_some());
}

// ============================================================================
// CONTEXT INTEGRATION TESTS
// ============================================================================

#[test]
fn landing_page_updates_when_provider_changes() {
    let mut context = TestContext::new();
    context.set_provider("azure");

    let rendered = render_with_context!(context, LandingPage());

    assert!(rendered.contains_text("Azure"));
}

#[test]
fn landing_page_shows_provider_color() {
    let mut context = TestContext::new();
    context.set_provider("aws");

    let rendered = render_with_context!(context, LandingPage());

    let highlight = rendered.query_selector(".provider-highlight");
    assert!(highlight.is_some());
    // AWS color is #FF9900
    assert!(highlight.unwrap().style("color").contains("FF9900") ||
            highlight.unwrap().style("color").contains("rgb(255, 153, 0)"));
}

// ============================================================================
// ACCESSIBILITY TESTS
// ============================================================================

#[test]
fn landing_page_has_main_heading() {
    let rendered = render! {
        TestContextProvider {
            LandingPage()
        }
    };

    let h1 = rendered.query_selector("h1");
    assert!(h1.is_some());
}

#[test]
fn feature_cards_are_accessible_links() {
    let rendered = render! {
        TestContextProvider {
            LandingPage()
        }
    };

    let links = rendered.query_selector_all("a.feature-card");
    for link in links {
        // Each link should have href
        assert!(link.attribute("href").is_some());
    }
}

#[test]
fn stat_cards_have_descriptive_titles() {
    let rendered = render! {
        TestContextProvider {
            LandingPage()
        }
    };

    let titles = rendered.query_selector_all(".stat-title");
    assert_eq!(titles.len(), 4);

    // Check all titles are non-empty
    for title in titles {
        assert!(!title.text_content().is_empty());
    }
}

// ============================================================================
// HELPER: Test Context Provider
// ============================================================================

#[component]
fn TestContextProvider(children: Children) -> Element {
    use @modules::context::AppContextProvider;

    rsx! {
        AppContextProvider {
            {children}
        }
    }
}

struct TestContext {
    provider: String,
    environment: String,
}

impl TestContext {
    fn new() -> Self {
        Self {
            provider: "aws".to_string(),
            environment: "local".to_string(),
        }
    }

    fn set_provider(&mut self, provider: &str) {
        self.provider = provider.to_string();
    }

    fn set_environment(&mut self, env: &str) {
        self.environment = env.to_string();
    }
}
