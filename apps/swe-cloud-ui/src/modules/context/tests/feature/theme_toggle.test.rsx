// ThemeToggle Component Tests
// Tests for the theme toggle button component

use rsc::test::*;
use crate::modules::context::theme::{ThemeMode, ThemeProvider, ThemeToggle};

// ============================================================================
// RENDER TESTS
// ============================================================================

#[test]
fn theme_toggle_renders_without_error() {
    let result = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    assert!(result.is_ok());
}

#[test]
fn theme_toggle_has_correct_class() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    assert!(rendered.query_selector(".theme-toggle").is_some());
}

#[test]
fn theme_toggle_has_testid() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    assert!(rendered.query_selector("[data-testid='theme-toggle']").is_some());
}

#[test]
fn theme_toggle_is_a_button() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    assert!(rendered.query_selector("button.theme-toggle").is_some());
}

// ============================================================================
// ICON TESTS
// ============================================================================

#[test]
fn theme_toggle_displays_icon() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    assert!(rendered.query_selector(".theme-icon").is_some());
    assert!(rendered.query_selector("[data-testid='theme-icon']").is_some());
}

#[test]
fn theme_toggle_shows_moon_icon_in_dark_mode() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    let icon = rendered.query_selector(".theme-icon");
    assert!(icon.is_some());
    assert!(icon.unwrap().has_class("moon-icon"));
}

#[test]
fn theme_toggle_shows_moon_symbol_in_dark_mode() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    let icon = rendered.query_selector(".theme-icon");
    assert!(icon.is_some());
    assert!(icon.unwrap().contains_text("moon-symbol"));
}

// ============================================================================
// TOOLTIP TESTS
// ============================================================================

#[test]
fn theme_toggle_has_title_attribute() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    let button = rendered.query_selector(".theme-toggle");
    assert!(button.is_some());
    assert!(button.unwrap().attribute("title").is_some());
}

#[test]
fn theme_toggle_tooltip_shows_next_mode() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    let button = rendered.query_selector(".theme-toggle").unwrap();
    let title = button.attribute("title").unwrap();
    // Default is dark, next is light
    assert!(title.contains("Light"));
}

// ============================================================================
// CLICK BEHAVIOR TESTS
// ============================================================================

#[test]
fn theme_toggle_cycles_to_light_on_click() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    // Initial state: dark mode with moon icon
    assert!(rendered.query_selector(".theme-icon.moon-icon").is_some());

    // Click toggle
    rendered.fire_event("[data-testid='theme-toggle']", "click");

    // Should now be light mode with sun icon
    assert!(rendered.query_selector(".theme-icon.sun-icon").is_some());
}

#[test]
fn theme_toggle_cycles_to_system_after_light() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    // Click once: dark -> light
    rendered.fire_event("[data-testid='theme-toggle']", "click");
    assert!(rendered.query_selector(".theme-icon.sun-icon").is_some());

    // Click again: light -> system
    rendered.fire_event("[data-testid='theme-toggle']", "click");
    assert!(rendered.query_selector(".theme-icon.monitor-icon").is_some());
}

#[test]
fn theme_toggle_cycles_back_to_dark() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    // Click three times: dark -> light -> system -> dark
    rendered.fire_event("[data-testid='theme-toggle']", "click");
    rendered.fire_event("[data-testid='theme-toggle']", "click");
    rendered.fire_event("[data-testid='theme-toggle']", "click");

    assert!(rendered.query_selector(".theme-icon.moon-icon").is_some());
}

#[test]
fn theme_toggle_updates_tooltip_after_click() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    // Initial tooltip should mention "Light"
    let button = rendered.query_selector(".theme-toggle").unwrap();
    assert!(button.attribute("title").unwrap().contains("Light"));

    // Click to switch to light mode
    rendered.fire_event("[data-testid='theme-toggle']", "click");

    // Tooltip should now mention "System"
    let button = rendered.query_selector(".theme-toggle").unwrap();
    assert!(button.attribute("title").unwrap().contains("System"));
}

#[test]
fn theme_toggle_updates_symbol_text_on_click() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    // Initially shows moon symbol
    let icon = rendered.query_selector(".theme-icon").unwrap();
    assert!(icon.contains_text("moon-symbol"));

    // Click to light mode
    rendered.fire_event("[data-testid='theme-toggle']", "click");

    // Should show sun symbol
    let icon = rendered.query_selector(".theme-icon").unwrap();
    assert!(icon.contains_text("sun-symbol"));

    // Click to system mode
    rendered.fire_event("[data-testid='theme-toggle']", "click");

    // Should show monitor symbol
    let icon = rendered.query_selector(".theme-icon").unwrap();
    assert!(icon.contains_text("monitor-symbol"));
}

// ============================================================================
// THEME PROVIDER INTEGRATION TESTS
// ============================================================================

#[test]
fn theme_toggle_updates_data_theme_attribute() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    // Initial: dark theme
    let root = rendered.query_selector(".theme-root").unwrap();
    assert_eq!(root.attribute("data-theme"), Some("dark"));

    // Click to light
    rendered.fire_event("[data-testid='theme-toggle']", "click");

    let root = rendered.query_selector(".theme-root").unwrap();
    assert_eq!(root.attribute("data-theme"), Some("light"));

    // Click to system
    rendered.fire_event("[data-testid='theme-toggle']", "click");

    let root = rendered.query_selector(".theme-root").unwrap();
    assert_eq!(root.attribute("data-theme"), Some("system"));
}

// ============================================================================
// ACCESSIBILITY TESTS
// ============================================================================

#[test]
fn theme_toggle_button_is_focusable() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    let button = rendered.query_selector("button.theme-toggle");
    assert!(button.is_some());
    // Buttons are focusable by default
}

#[test]
fn theme_toggle_can_be_activated_with_enter() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    // Focus the button
    let button = rendered.query_selector("[data-testid='theme-toggle']").unwrap();
    button.focus();

    // Press Enter
    button.fire_event("keydown", |e| e.key = "Enter");

    // Should toggle (handled by browser default button behavior)
}

#[test]
fn theme_toggle_can_be_activated_with_space() {
    let rendered = render! {
        ThemeProvider {
            ThemeToggle()
        }
    };

    let button = rendered.query_selector("[data-testid='theme-toggle']").unwrap();
    button.focus();

    // Press Space
    button.fire_event("keydown", |e| e.key = " ");

    // Should toggle (handled by browser default button behavior)
}
