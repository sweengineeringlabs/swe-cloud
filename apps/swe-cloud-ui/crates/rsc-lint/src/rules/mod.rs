//! # Lint Rules Module
//!
//! This module provides all lint rules for RustScript projects.
//! Rules are organized by category and can be individually enabled or disabled.
//!
//! ## Rule Categories
//!
//! - **i18n** - Internationalization rules (BP-005)
//! - **security** - Security-related rules (future)
//! - **accessibility** - Accessibility rules (future)
//! - **performance** - Performance optimization rules (future)
//!
//! ## Using Rules
//!
//! ```rust
//! use rsc_lint::{LintEngine, LintConfig};
//!
//! let config = LintConfig::default();
//! let engine = LintEngine::new(config);
//!
//! // Run all enabled rules
//! let diagnostics = engine.lint_file("src/main.rsx")?;
//! ```

pub mod i18n;

use std::path::Path;
use std::sync::Arc;

use crate::{LintDiagnostic, LintRule};
use crate::config::LintConfig;

// Re-export i18n rules
pub use i18n::{MissingTranslation, NoHardcodedStrings, UseTranslationKey};

/// Registry of all available lint rules.
///
/// The registry manages rule creation and provides access to rules
/// based on configuration settings.
#[derive(Debug)]
pub struct RuleRegistry {
    rules: Vec<Arc<dyn LintRule>>,
    disabled_rules: Vec<String>,
}

impl RuleRegistry {
    /// Create a new registry with the given configuration.
    pub fn new(config: &LintConfig) -> Self {
        let mut rules: Vec<Arc<dyn LintRule>> = Vec::new();

        // Register i18n rules if enabled
        if config.i18n.enabled {
            rules.push(Arc::new(NoHardcodedStrings::new(&config.i18n)));
            rules.push(Arc::new(UseTranslationKey::new(&config.i18n)));
            rules.push(Arc::new(MissingTranslation::new(&config.i18n)));
        }

        // Future: Register other rule categories here
        // if config.security.enabled { ... }
        // if config.accessibility.enabled { ... }

        Self {
            rules,
            disabled_rules: config.disabled_rules.clone(),
        }
    }

    /// Get all enabled rules.
    pub fn enabled_rules(&self) -> impl Iterator<Item = &Arc<dyn LintRule>> {
        self.rules
            .iter()
            .filter(|rule| !self.disabled_rules.contains(&rule.id().to_string()))
    }

    /// Get a rule by its ID.
    pub fn get_rule(&self, id: &str) -> Option<&Arc<dyn LintRule>> {
        self.rules.iter().find(|rule| rule.id() == id)
    }

    /// Check if a rule is enabled.
    pub fn is_rule_enabled(&self, id: &str) -> bool {
        !self.disabled_rules.contains(&id.to_string())
            && self.rules.iter().any(|rule| rule.id() == id)
    }

    /// Get all registered rule IDs.
    pub fn all_rule_ids(&self) -> Vec<&'static str> {
        self.rules.iter().map(|rule| rule.id()).collect()
    }

    /// Get all i18n rule IDs.
    pub fn i18n_rule_ids() -> Vec<&'static str> {
        vec!["I18N001", "I18N002", "I18N003"]
    }

    /// Run all enabled rules on a source file.
    pub fn check_file(&self, source: &str, file_path: &Path) -> Vec<LintDiagnostic> {
        let mut diagnostics = Vec::new();

        for rule in self.enabled_rules() {
            let rule_diagnostics = rule.check(source, file_path);
            diagnostics.extend(rule_diagnostics);
        }

        // Sort diagnostics by location for consistent output
        diagnostics.sort_by(|a, b| {
            a.location
                .line
                .cmp(&b.location.line)
                .then(a.location.column.cmp(&b.location.column))
        });

        diagnostics
    }
}

impl Default for RuleRegistry {
    fn default() -> Self {
        Self::new(&LintConfig::default())
    }
}

/// Rule category for organizing and filtering rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleCategory {
    /// Internationalization rules
    I18n,
    /// Security rules
    Security,
    /// Accessibility rules
    Accessibility,
    /// Performance rules
    Performance,
    /// Code style rules
    Style,
    /// Best practices
    BestPractices,
}

impl RuleCategory {
    /// Get the category for a rule ID.
    pub fn from_rule_id(id: &str) -> Option<Self> {
        if id.starts_with("I18N") {
            Some(Self::I18n)
        } else if id.starts_with("SEC") {
            Some(Self::Security)
        } else if id.starts_with("A11Y") {
            Some(Self::Accessibility)
        } else if id.starts_with("PERF") {
            Some(Self::Performance)
        } else if id.starts_with("STYLE") {
            Some(Self::Style)
        } else if id.starts_with("BP") {
            Some(Self::BestPractices)
        } else {
            None
        }
    }

    /// Get the display name for this category.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::I18n => "Internationalization",
            Self::Security => "Security",
            Self::Accessibility => "Accessibility",
            Self::Performance => "Performance",
            Self::Style => "Code Style",
            Self::BestPractices => "Best Practices",
        }
    }

    /// Get the short name for this category.
    pub fn short_name(&self) -> &'static str {
        match self {
            Self::I18n => "i18n",
            Self::Security => "security",
            Self::Accessibility => "a11y",
            Self::Performance => "perf",
            Self::Style => "style",
            Self::BestPractices => "bp",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let config = LintConfig::default();
        let registry = RuleRegistry::new(&config);

        // Should have all i18n rules
        assert!(registry.is_rule_enabled("I18N001"));
        assert!(registry.is_rule_enabled("I18N002"));
        assert!(registry.is_rule_enabled("I18N003"));
    }

    #[test]
    fn test_disabled_rules() {
        let mut config = LintConfig::default();
        config.disabled_rules.push("I18N001".to_string());

        let registry = RuleRegistry::new(&config);

        assert!(!registry.is_rule_enabled("I18N001"));
        assert!(registry.is_rule_enabled("I18N002"));
        assert!(registry.is_rule_enabled("I18N003"));
    }

    #[test]
    fn test_i18n_disabled() {
        let mut config = LintConfig::default();
        config.i18n.enabled = false;

        let registry = RuleRegistry::new(&config);

        assert!(!registry.is_rule_enabled("I18N001"));
        assert!(!registry.is_rule_enabled("I18N002"));
        assert!(!registry.is_rule_enabled("I18N003"));
    }

    #[test]
    fn test_get_rule() {
        let config = LintConfig::default();
        let registry = RuleRegistry::new(&config);

        let rule = registry.get_rule("I18N001");
        assert!(rule.is_some());
        assert_eq!(rule.unwrap().id(), "I18N001");

        let missing = registry.get_rule("UNKNOWN");
        assert!(missing.is_none());
    }

    #[test]
    fn test_all_rule_ids() {
        let config = LintConfig::default();
        let registry = RuleRegistry::new(&config);

        let ids = registry.all_rule_ids();
        assert!(ids.contains(&"I18N001"));
        assert!(ids.contains(&"I18N002"));
        assert!(ids.contains(&"I18N003"));
    }

    #[test]
    fn test_check_file() {
        let config = LintConfig::default();
        let registry = RuleRegistry::new(&config);

        let source = r#"<h1>"Hello World"</h1>"#;
        let diagnostics = registry.check_file(source, Path::new("test.rsx"));

        // Should detect hardcoded string
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn test_rule_category_from_id() {
        assert_eq!(
            RuleCategory::from_rule_id("I18N001"),
            Some(RuleCategory::I18n)
        );
        assert_eq!(
            RuleCategory::from_rule_id("SEC001"),
            Some(RuleCategory::Security)
        );
        assert_eq!(
            RuleCategory::from_rule_id("A11Y001"),
            Some(RuleCategory::Accessibility)
        );
        assert_eq!(RuleCategory::from_rule_id("UNKNOWN"), None);
    }

    #[test]
    fn test_rule_category_names() {
        assert_eq!(RuleCategory::I18n.display_name(), "Internationalization");
        assert_eq!(RuleCategory::I18n.short_name(), "i18n");
    }

    #[test]
    fn test_enabled_rules_iterator() {
        let mut config = LintConfig::default();
        config.disabled_rules.push("I18N002".to_string());

        let registry = RuleRegistry::new(&config);
        let enabled: Vec<_> = registry.enabled_rules().collect();

        // Should have 2 rules (I18N001 and I18N003)
        assert_eq!(enabled.len(), 2);
        assert!(enabled.iter().any(|r| r.id() == "I18N001"));
        assert!(enabled.iter().any(|r| r.id() == "I18N003"));
        assert!(!enabled.iter().any(|r| r.id() == "I18N002"));
    }
}
