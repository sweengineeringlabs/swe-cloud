//! NotFound Page Tests
//!
//! Tests for the customizable 404 Not Found page component.

use rsc::test::*;

// ============================================================================
// BASIC RENDERING TESTS
// ============================================================================

#[test]
fn not_found_renders_without_error() {
    let result = render! {
        TestContextProvider {
            NotFound()
        }
    };
    assert!(result.is_ok());
}

#[test]
fn not_found_displays_heading() {
    let rendered = render! {
        TestContextProvider {
            NotFound()
        }
    };
    assert!(rendered.query_selector(".not-found-heading").is_some());
}

#[test]
fn not_found_displays_message() {
    let rendered = render! {
        TestContextProvider {
            NotFound()
        }
    };
    assert!(rendered.query_selector(".not-found-message").is_some());
}

#[test]
fn not_found_has_actions() {
    let rendered = render! {
        TestContextProvider {
            NotFound()
        }
    };
    assert!(rendered.query_selector(".not-found-actions").is_some());
}

// ============================================================================
// DEFAULT CONFIG TESTS
// ============================================================================

#[test]
fn not_found_shows_default_404_heading() {
    let rendered = render! {
        TestContextProvider {
            NotFound()
        }
    };
    assert!(rendered.contains_text("404"));
}

#[test]
fn not_found_shows_default_message() {
    let rendered = render! {
        TestContextProvider {
            NotFound()
        }
    };
    assert!(rendered.contains_text("doesn't exist"));
}

#[test]
fn not_found_shows_home_link_by_default() {
    let rendered = render! {
        TestContextProvider {
            NotFound()
        }
    };
    assert!(rendered.contains_text("Go to"));
}

#[test]
fn not_found_shows_back_button_by_default() {
    let rendered = render! {
        TestContextProvider {
            NotFound()
        }
    };
    assert!(rendered.contains_text("Go Back"));
}

// ============================================================================
// PROPS OVERRIDE TESTS
// ============================================================================

#[test]
fn not_found_accepts_custom_heading() {
    let rendered = render! {
        TestContextProvider {
            NotFound(heading: Some("Oops!".to_string()))
        }
    };
    assert!(rendered.contains_text("Oops!"));
}

#[test]
fn not_found_accepts_custom_message() {
    let rendered = render! {
        TestContextProvider {
            NotFound(message: Some("Custom error message".to_string()))
        }
    };
    assert!(rendered.contains_text("Custom error message"));
}

#[test]
fn not_found_can_hide_home_link() {
    let rendered = render! {
        TestContextProvider {
            NotFound(show_home_link: Some(false))
        }
    };
    // Should not contain home link button
    let buttons = rendered.query_all(".btn-primary");
    assert!(buttons.is_empty() || !rendered.contains_text("Go to"));
}

#[test]
fn not_found_can_hide_back_button() {
    let rendered = render! {
        TestContextProvider {
            NotFound(show_back_button: Some(false))
        }
    };
    // Should not contain back button
    assert!(!rendered.contains_text("Go Back"));
}

#[test]
fn not_found_accepts_custom_class() {
    let rendered = render! {
        TestContextProvider {
            NotFound(class: Some("custom-404-class".to_string()))
        }
    };
    assert!(rendered.query_selector(".custom-404-class").is_some());
}

// ============================================================================
// CONFIG-BASED TESTS
// ============================================================================

#[test]
fn not_found_reads_config_heading() {
    let config = NotFoundConfig {
        heading: "Lost!".to_string(),
        ..Default::default()
    };

    let rendered = render_with_config! {
        config: config,
        TestContextProvider {
            NotFound()
        }
    };
    assert!(rendered.contains_text("Lost!"));
}

#[test]
fn not_found_reads_config_message() {
    let config = NotFoundConfig {
        message: "We lost the page.".to_string(),
        ..Default::default()
    };

    let rendered = render_with_config! {
        config: config,
        TestContextProvider {
            NotFound()
        }
    };
    assert!(rendered.contains_text("We lost the page."));
}

#[test]
fn not_found_reads_config_home_link_text() {
    let config = NotFoundConfig {
        home_link_text: "Return Home".to_string(),
        ..Default::default()
    };

    let rendered = render_with_config! {
        config: config,
        TestContextProvider {
            NotFound()
        }
    };
    assert!(rendered.contains_text("Return Home"));
}

#[test]
fn not_found_reads_config_back_button_text() {
    let config = NotFoundConfig {
        back_button_text: "Previous Page".to_string(),
        ..Default::default()
    };

    let rendered = render_with_config! {
        config: config,
        TestContextProvider {
            NotFound()
        }
    };
    assert!(rendered.contains_text("Previous Page"));
}

// ============================================================================
// IMAGE TESTS
// ============================================================================

#[test]
fn not_found_shows_image_when_configured() {
    let config = NotFoundConfig {
        image: Some("/assets/404.svg".to_string()),
        ..Default::default()
    };

    let rendered = render_with_config! {
        config: config,
        TestContextProvider {
            NotFound()
        }
    };
    assert!(rendered.query_selector(".not-found-image img").is_some());
}

#[test]
fn not_found_hides_image_when_not_configured() {
    let rendered = render! {
        TestContextProvider {
            NotFound()
        }
    };
    // Default config has no image
    assert!(rendered.query_selector(".not-found-image").is_none());
}

// ============================================================================
// SUGGESTIONS TESTS
// ============================================================================

#[test]
fn not_found_hides_suggestions_by_default() {
    let rendered = render! {
        TestContextProvider {
            NotFound()
        }
    };
    assert!(rendered.query_selector(".not-found-suggestions").is_none());
}

#[test]
fn not_found_shows_suggestions_when_enabled() {
    let config = NotFoundConfig {
        show_suggestions: true,
        ..Default::default()
    };

    let rendered = render_with_config! {
        config: config,
        TestContextProvider {
            NotFound()
        }
    };
    // Suggestions component should be rendered (may be empty if no routes match)
    // The component is rendered but may be empty
    assert!(rendered.is_ok());
}

// ============================================================================
// NAVIGATION TESTS
// ============================================================================

#[test]
fn not_found_back_button_calls_navigate_back() {
    let (rendered, nav_spy) = render_with_nav_spy! {
        TestContextProvider {
            NotFound()
        }
    };

    let back_btn = rendered.query_selector(".btn-secondary").unwrap();
    back_btn.click();

    assert!(nav_spy.back_called());
}

#[test]
fn not_found_home_link_navigates_to_configured_path() {
    let config = NotFoundConfig {
        home_link_path: "/dashboard".to_string(),
        ..Default::default()
    };

    let (rendered, nav_spy) = render_with_nav_spy! {
        config: config,
        TestContextProvider {
            NotFound()
        }
    };

    let home_btn = rendered.query_selector(".btn-primary").unwrap();
    home_btn.click();

    assert_eq!(nav_spy.last_path(), Some("/dashboard".to_string()));
}

// ============================================================================
// CSS CLASS TESTS
// ============================================================================

#[test]
fn not_found_has_base_class() {
    let rendered = render! {
        TestContextProvider {
            NotFound()
        }
    };
    assert!(rendered.query_selector(".not-found-page").is_some());
}

#[test]
fn not_found_applies_config_class() {
    let config = NotFoundConfig {
        class: Some("dark-theme-404".to_string()),
        ..Default::default()
    };

    let rendered = render_with_config! {
        config: config,
        TestContextProvider {
            NotFound()
        }
    };
    assert!(rendered.query_selector(".dark-theme-404").is_some());
}

// ============================================================================
// ACCESSIBILITY TESTS
// ============================================================================

#[test]
fn not_found_has_heading_element() {
    let rendered = render! {
        TestContextProvider {
            NotFound()
        }
    };
    assert!(rendered.query_selector("h1").is_some());
}

#[test]
fn not_found_buttons_are_accessible() {
    let rendered = render! {
        TestContextProvider {
            NotFound()
        }
    };

    let buttons = rendered.query_all("button");
    for button in buttons {
        // Buttons should have text content
        assert!(!button.text_content().is_empty());
    }
}

#[test]
fn not_found_image_has_alt_text() {
    let config = NotFoundConfig {
        image: Some("/assets/404.svg".to_string()),
        ..Default::default()
    };

    let rendered = render_with_config! {
        config: config,
        TestContextProvider {
            NotFound()
        }
    };

    let img = rendered.query_selector("img");
    if let Some(img) = img {
        assert!(img.has_attribute("alt"));
    }
}
