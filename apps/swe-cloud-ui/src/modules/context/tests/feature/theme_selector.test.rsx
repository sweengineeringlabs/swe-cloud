// ThemeSelector Component Tests
// Tests for the theme selector dropdown component

use rsc::test::*;
use crate::modules::context::theme::{ThemeMode, ThemeProvider, ThemeSelector};

// ============================================================================
// RENDER TESTS
// ============================================================================

#[test]
fn theme_selector_renders_without_error() {
    let result = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    assert!(result.is_ok());
}

#[test]
fn theme_selector_has_correct_class() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    assert!(rendered.query_selector(".theme-selector").is_some());
}

#[test]
fn theme_selector_has_testid() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    assert!(rendered.query_selector("[data-testid='theme-selector']").is_some());
}

// ============================================================================
// BUTTON TESTS
// ============================================================================

#[test]
fn theme_selector_has_button() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    assert!(rendered.query_selector(".selector-button").is_some());
    assert!(rendered.query_selector("[data-testid='theme-button']").is_some());
}

#[test]
fn theme_selector_button_shows_current_theme_icon() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    let icon = rendered.query_selector(".selector-icon");
    assert!(icon.is_some());
    // Default is dark
    assert!(icon.unwrap().contains_text("dark-icon"));
}

#[test]
fn theme_selector_button_shows_current_theme_label() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    let label = rendered.query_selector(".selector-label");
    assert!(label.is_some());
    assert!(label.unwrap().contains_text("Dark"));
}

#[test]
fn theme_selector_label_has_testid() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    assert!(rendered.query_selector("[data-testid='theme-label']").is_some());
}

#[test]
fn theme_selector_button_shows_arrow() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    let arrow = rendered.query_selector(".selector-arrow");
    assert!(arrow.is_some());
    assert!(arrow.unwrap().contains_text("arrow-down"));
}

// ============================================================================
// DROPDOWN VISIBILITY TESTS
// ============================================================================

#[test]
fn theme_selector_dropdown_hidden_by_default() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    assert!(rendered.query_selector(".selector-dropdown").is_none());
    assert!(rendered.query_selector("[data-testid='theme-dropdown']").is_none());
}

#[test]
fn theme_selector_dropdown_shows_on_click() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    rendered.fire_event("[data-testid='theme-button']", "click");

    assert!(rendered.query_selector(".selector-dropdown").is_some());
    assert!(rendered.query_selector("[data-testid='theme-dropdown']").is_some());
}

#[test]
fn theme_selector_dropdown_hides_on_second_click() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    // Open
    rendered.fire_event("[data-testid='theme-button']", "click");
    assert!(rendered.query_selector(".selector-dropdown").is_some());

    // Close
    rendered.fire_event("[data-testid='theme-button']", "click");
    assert!(rendered.query_selector(".selector-dropdown").is_none());
}

// ============================================================================
// DROPDOWN OPTIONS TESTS
// ============================================================================

#[test]
fn theme_selector_dropdown_has_three_options() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    rendered.fire_event("[data-testid='theme-button']", "click");

    let options = rendered.query_selector_all(".selector-option");
    assert_eq!(options.len(), 3);
}

#[test]
fn theme_selector_dropdown_has_dark_option() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    rendered.fire_event("[data-testid='theme-button']", "click");

    let option = rendered.query_selector("[data-testid='theme-option-dark']");
    assert!(option.is_some());
}

#[test]
fn theme_selector_dropdown_has_light_option() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    rendered.fire_event("[data-testid='theme-button']", "click");

    let option = rendered.query_selector("[data-testid='theme-option-light']");
    assert!(option.is_some());
}

#[test]
fn theme_selector_dropdown_has_system_option() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    rendered.fire_event("[data-testid='theme-button']", "click");

    let option = rendered.query_selector("[data-testid='theme-option-system']");
    assert!(option.is_some());
}

#[test]
fn theme_selector_options_have_icons() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    rendered.fire_event("[data-testid='theme-button']", "click");

    let dark_option = rendered.query_selector("[data-testid='theme-option-dark'] .option-icon");
    assert!(dark_option.is_some());
    assert!(dark_option.unwrap().contains_text("dark-icon"));

    let light_option = rendered.query_selector("[data-testid='theme-option-light'] .option-icon");
    assert!(light_option.is_some());
    assert!(light_option.unwrap().contains_text("light-icon"));

    let system_option = rendered.query_selector("[data-testid='theme-option-system'] .option-icon");
    assert!(system_option.is_some());
    assert!(system_option.unwrap().contains_text("system-icon"));
}

#[test]
fn theme_selector_options_have_labels() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    rendered.fire_event("[data-testid='theme-button']", "click");

    let dark_label = rendered.query_selector("[data-testid='theme-option-dark'] .option-label");
    assert!(dark_label.is_some());
    assert!(dark_label.unwrap().contains_text("Dark"));

    let light_label = rendered.query_selector("[data-testid='theme-option-light'] .option-label");
    assert!(light_label.is_some());
    assert!(light_label.unwrap().contains_text("Light"));

    let system_label = rendered.query_selector("[data-testid='theme-option-system'] .option-label");
    assert!(system_label.is_some());
    assert!(system_label.unwrap().contains_text("System"));
}

// ============================================================================
// ACTIVE STATE TESTS
// ============================================================================

#[test]
fn theme_selector_dark_option_active_by_default() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    rendered.fire_event("[data-testid='theme-button']", "click");

    let dark_option = rendered.query_selector("[data-testid='theme-option-dark']").unwrap();
    assert!(dark_option.has_class("active"));
}

#[test]
fn theme_selector_light_option_not_active_by_default() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    rendered.fire_event("[data-testid='theme-button']", "click");

    let light_option = rendered.query_selector("[data-testid='theme-option-light']").unwrap();
    assert!(!light_option.has_class("active"));
}

#[test]
fn theme_selector_system_option_not_active_by_default() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    rendered.fire_event("[data-testid='theme-button']", "click");

    let system_option = rendered.query_selector("[data-testid='theme-option-system']").unwrap();
    assert!(!system_option.has_class("active"));
}

// ============================================================================
// SELECTION TESTS
// ============================================================================

#[test]
fn theme_selector_selects_light_theme() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    // Open dropdown
    rendered.fire_event("[data-testid='theme-button']", "click");

    // Select light
    rendered.fire_event("[data-testid='theme-option-light']", "click");

    // Check label updated
    let label = rendered.query_selector("[data-testid='theme-label']").unwrap();
    assert!(label.contains_text("Light"));
}

#[test]
fn theme_selector_selects_system_theme() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    rendered.fire_event("[data-testid='theme-button']", "click");
    rendered.fire_event("[data-testid='theme-option-system']", "click");

    let label = rendered.query_selector("[data-testid='theme-label']").unwrap();
    assert!(label.contains_text("System"));
}

#[test]
fn theme_selector_closes_dropdown_on_selection() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    rendered.fire_event("[data-testid='theme-button']", "click");
    assert!(rendered.query_selector(".selector-dropdown").is_some());

    rendered.fire_event("[data-testid='theme-option-light']", "click");
    assert!(rendered.query_selector(".selector-dropdown").is_none());
}

#[test]
fn theme_selector_updates_active_state_on_selection() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    // Select light theme
    rendered.fire_event("[data-testid='theme-button']", "click");
    rendered.fire_event("[data-testid='theme-option-light']", "click");

    // Reopen and check active state
    rendered.fire_event("[data-testid='theme-button']", "click");

    let dark_option = rendered.query_selector("[data-testid='theme-option-dark']").unwrap();
    assert!(!dark_option.has_class("active"));

    let light_option = rendered.query_selector("[data-testid='theme-option-light']").unwrap();
    assert!(light_option.has_class("active"));
}

#[test]
fn theme_selector_updates_icon_on_selection() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    // Initially dark
    let icon = rendered.query_selector(".selector-icon").unwrap();
    assert!(icon.contains_text("dark-icon"));

    // Select light
    rendered.fire_event("[data-testid='theme-button']", "click");
    rendered.fire_event("[data-testid='theme-option-light']", "click");

    let icon = rendered.query_selector(".selector-icon").unwrap();
    assert!(icon.contains_text("light-icon"));

    // Select system
    rendered.fire_event("[data-testid='theme-button']", "click");
    rendered.fire_event("[data-testid='theme-option-system']", "click");

    let icon = rendered.query_selector(".selector-icon").unwrap();
    assert!(icon.contains_text("system-icon"));
}

// ============================================================================
// THEME PROVIDER INTEGRATION TESTS
// ============================================================================

#[test]
fn theme_selector_updates_data_theme_attribute() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    // Initial: dark
    let root = rendered.query_selector(".theme-root").unwrap();
    assert_eq!(root.attribute("data-theme"), Some("dark"));

    // Select light
    rendered.fire_event("[data-testid='theme-button']", "click");
    rendered.fire_event("[data-testid='theme-option-light']", "click");

    let root = rendered.query_selector(".theme-root").unwrap();
    assert_eq!(root.attribute("data-theme"), Some("light"));

    // Select system
    rendered.fire_event("[data-testid='theme-button']", "click");
    rendered.fire_event("[data-testid='theme-option-system']", "click");

    let root = rendered.query_selector(".theme-root").unwrap();
    assert_eq!(root.attribute("data-theme"), Some("system"));
}

// ============================================================================
// ACCESSIBILITY TESTS
// ============================================================================

#[test]
fn theme_selector_button_is_focusable() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    let button = rendered.query_selector("button.selector-button");
    assert!(button.is_some());
}

#[test]
fn theme_selector_options_are_buttons() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    rendered.fire_event("[data-testid='theme-button']", "click");

    let dark_option = rendered.query_selector("button[data-testid='theme-option-dark']");
    assert!(dark_option.is_some());

    let light_option = rendered.query_selector("button[data-testid='theme-option-light']");
    assert!(light_option.is_some());

    let system_option = rendered.query_selector("button[data-testid='theme-option-system']");
    assert!(system_option.is_some());
}

// ============================================================================
// RESELECTION TESTS
// ============================================================================

#[test]
fn theme_selector_can_reselect_same_theme() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    // Select dark (already selected)
    rendered.fire_event("[data-testid='theme-button']", "click");
    rendered.fire_event("[data-testid='theme-option-dark']", "click");

    // Should still be dark
    let label = rendered.query_selector("[data-testid='theme-label']").unwrap();
    assert!(label.contains_text("Dark"));

    // Dropdown should be closed
    assert!(rendered.query_selector(".selector-dropdown").is_none());
}

#[test]
fn theme_selector_can_switch_back_to_dark() {
    let rendered = render! {
        ThemeProvider {
            ThemeSelector()
        }
    };

    // Switch to light
    rendered.fire_event("[data-testid='theme-button']", "click");
    rendered.fire_event("[data-testid='theme-option-light']", "click");

    // Switch back to dark
    rendered.fire_event("[data-testid='theme-button']", "click");
    rendered.fire_event("[data-testid='theme-option-dark']", "click");

    let label = rendered.query_selector("[data-testid='theme-label']").unwrap();
    assert!(label.contains_text("Dark"));

    let root = rendered.query_selector(".theme-root").unwrap();
    assert_eq!(root.attribute("data-theme"), Some("dark"));
}
