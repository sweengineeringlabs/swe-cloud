//! # Configuration Types for RustScript Lint
//!
//! This module provides configuration structures for the linting system,
//! including rule-specific configurations and global lint settings.

use serde::{Deserialize, Serialize};

/// Main configuration for the linting system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintConfig {
    /// Whether linting is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Enable strict mode (treat warnings as errors)
    #[serde(default)]
    pub strict: bool,

    /// Internationalization configuration
    #[serde(default)]
    pub i18n: I18nConfig,

    /// List of rule IDs to disable
    #[serde(default)]
    pub disabled_rules: Vec<String>,

    /// List of file patterns to exclude from linting
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
}

impl Default for LintConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strict: false,
            i18n: I18nConfig::default(),
            disabled_rules: Vec::new(),
            exclude_patterns: vec![
                "node_modules/**".to_string(),
                "dist/**".to_string(),
                "*.test.rsx".to_string(),
            ],
        }
    }
}

/// Configuration for internationalization (i18n) lint rules.
///
/// This configuration controls how the i18n rules behave, including
/// which strings are allowed, where locale files are located, and
/// how strict the validation should be.
///
/// ## Example Configuration (rsc.toml)
///
/// ```toml
/// [lint.i18n]
/// enabled = true
/// strict = true
/// locale_dir = "locales"
/// primary_locale = "en"
/// check_all_locales = true
///
/// [lint.i18n.allowlist]
/// strings = ["SWE Cloud", "RustScript", "WebAssembly"]
/// patterns = ["Version*", "*Copyright*"]
///
/// [lint.i18n.translation]
/// function = "t"
/// alternatives = ["trans", "translate"]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct I18nConfig {
    /// Whether i18n rules are enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Enable strict mode for i18n rules
    ///
    /// In strict mode:
    /// - Missing translations are errors (not warnings)
    /// - All strings must use translation keys (no exceptions beyond allowlist)
    #[serde(default)]
    pub strict: bool,

    /// Directory containing locale files (e.g., "locales" or "src/i18n")
    ///
    /// Locale files should be JSON format with the locale code as filename:
    /// - `en.json` - English translations
    /// - `es.json` - Spanish translations
    /// - etc.
    #[serde(default)]
    pub locale_dir: Option<String>,

    /// List of strings that don't require translation
    ///
    /// Use this for:
    /// - Brand names (e.g., "SWE Cloud", "GitHub")
    /// - Technical terms that shouldn't be translated
    /// - Abbreviations
    #[serde(default)]
    pub allowlist: Vec<String>,

    /// Pattern-based allowlist for strings
    ///
    /// Supports simple glob patterns:
    /// - `Version*` - Matches "Version 1.0", "Version 2.0", etc.
    /// - `*Copyright*` - Matches anything containing "Copyright"
    /// - `*.js` - Matches file extensions
    #[serde(default)]
    pub allowlist_patterns: Vec<String>,

    /// The translation function name to look for (default: "t")
    ///
    /// This is the function used to translate strings:
    /// - `t("key")` - Using the default
    /// - `translate("key")` - Using a custom function
    #[serde(default)]
    pub translation_function: Option<String>,

    /// Alternative function names that are also acceptable
    ///
    /// If your codebase uses multiple translation functions, list them here.
    /// The primary function is defined in `translation_function`.
    #[serde(default)]
    pub alternative_functions: Vec<String>,

    /// Primary locale for missing translation checks (default: "en")
    ///
    /// This locale is always checked for missing translations.
    #[serde(default)]
    pub primary_locale: Option<String>,

    /// Whether to check all locales for missing translations
    ///
    /// When true, the linter will verify that translation keys exist
    /// in all locale files, not just the primary locale.
    #[serde(default)]
    pub check_all_locales: bool,
}

impl Default for I18nConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strict: false,
            locale_dir: None,
            allowlist: vec![],
            allowlist_patterns: vec![],
            translation_function: Some("t".to_string()),
            alternative_functions: vec![],
            primary_locale: Some("en".to_string()),
            check_all_locales: false,
        }
    }
}

impl I18nConfig {
    /// Create a new I18nConfig with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a builder for I18nConfig.
    pub fn builder() -> I18nConfigBuilder {
        I18nConfigBuilder::default()
    }

    /// Check if a specific rule is enabled.
    pub fn is_rule_enabled(&self, rule_id: &str) -> bool {
        if !self.enabled {
            return false;
        }
        // All i18n rules are enabled if i18n is enabled
        // Individual rule disabling is handled at the LintConfig level
        matches!(rule_id, "I18N001" | "I18N002" | "I18N003")
    }
}

/// Builder for I18nConfig.
#[derive(Debug, Default)]
pub struct I18nConfigBuilder {
    enabled: Option<bool>,
    strict: Option<bool>,
    locale_dir: Option<String>,
    allowlist: Vec<String>,
    allowlist_patterns: Vec<String>,
    translation_function: Option<String>,
    alternative_functions: Vec<String>,
    primary_locale: Option<String>,
    check_all_locales: Option<bool>,
}

impl I18nConfigBuilder {
    /// Enable or disable i18n rules.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Enable or disable strict mode.
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = Some(strict);
        self
    }

    /// Set the locale directory.
    pub fn locale_dir(mut self, dir: impl Into<String>) -> Self {
        self.locale_dir = Some(dir.into());
        self
    }

    /// Add a string to the allowlist.
    pub fn allow_string(mut self, s: impl Into<String>) -> Self {
        self.allowlist.push(s.into());
        self
    }

    /// Add multiple strings to the allowlist.
    pub fn allow_strings(mut self, strings: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.allowlist.extend(strings.into_iter().map(Into::into));
        self
    }

    /// Add a pattern to the allowlist.
    pub fn allow_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.allowlist_patterns.push(pattern.into());
        self
    }

    /// Set the translation function name.
    pub fn translation_function(mut self, func: impl Into<String>) -> Self {
        self.translation_function = Some(func.into());
        self
    }

    /// Add an alternative translation function.
    pub fn alternative_function(mut self, func: impl Into<String>) -> Self {
        self.alternative_functions.push(func.into());
        self
    }

    /// Set the primary locale.
    pub fn primary_locale(mut self, locale: impl Into<String>) -> Self {
        self.primary_locale = Some(locale.into());
        self
    }

    /// Enable or disable checking all locales.
    pub fn check_all_locales(mut self, check: bool) -> Self {
        self.check_all_locales = Some(check);
        self
    }

    /// Build the I18nConfig.
    pub fn build(self) -> I18nConfig {
        let default = I18nConfig::default();
        I18nConfig {
            enabled: self.enabled.unwrap_or(default.enabled),
            strict: self.strict.unwrap_or(default.strict),
            locale_dir: self.locale_dir.or(default.locale_dir),
            allowlist: if self.allowlist.is_empty() {
                default.allowlist
            } else {
                self.allowlist
            },
            allowlist_patterns: if self.allowlist_patterns.is_empty() {
                default.allowlist_patterns
            } else {
                self.allowlist_patterns
            },
            translation_function: self.translation_function.or(default.translation_function),
            alternative_functions: if self.alternative_functions.is_empty() {
                default.alternative_functions
            } else {
                self.alternative_functions
            },
            primary_locale: self.primary_locale.or(default.primary_locale),
            check_all_locales: self.check_all_locales.unwrap_or(default.check_all_locales),
        }
    }
}

/// Default function returning true for serde defaults.
fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_lint_config() {
        let config = LintConfig::default();
        assert!(config.enabled);
        assert!(!config.strict);
        assert!(config.i18n.enabled);
        assert!(config.disabled_rules.is_empty());
    }

    #[test]
    fn test_default_i18n_config() {
        let config = I18nConfig::default();
        assert!(config.enabled);
        assert!(!config.strict);
        assert!(config.locale_dir.is_none());
        assert!(config.allowlist.is_empty());
        assert_eq!(config.translation_function, Some("t".to_string()));
        assert_eq!(config.primary_locale, Some("en".to_string()));
        assert!(!config.check_all_locales);
    }

    #[test]
    fn test_i18n_config_builder() {
        let config = I18nConfig::builder()
            .enabled(true)
            .strict(true)
            .locale_dir("locales")
            .allow_string("MyBrand")
            .allow_pattern("Version*")
            .translation_function("translate")
            .alternative_function("t")
            .primary_locale("es")
            .check_all_locales(true)
            .build();

        assert!(config.enabled);
        assert!(config.strict);
        assert_eq!(config.locale_dir, Some("locales".to_string()));
        assert!(config.allowlist.contains(&"MyBrand".to_string()));
        assert!(config.allowlist_patterns.contains(&"Version*".to_string()));
        assert_eq!(config.translation_function, Some("translate".to_string()));
        assert!(config.alternative_functions.contains(&"t".to_string()));
        assert_eq!(config.primary_locale, Some("es".to_string()));
        assert!(config.check_all_locales);
    }

    #[test]
    fn test_is_rule_enabled() {
        let config = I18nConfig::default();
        assert!(config.is_rule_enabled("I18N001"));
        assert!(config.is_rule_enabled("I18N002"));
        assert!(config.is_rule_enabled("I18N003"));
        assert!(!config.is_rule_enabled("OTHER001"));

        let disabled_config = I18nConfig::builder().enabled(false).build();
        assert!(!disabled_config.is_rule_enabled("I18N001"));
    }

    #[test]
    fn test_serde_serialization() {
        let config = I18nConfig::builder()
            .locale_dir("i18n")
            .allow_string("Test")
            .build();

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: I18nConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.locale_dir, deserialized.locale_dir);
        assert_eq!(config.allowlist, deserialized.allowlist);
    }

    #[test]
    fn test_lint_config_exclude_patterns() {
        let config = LintConfig::default();
        assert!(config.exclude_patterns.contains(&"node_modules/**".to_string()));
        assert!(config.exclude_patterns.contains(&"dist/**".to_string()));
        assert!(config.exclude_patterns.contains(&"*.test.rsx".to_string()));
    }
}
