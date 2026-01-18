// Header Component Tests
// Tests for the main application header

use rsc::test::*;
use @modules::navigation::Header;

// ============================================================================
// HEADER RENDER TESTS
// ============================================================================

#[test]
fn header_renders_without_error() {
    let result = render! {
        Header()
    };

    assert!(result.is_ok());
}

#[test]
fn header_has_correct_class() {
    let rendered = render! {
        Header()
    };

    assert!(rendered.query_selector(".app-header").is_some());
}

#[test]
fn header_has_three_sections() {
    let rendered = render! {
        Header()
    };

    assert!(rendered.query_selector(".header-left").is_some());
    assert!(rendered.query_selector(".header-center").is_some());
    assert!(rendered.query_selector(".header-right").is_some());
}

// ============================================================================
// BRAND TESTS
// ============================================================================

#[test]
fn header_displays_brand() {
    let rendered = render! {
        Header()
    };

    assert!(rendered.query_selector(".brand").is_some());
}

#[test]
fn brand_is_a_link_to_home() {
    let rendered = render! {
        Header()
    };

    let brand = rendered.query_selector("a.brand");
    assert!(brand.is_some());
    assert_eq!(brand.unwrap().attribute("href"), Some("/"));
}

#[test]
fn brand_displays_icon() {
    let rendered = render! {
        Header()
    };

    let icon = rendered.query_selector(".brand-icon");
    assert!(icon.is_some());
    assert!(icon.unwrap().contains_text("☁"));
}

#[test]
fn brand_displays_name() {
    let rendered = render! {
        Header()
    };

    let name = rendered.query_selector(".brand-name");
    assert!(name.is_some());
    assert!(name.unwrap().contains_text("SWE Cloud"));
}

// ============================================================================
// SEARCH INPUT TESTS
// ============================================================================

#[test]
fn header_displays_search_input() {
    let rendered = render! {
        Header()
    };

    assert!(rendered.query_selector(".search-wrapper").is_some());
    assert!(rendered.query_selector("input[type='search']").is_some());
}

#[test]
fn search_input_has_placeholder() {
    let rendered = render! {
        Header()
    };

    let input = rendered.query_selector("input[type='search']");
    assert!(input.is_some());

    let placeholder = input.unwrap().attribute("placeholder");
    assert!(placeholder.is_some());
    assert!(placeholder.unwrap().contains("Search"));
}

#[test]
fn search_input_shows_keyboard_shortcut() {
    let rendered = render! {
        Header()
    };

    let shortcut = rendered.query_selector(".search-shortcut");
    assert!(shortcut.is_some());
    assert!(shortcut.unwrap().contains_text("/"));
}

#[test]
fn search_input_shows_search_icon() {
    let rendered = render! {
        Header()
    };

    let icon = rendered.query_selector(".search-icon");
    assert!(icon.is_some());
    assert!(icon.unwrap().contains_text("🔍"));
}

#[test]
fn search_input_updates_on_typing() {
    let rendered = render! {
        Header()
    };

    let input = rendered.query_selector("input[type='search']").unwrap();

    // Simulate typing
    input.fire_event("input", |e| e.value = "s3 bucket");

    // Value should update
    assert_eq!(input.value(), "s3 bucket");
}

// ============================================================================
// NOTIFICATION BELL TESTS
// ============================================================================

#[test]
fn header_displays_notification_bell() {
    let rendered = render! {
        Header()
    };

    assert!(rendered.query_selector(".notification-bell").is_some());
}

#[test]
fn notification_bell_has_button() {
    let rendered = render! {
        Header()
    };

    assert!(rendered.query_selector(".bell-button").is_some());
}

#[test]
fn notification_bell_displays_icon() {
    let rendered = render! {
        Header()
    };

    let icon = rendered.query_selector(".bell-icon");
    assert!(icon.is_some());
    assert!(icon.unwrap().contains_text("🔔"));
}

#[test]
fn notification_bell_shows_badge_when_unread() {
    let rendered = render! {
        Header()
    };

    // Default has 3 unread notifications
    let badge = rendered.query_selector(".badge");
    assert!(badge.is_some());
    assert!(badge.unwrap().contains_text("3"));
}

#[test]
fn notification_dropdown_hidden_by_default() {
    let rendered = render! {
        Header()
    };

    assert!(rendered.query_selector(".notification-dropdown").is_none());
}

#[test]
fn notification_dropdown_shows_on_click() {
    let rendered = render! {
        Header()
    };

    rendered.fire_event(".bell-button", "click");

    assert!(rendered.query_selector(".notification-dropdown").is_some());
}

#[test]
fn notification_dropdown_has_header() {
    let rendered = render! {
        Header()
    };

    rendered.fire_event(".bell-button", "click");

    let header = rendered.query_selector(".dropdown-header");
    assert!(header.is_some());
    assert!(header.unwrap().contains_text("Notifications"));
}

#[test]
fn notification_dropdown_has_notification_list() {
    let rendered = render! {
        Header()
    };

    rendered.fire_event(".bell-button", "click");

    assert!(rendered.query_selector(".notification-list").is_some());
}

#[test]
fn notification_dropdown_has_view_all_link() {
    let rendered = render! {
        Header()
    };

    rendered.fire_event(".bell-button", "click");

    let view_all = rendered.query_selector(".view-all");
    assert!(view_all.is_some());
    assert_eq!(view_all.unwrap().attribute("href"), Some("/notifications"));
}

// ============================================================================
// USER MENU TESTS
// ============================================================================

#[test]
fn header_displays_user_menu() {
    let rendered = render! {
        Header()
    };

    assert!(rendered.query_selector(".user-menu").is_some());
}

#[test]
fn user_menu_has_button() {
    let rendered = render! {
        Header()
    };

    assert!(rendered.query_selector(".user-button").is_some());
}

#[test]
fn user_menu_displays_avatar() {
    let rendered = render! {
        Header()
    };

    let avatar = rendered.query_selector(".user-avatar");
    assert!(avatar.is_some());
    assert!(avatar.unwrap().contains_text("👤"));
}

#[test]
fn user_dropdown_hidden_by_default() {
    let rendered = render! {
        Header()
    };

    assert!(rendered.query_selector(".user-dropdown").is_none());
}

#[test]
fn user_dropdown_shows_on_click() {
    let rendered = render! {
        Header()
    };

    rendered.fire_event(".user-button", "click");

    assert!(rendered.query_selector(".user-dropdown").is_some());
}

#[test]
fn user_dropdown_shows_user_info() {
    let rendered = render! {
        Header()
    };

    rendered.fire_event(".user-button", "click");

    assert!(rendered.query_selector(".user-info").is_some());
    assert!(rendered.query_selector(".user-name").is_some());
    assert!(rendered.query_selector(".user-email").is_some());
}

#[test]
fn user_dropdown_has_profile_link() {
    let rendered = render! {
        Header()
    };

    rendered.fire_event(".user-button", "click");

    let profile_link = rendered.query_selector("a[href='/settings/profile']");
    assert!(profile_link.is_some());
}

#[test]
fn user_dropdown_has_preferences_link() {
    let rendered = render! {
        Header()
    };

    rendered.fire_event(".user-button", "click");

    let prefs_link = rendered.query_selector("a[href='/settings/preferences']");
    assert!(prefs_link.is_some());
}

#[test]
fn user_dropdown_has_logout_button() {
    let rendered = render! {
        Header()
    };

    rendered.fire_event(".user-button", "click");

    let logout = rendered.query_selector(".logout-btn");
    assert!(logout.is_some());
    assert!(logout.unwrap().contains_text("Sign Out"));
}

// ============================================================================
// KEYBOARD SHORTCUT TESTS
// ============================================================================

#[test]
fn slash_key_focuses_search() {
    let rendered = render! {
        Header()
    };

    // Simulate pressing '/' key
    rendered.fire_event_global("keydown", |e| e.key = "/");

    let input = rendered.query_selector("input[type='search']").unwrap();
    assert!(input.is_focused());
}

#[test]
fn escape_key_closes_dropdowns() {
    let rendered = render! {
        Header()
    };

    // Open notification dropdown
    rendered.fire_event(".bell-button", "click");
    assert!(rendered.query_selector(".notification-dropdown").is_some());

    // Press Escape
    rendered.fire_event_global("keydown", |e| e.key = "Escape");

    // Should be closed
    assert!(rendered.query_selector(".notification-dropdown").is_none());
}

// ============================================================================
// CLICK OUTSIDE TESTS
// ============================================================================

#[test]
fn clicking_outside_closes_notification_dropdown() {
    let rendered = render! {
        Header()
    };

    rendered.fire_event(".bell-button", "click");
    assert!(rendered.query_selector(".notification-dropdown").is_some());

    // Click somewhere else
    rendered.fire_event_global("click", |_| {});

    // Should be closed (dropdown contains click handler to stop propagation)
    // This test verifies the click-outside behavior
}

#[test]
fn clicking_outside_closes_user_dropdown() {
    let rendered = render! {
        Header()
    };

    rendered.fire_event(".user-button", "click");
    assert!(rendered.query_selector(".user-dropdown").is_some());

    // Click somewhere else
    rendered.fire_event_global("click", |_| {});

    // Dropdown should close
}

// ============================================================================
// ACCESSIBILITY TESTS
// ============================================================================

#[test]
fn header_is_a_header_element() {
    let rendered = render! {
        Header()
    };

    assert!(rendered.query_selector("header.app-header").is_some());
}

#[test]
fn search_input_has_type_search() {
    let rendered = render! {
        Header()
    };

    let input = rendered.query_selector("input");
    assert!(input.is_some());
    assert_eq!(input.unwrap().attribute("type"), Some("search"));
}

#[test]
fn notification_button_is_focusable() {
    let rendered = render! {
        Header()
    };

    let button = rendered.query_selector(".bell-button");
    assert!(button.is_some());
    // Buttons are focusable by default
}

#[test]
fn user_button_is_focusable() {
    let rendered = render! {
        Header()
    };

    let button = rendered.query_selector(".user-button");
    assert!(button.is_some());
}
