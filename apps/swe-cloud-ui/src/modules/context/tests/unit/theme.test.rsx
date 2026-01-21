// Theme Unit Tests
// Tests for ThemeMode enum and ThemeContext struct

use rsc::test::*;
use crate::modules::context::theme::{ThemeMode, ThemeContext};

// ============================================================================
// THEME MODE ENUM TESTS
// ============================================================================

#[test]
fn theme_mode_default_is_dark() {
    let mode = ThemeMode::default();
    assert_eq!(mode, ThemeMode::Dark);
}

#[test]
fn theme_mode_dark_as_str() {
    assert_eq!(ThemeMode::Dark.as_str(), "dark");
}

#[test]
fn theme_mode_light_as_str() {
    assert_eq!(ThemeMode::Light.as_str(), "light");
}

#[test]
fn theme_mode_system_as_str() {
    assert_eq!(ThemeMode::System.as_str(), "system");
}

#[test]
fn theme_mode_from_str_dark() {
    assert_eq!(ThemeMode::from_str("dark"), ThemeMode::Dark);
}

#[test]
fn theme_mode_from_str_light() {
    assert_eq!(ThemeMode::from_str("light"), ThemeMode::Light);
}

#[test]
fn theme_mode_from_str_system() {
    assert_eq!(ThemeMode::from_str("system"), ThemeMode::System);
}

#[test]
fn theme_mode_from_str_unknown_defaults_to_dark() {
    assert_eq!(ThemeMode::from_str("unknown"), ThemeMode::Dark);
    assert_eq!(ThemeMode::from_str(""), ThemeMode::Dark);
    assert_eq!(ThemeMode::from_str("invalid"), ThemeMode::Dark);
}

#[test]
fn theme_mode_dark_label() {
    assert_eq!(ThemeMode::Dark.label(), "Dark");
}

#[test]
fn theme_mode_light_label() {
    assert_eq!(ThemeMode::Light.label(), "Light");
}

#[test]
fn theme_mode_system_label() {
    assert_eq!(ThemeMode::System.label(), "System");
}

#[test]
fn theme_mode_dark_icon() {
    assert_eq!(ThemeMode::Dark.icon(), "moon");
}

#[test]
fn theme_mode_light_icon() {
    assert_eq!(ThemeMode::Light.icon(), "sun");
}

#[test]
fn theme_mode_system_icon() {
    assert_eq!(ThemeMode::System.icon(), "monitor");
}

#[test]
fn theme_mode_clone() {
    let mode = ThemeMode::Light;
    let cloned = mode.clone();
    assert_eq!(mode, cloned);
}

#[test]
fn theme_mode_equality() {
    assert_eq!(ThemeMode::Dark, ThemeMode::Dark);
    assert_eq!(ThemeMode::Light, ThemeMode::Light);
    assert_eq!(ThemeMode::System, ThemeMode::System);
    assert_ne!(ThemeMode::Dark, ThemeMode::Light);
    assert_ne!(ThemeMode::Light, ThemeMode::System);
    assert_ne!(ThemeMode::System, ThemeMode::Dark);
}

#[test]
fn theme_mode_debug_format() {
    assert_eq!(format!("{:?}", ThemeMode::Dark), "Dark");
    assert_eq!(format!("{:?}", ThemeMode::Light), "Light");
    assert_eq!(format!("{:?}", ThemeMode::System), "System");
}

// ============================================================================
// THEME CONTEXT TESTS
// ============================================================================

#[test]
fn theme_context_default_is_dark() {
    let ctx = ThemeContext::default();
    assert_eq!(ctx.mode, ThemeMode::Dark);
}

#[test]
fn theme_context_is_dark_true_when_dark() {
    let ctx = ThemeContext { mode: ThemeMode::Dark };
    assert!(ctx.is_dark());
    assert!(!ctx.is_light());
    assert!(!ctx.is_system());
}

#[test]
fn theme_context_is_light_true_when_light() {
    let ctx = ThemeContext { mode: ThemeMode::Light };
    assert!(!ctx.is_dark());
    assert!(ctx.is_light());
    assert!(!ctx.is_system());
}

#[test]
fn theme_context_is_system_true_when_system() {
    let ctx = ThemeContext { mode: ThemeMode::System };
    assert!(!ctx.is_dark());
    assert!(!ctx.is_light());
    assert!(ctx.is_system());
}

#[test]
fn theme_context_set_mode() {
    let mut ctx = ThemeContext::default();
    assert_eq!(ctx.mode, ThemeMode::Dark);

    ctx.set_mode(ThemeMode::Light);
    assert_eq!(ctx.mode, ThemeMode::Light);

    ctx.set_mode(ThemeMode::System);
    assert_eq!(ctx.mode, ThemeMode::System);

    ctx.set_mode(ThemeMode::Dark);
    assert_eq!(ctx.mode, ThemeMode::Dark);
}

#[test]
fn theme_context_toggle_dark_to_light() {
    let mut ctx = ThemeContext { mode: ThemeMode::Dark };
    ctx.toggle();
    assert_eq!(ctx.mode, ThemeMode::Light);
}

#[test]
fn theme_context_toggle_light_to_system() {
    let mut ctx = ThemeContext { mode: ThemeMode::Light };
    ctx.toggle();
    assert_eq!(ctx.mode, ThemeMode::System);
}

#[test]
fn theme_context_toggle_system_to_dark() {
    let mut ctx = ThemeContext { mode: ThemeMode::System };
    ctx.toggle();
    assert_eq!(ctx.mode, ThemeMode::Dark);
}

#[test]
fn theme_context_toggle_cycles_through_all_modes() {
    let mut ctx = ThemeContext::default();
    assert_eq!(ctx.mode, ThemeMode::Dark);

    ctx.toggle();
    assert_eq!(ctx.mode, ThemeMode::Light);

    ctx.toggle();
    assert_eq!(ctx.mode, ThemeMode::System);

    ctx.toggle();
    assert_eq!(ctx.mode, ThemeMode::Dark);
}

#[test]
fn theme_context_cycle_next_same_as_toggle() {
    let mut ctx1 = ThemeContext { mode: ThemeMode::Dark };
    let mut ctx2 = ThemeContext { mode: ThemeMode::Dark };

    ctx1.toggle();
    ctx2.cycle_next();

    assert_eq!(ctx1.mode, ctx2.mode);
}

#[test]
fn theme_context_data_theme_dark() {
    let ctx = ThemeContext { mode: ThemeMode::Dark };
    assert_eq!(ctx.data_theme(), "dark");
}

#[test]
fn theme_context_data_theme_light() {
    let ctx = ThemeContext { mode: ThemeMode::Light };
    assert_eq!(ctx.data_theme(), "light");
}

#[test]
fn theme_context_data_theme_system() {
    let ctx = ThemeContext { mode: ThemeMode::System };
    assert_eq!(ctx.data_theme(), "system");
}

// ============================================================================
// ROUNDTRIP TESTS
// ============================================================================

#[test]
fn theme_mode_roundtrip_dark() {
    let original = ThemeMode::Dark;
    let str_repr = original.as_str();
    let recovered = ThemeMode::from_str(str_repr);
    assert_eq!(original, recovered);
}

#[test]
fn theme_mode_roundtrip_light() {
    let original = ThemeMode::Light;
    let str_repr = original.as_str();
    let recovered = ThemeMode::from_str(str_repr);
    assert_eq!(original, recovered);
}

#[test]
fn theme_mode_roundtrip_system() {
    let original = ThemeMode::System;
    let str_repr = original.as_str();
    let recovered = ThemeMode::from_str(str_repr);
    assert_eq!(original, recovered);
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn theme_mode_from_str_case_sensitive() {
    // Should default to dark for wrong case
    assert_eq!(ThemeMode::from_str("Dark"), ThemeMode::Dark);
    assert_eq!(ThemeMode::from_str("LIGHT"), ThemeMode::Dark);
    assert_eq!(ThemeMode::from_str("SYSTEM"), ThemeMode::Dark);
}

#[test]
fn theme_mode_from_str_whitespace() {
    assert_eq!(ThemeMode::from_str(" dark"), ThemeMode::Dark);
    assert_eq!(ThemeMode::from_str("light "), ThemeMode::Dark);
    assert_eq!(ThemeMode::from_str(" system "), ThemeMode::Dark);
}
