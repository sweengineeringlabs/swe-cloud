//! # RustScript Lint (rsc-lint)
//!
//! A comprehensive linting framework for RustScript projects, providing
//! static analysis rules for code quality, internationalization, security,
//! and accessibility.
//!
//! ## Features
//!
//! - **Internationalization (i18n)** - Enforce proper translation key usage (BP-005)
//! - **Extensible** - Easy to add new rule categories and individual rules
//! - **Configurable** - Fine-grained control over which rules are enabled
//! - **Advocacy Messages** - Educational messages explaining why rules matter
//!
//! ## Quick Start
//!
//! ```rust
//! use rsc_lint::{LintEngine, LintConfig};
//!
//! // Create engine with default configuration
//! let engine = LintEngine::new(LintConfig::default());
//!
//! // Lint a source file
//! let source = r#"<h1>"Hello World"</h1>"#;
//! let diagnostics = engine.lint_source(source, "test.rsx");
//!
//! for diagnostic in diagnostics {
//!     println!("{}: {}", diagnostic.rule_id, diagnostic.message);
//! }
//! ```
//!
//! ## Configuration
//!
//! The linter can be configured via `rsc.toml`:
//!
//! ```toml
//! [lint]
//! enabled = true
//! strict = false
//!
//! [lint.i18n]
//! enabled = true
//! locale_dir = "locales"
//! allowlist = ["MyBrand", "RustScript"]
//! ```
//!
//! ## Rule Categories
//!
//! | Category | Prefix | Description |
//! |----------|--------|-------------|
//! | i18n | I18N | Internationalization rules |
//! | Security | SEC | Security vulnerability detection |
//! | Accessibility | A11Y | Accessibility compliance |
//! | Performance | PERF | Performance optimization |

#![warn(missing_docs)]
#![deny(unsafe_code)]

pub mod config;
pub mod rules;

use std::fmt;
use std::path::{Path, PathBuf};

pub use config::{I18nConfig, LintConfig};
pub use rules::{MissingTranslation, NoHardcodedStrings, RuleRegistry, UseTranslationKey};

/// Severity level for lint diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    /// Informational message, not a problem.
    Info,
    /// Warning that should be addressed but doesn't block.
    Warning,
    /// Error that should be fixed before deployment.
    Error,
}

impl Severity {
    /// Get the display string for this severity.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    /// Check if this severity is at least as severe as another.
    pub fn is_at_least(&self, other: Self) -> bool {
        match (self, other) {
            (Self::Error, _) => true,
            (Self::Warning, Self::Error) => false,
            (Self::Warning, _) => true,
            (Self::Info, Self::Info) => true,
            (Self::Info, _) => false,
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Location of an issue in source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    /// File path.
    pub file: PathBuf,
    /// Line number (1-indexed).
    pub line: usize,
    /// Column number (1-indexed).
    pub column: usize,
    /// End line number (1-indexed).
    pub end_line: usize,
    /// End column number (1-indexed).
    pub end_column: usize,
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self {
            file: PathBuf::new(),
            line: 1,
            column: 1,
            end_line: 1,
            end_column: 1,
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.file.display(),
            self.line,
            self.column
        )
    }
}

/// A diagnostic message from a lint rule.
#[derive(Debug, Clone)]
pub struct LintDiagnostic {
    /// Rule ID (e.g., "I18N001").
    pub rule_id: String,
    /// Rule name (e.g., "no-hardcoded-strings").
    pub rule_name: String,
    /// Severity of the issue.
    pub severity: Severity,
    /// Human-readable message describing the issue.
    pub message: String,
    /// Location in the source code.
    pub location: SourceLocation,
    /// Suggested fix (if available).
    pub suggestion: Option<String>,
    /// Advocacy message explaining why this rule matters.
    pub advocacy_message: Option<String>,
}

impl LintDiagnostic {
    /// Create a new diagnostic.
    pub fn new(
        rule_id: impl Into<String>,
        rule_name: impl Into<String>,
        severity: Severity,
        message: impl Into<String>,
        location: SourceLocation,
    ) -> Self {
        Self {
            rule_id: rule_id.into(),
            rule_name: rule_name.into(),
            severity,
            message: message.into(),
            location,
            suggestion: None,
            advocacy_message: None,
        }
    }

    /// Add a suggestion to the diagnostic.
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Add an advocacy message to the diagnostic.
    pub fn with_advocacy(mut self, message: impl Into<String>) -> Self {
        self.advocacy_message = Some(message.into());
        self
    }
}

impl fmt::Display for LintDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {} [{}]",
            self.severity, self.location, self.message, self.rule_id
        )
    }
}

/// Trait for implementing lint rules.
///
/// Each rule must implement this trait to be registered in the linter.
///
/// ## Example
///
/// ```rust
/// use rsc_lint::{LintRule, LintDiagnostic, Severity, SourceLocation};
/// use std::path::Path;
///
/// struct MyRule;
///
/// impl LintRule for MyRule {
///     fn id(&self) -> &'static str { "MY001" }
///     fn name(&self) -> &'static str { "my-rule" }
///     fn severity(&self) -> Severity { Severity::Warning }
///     fn description(&self) -> &'static str { "Description of my rule" }
///
///     fn check(&self, source: &str, file_path: &Path) -> Vec<LintDiagnostic> {
///         // Implementation here
///         Vec::new()
///     }
/// }
/// ```
pub trait LintRule: Send + Sync + std::fmt::Debug {
    /// Unique identifier for the rule (e.g., "I18N001").
    fn id(&self) -> &'static str;

    /// Human-readable name (e.g., "no-hardcoded-strings").
    fn name(&self) -> &'static str;

    /// Default severity for violations.
    fn severity(&self) -> Severity;

    /// Description of what the rule checks.
    fn description(&self) -> &'static str;

    /// Check a source file for violations.
    ///
    /// Returns a list of diagnostics for any issues found.
    fn check(&self, source: &str, file_path: &Path) -> Vec<LintDiagnostic>;
}

/// Main lint engine for running rules against source files.
pub struct LintEngine {
    config: LintConfig,
    registry: RuleRegistry,
}

impl LintEngine {
    /// Create a new lint engine with the given configuration.
    pub fn new(config: LintConfig) -> Self {
        let registry = RuleRegistry::new(&config);
        Self { config, registry }
    }

    /// Get the current configuration.
    pub fn config(&self) -> &LintConfig {
        &self.config
    }

    /// Get the rule registry.
    pub fn registry(&self) -> &RuleRegistry {
        &self.registry
    }

    /// Lint a source string.
    pub fn lint_source(&self, source: &str, file_name: &str) -> Vec<LintDiagnostic> {
        if !self.config.enabled {
            return Vec::new();
        }

        let file_path = Path::new(file_name);

        // Check if file should be excluded
        if self.should_exclude(file_name) {
            return Vec::new();
        }

        self.registry.check_file(source, file_path)
    }

    /// Lint a file from disk.
    pub fn lint_file(&self, file_path: &Path) -> Result<Vec<LintDiagnostic>, std::io::Error> {
        if !self.config.enabled {
            return Ok(Vec::new());
        }

        // Check if file should be excluded
        let file_name = file_path.to_string_lossy();
        if self.should_exclude(&file_name) {
            return Ok(Vec::new());
        }

        let source = std::fs::read_to_string(file_path)?;
        Ok(self.registry.check_file(&source, file_path))
    }

    /// Check if a file should be excluded from linting.
    fn should_exclude(&self, file_name: &str) -> bool {
        for pattern in &self.config.exclude_patterns {
            if self.matches_glob(file_name, pattern) {
                return true;
            }
        }
        false
    }

    /// Simple glob matching.
    fn matches_glob(&self, file_name: &str, pattern: &str) -> bool {
        if pattern.contains("**") {
            // Recursive directory match
            let parts: Vec<&str> = pattern.split("**").collect();
            if parts.len() == 2 {
                let prefix = parts[0].trim_end_matches('/');
                let suffix = parts[1].trim_start_matches('/');

                if prefix.is_empty() && suffix.is_empty() {
                    return true;
                }

                if !prefix.is_empty() && !file_name.contains(prefix) {
                    return false;
                }

                if !suffix.is_empty() {
                    return file_name.ends_with(suffix);
                }

                return file_name.contains(prefix);
            }
        } else if pattern.starts_with('*') {
            // Suffix match
            let suffix = &pattern[1..];
            return file_name.ends_with(suffix);
        } else if pattern.ends_with('*') {
            // Prefix match
            let prefix = &pattern[..pattern.len() - 1];
            return file_name.starts_with(prefix);
        }

        file_name == pattern
    }

    /// Get summary statistics for diagnostics.
    pub fn summarize(diagnostics: &[LintDiagnostic]) -> LintSummary {
        let mut summary = LintSummary::default();

        for diagnostic in diagnostics {
            match diagnostic.severity {
                Severity::Error => summary.errors += 1,
                Severity::Warning => summary.warnings += 1,
                Severity::Info => summary.infos += 1,
            }
        }

        summary.total = diagnostics.len();
        summary
    }
}

/// Summary statistics for lint results.
#[derive(Debug, Clone, Default)]
pub struct LintSummary {
    /// Total number of diagnostics.
    pub total: usize,
    /// Number of errors.
    pub errors: usize,
    /// Number of warnings.
    pub warnings: usize,
    /// Number of informational messages.
    pub infos: usize,
}

impl fmt::Display for LintSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} errors, {} warnings, {} infos ({} total)",
            self.errors, self.warnings, self.infos, self.total
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_display() {
        assert_eq!(Severity::Error.as_str(), "error");
        assert_eq!(Severity::Warning.as_str(), "warning");
        assert_eq!(Severity::Info.as_str(), "info");
    }

    #[test]
    fn test_severity_comparison() {
        assert!(Severity::Error.is_at_least(Severity::Warning));
        assert!(Severity::Error.is_at_least(Severity::Info));
        assert!(Severity::Warning.is_at_least(Severity::Warning));
        assert!(Severity::Warning.is_at_least(Severity::Info));
        assert!(!Severity::Info.is_at_least(Severity::Warning));
    }

    #[test]
    fn test_source_location_display() {
        let location = SourceLocation {
            file: PathBuf::from("test.rsx"),
            line: 10,
            column: 5,
            end_line: 10,
            end_column: 20,
        };
        assert_eq!(format!("{}", location), "test.rsx:10:5");
    }

    #[test]
    fn test_lint_diagnostic_display() {
        let diagnostic = LintDiagnostic::new(
            "I18N001",
            "no-hardcoded-strings",
            Severity::Warning,
            "Hardcoded string found",
            SourceLocation {
                file: PathBuf::from("test.rsx"),
                line: 5,
                column: 10,
                end_line: 5,
                end_column: 25,
            },
        );
        let display = format!("{}", diagnostic);
        assert!(display.contains("warning"));
        assert!(display.contains("Hardcoded string found"));
        assert!(display.contains("I18N001"));
    }

    #[test]
    fn test_lint_engine_disabled() {
        let mut config = LintConfig::default();
        config.enabled = false;

        let engine = LintEngine::new(config);
        let diagnostics = engine.lint_source(r#"<h1>"Test"</h1>"#, "test.rsx");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn test_lint_engine_excludes() {
        let config = LintConfig::default();
        let engine = LintEngine::new(config);

        // Test exclusion patterns
        assert!(engine.should_exclude("node_modules/test.rsx"));
        assert!(engine.should_exclude("dist/main.rsx"));
        assert!(engine.should_exclude("component.test.rsx"));
        assert!(!engine.should_exclude("src/main.rsx"));
    }

    #[test]
    fn test_lint_engine_basic() {
        let config = LintConfig::default();
        let engine = LintEngine::new(config);

        let source = r#"<h1>"Hello World"</h1>"#;
        let diagnostics = engine.lint_source(source, "test.rsx");

        // Should detect hardcoded string
        assert!(!diagnostics.is_empty());
    }

    #[test]
    fn test_lint_summary() {
        let diagnostics = vec![
            LintDiagnostic::new(
                "I18N001",
                "test",
                Severity::Error,
                "Error",
                SourceLocation::default(),
            ),
            LintDiagnostic::new(
                "I18N002",
                "test",
                Severity::Warning,
                "Warning",
                SourceLocation::default(),
            ),
            LintDiagnostic::new(
                "I18N003",
                "test",
                Severity::Info,
                "Info",
                SourceLocation::default(),
            ),
        ];

        let summary = LintEngine::summarize(&diagnostics);
        assert_eq!(summary.total, 3);
        assert_eq!(summary.errors, 1);
        assert_eq!(summary.warnings, 1);
        assert_eq!(summary.infos, 1);
    }

    #[test]
    fn test_glob_matching() {
        let engine = LintEngine::new(LintConfig::default());

        // ** patterns
        assert!(engine.matches_glob("node_modules/test.js", "node_modules/**"));
        assert!(engine.matches_glob("dist/bundle.js", "dist/**"));

        // * patterns
        assert!(engine.matches_glob("test.test.rsx", "*.test.rsx"));
        assert!(engine.matches_glob("main.test.rsx", "*.test.rsx"));
        assert!(!engine.matches_glob("main.rsx", "*.test.rsx"));
    }
}
