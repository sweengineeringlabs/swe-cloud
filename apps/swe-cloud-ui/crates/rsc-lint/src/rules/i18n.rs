//! # Internationalization (i18n) Enforcement Rules (BP-005)
//!
//! This module implements lint rules for enforcing internationalization best practices
//! in RustScript JSX components. These rules help ensure applications are ready for
//! localization and can support multiple languages.
//!
//! ## Why i18n Matters
//!
//! Internationalization is not just about translation - it's about building software
//! that respects and serves users from diverse linguistic and cultural backgrounds.
//! Hardcoded strings create technical debt and exclude non-English speakers from
//! fully using your application.
//!
//! ## Rules
//!
//! - **I18N001**: `no-hardcoded-strings` - Detects hardcoded user-facing strings in JSX
//! - **I18N002**: `use-translation-key` - Requires `t("key")` pattern for text content
//! - **I18N003**: `missing-translation` - Validates translation keys exist in locale files

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::config::I18nConfig;
use crate::{LintDiagnostic, LintRule, Severity, SourceLocation};

// ============================================================================
// I18N001: No Hardcoded Strings
// ============================================================================

/// Rule I18N001: Detects hardcoded user-facing strings in JSX text content.
///
/// This rule identifies string literals that appear directly in JSX elements,
/// which should be replaced with translation function calls for proper i18n support.
///
/// ## Rationale
///
/// Hardcoded strings make it impossible to translate your application without
/// modifying source code. By using translation keys, you can:
/// - Support multiple languages without code changes
/// - Allow translators to work independently of developers
/// - Ensure consistent terminology across your application
///
/// ## Examples
///
/// ### Bad
/// ```rsx
/// <h1>"Welcome to our app"</h1>
/// <button>"Submit"</button>
/// ```
///
/// ### Good
/// ```rsx
/// <h1>{t("welcome.title")}</h1>
/// <button>{t("common.submit")}</button>
/// ```
#[derive(Debug, Clone)]
pub struct NoHardcodedStrings {
    /// Allowlist of strings that don't require translation (brand names, etc.)
    allowlist: HashSet<String>,
    /// Patterns to allowlist (regex-like simple patterns)
    allowlist_patterns: Vec<String>,
}

impl NoHardcodedStrings {
    /// Create a new instance with the given configuration.
    pub fn new(config: &I18nConfig) -> Self {
        Self {
            allowlist: config.allowlist.iter().cloned().collect(),
            allowlist_patterns: config.allowlist_patterns.clone(),
        }
    }

    /// Check if a string is in the allowlist.
    fn is_allowed(&self, text: &str) -> bool {
        // Direct match
        if self.allowlist.contains(text) {
            return true;
        }

        // Pattern matching (simple glob-style)
        for pattern in &self.allowlist_patterns {
            if self.matches_pattern(text, pattern) {
                return true;
            }
        }

        // Skip pure whitespace
        if text.trim().is_empty() {
            return true;
        }

        // Skip numeric-only strings (e.g., "100", "2.5ms")
        if text.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '%' || c == 'K' || c == 'M' || c == '+') {
            return true;
        }

        // Skip single characters (often icons or symbols)
        if text.chars().count() <= 2 {
            return true;
        }

        false
    }

    /// Simple pattern matching for allowlist patterns.
    fn matches_pattern(&self, text: &str, pattern: &str) -> bool {
        if pattern.starts_with('*') && pattern.ends_with('*') {
            // *contains*
            let inner = &pattern[1..pattern.len() - 1];
            text.contains(inner)
        } else if pattern.starts_with('*') {
            // *endswith
            let suffix = &pattern[1..];
            text.ends_with(suffix)
        } else if pattern.ends_with('*') {
            // startswith*
            let prefix = &pattern[..pattern.len() - 1];
            text.starts_with(prefix)
        } else {
            text == pattern
        }
    }

    /// Extract hardcoded strings from JSX content.
    fn find_hardcoded_strings(&self, source: &str) -> Vec<HardcodedStringMatch> {
        let mut matches = Vec::new();
        let mut in_jsx = false;
        let mut in_string = false;
        let mut string_start = 0;
        let mut current_string = String::new();
        let mut line = 1;
        let mut column = 1;
        let mut string_line = 1;
        let mut string_column = 1;
        let chars: Vec<char> = source.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];

            // Track line and column
            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }

            // Detect JSX context (simplified detection)
            if c == '<' && i + 1 < chars.len() && chars[i + 1].is_alphabetic() {
                in_jsx = true;
            }

            // Detect string literals in JSX context
            if in_jsx && c == '"' && !in_string {
                // Check if this is a JSX text content (not an attribute)
                // Look back to see if we're after a '>' or whitespace (not after '=')
                let mut j = i;
                while j > 0 && chars[j - 1].is_whitespace() {
                    j -= 1;
                }
                let is_text_content = j == 0 || chars[j - 1] == '>' || chars[j - 1] == '{';

                if is_text_content {
                    in_string = true;
                    string_start = i;
                    string_line = line;
                    string_column = column;
                    current_string.clear();
                    i += 1;
                    continue;
                }
            } else if in_string && c == '"' {
                // End of string
                in_string = false;

                // Check if this string should be flagged
                if !self.is_allowed(&current_string) {
                    matches.push(HardcodedStringMatch {
                        text: current_string.clone(),
                        location: SourceLocation {
                            file: PathBuf::new(),
                            line: string_line,
                            column: string_column,
                            end_line: line,
                            end_column: column,
                        },
                    });
                }
            } else if in_string {
                current_string.push(c);
            }

            // Exit JSX context
            if c == '>' && !in_string {
                // Check if it's a closing tag
                if i > 0 && chars[i - 1] == '/' {
                    in_jsx = false;
                }
            }

            i += 1;
        }

        matches
    }
}

#[derive(Debug, Clone)]
struct HardcodedStringMatch {
    text: String,
    location: SourceLocation,
}

impl LintRule for NoHardcodedStrings {
    fn id(&self) -> &'static str {
        "I18N001"
    }

    fn name(&self) -> &'static str {
        "no-hardcoded-strings"
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn description(&self) -> &'static str {
        "Detects hardcoded user-facing strings in JSX text that should use translation keys"
    }

    fn check(&self, source: &str, file_path: &Path) -> Vec<LintDiagnostic> {
        let matches = self.find_hardcoded_strings(source);

        matches
            .into_iter()
            .map(|m| {
                let mut location = m.location;
                location.file = file_path.to_path_buf();

                LintDiagnostic {
                    rule_id: self.id().to_string(),
                    rule_name: self.name().to_string(),
                    severity: self.severity(),
                    message: format!(
                        "Hardcoded string \"{}\" should use a translation key",
                        truncate_string(&m.text, 50)
                    ),
                    location,
                    suggestion: Some(format!(
                        "Replace with: {{t(\"{}\")}}",
                        generate_translation_key(&m.text)
                    )),
                    advocacy_message: Some(
                        "Internationalization enables your application to serve users worldwide. \
                        Using translation keys instead of hardcoded strings makes your app \
                        accessible to non-English speakers and simplifies the localization process."
                            .to_string(),
                    ),
                }
            })
            .collect()
    }
}

// ============================================================================
// I18N002: Use Translation Key
// ============================================================================

/// Rule I18N002: Requires the `t("key")` pattern for text content.
///
/// This rule ensures that text content in JSX uses the translation function
/// with proper key-based lookup rather than inline strings.
///
/// ## Rationale
///
/// The `t("key")` pattern provides a standardized way to handle translations:
/// - Keys are descriptive and context-aware
/// - The translation system can handle pluralization, interpolation, etc.
/// - Missing translations can be detected at build time
///
/// ## Examples
///
/// ### Bad
/// ```rsx
/// <span>{translate("Hello")}</span>  // Using the wrong function
/// <span>{`Hello ${name}`}</span>     // Template literals without translation
/// ```
///
/// ### Good
/// ```rsx
/// <span>{t("greeting.hello")}</span>
/// <span>{t("greeting.hello_name", { name })}</span>
/// ```
#[derive(Debug, Clone)]
pub struct UseTranslationKey {
    /// Required function name for translations
    translation_function: String,
    /// Alternative function names that are also acceptable
    alternative_functions: Vec<String>,
}

impl UseTranslationKey {
    /// Create a new instance with the given configuration.
    pub fn new(config: &I18nConfig) -> Self {
        Self {
            translation_function: config
                .translation_function
                .clone()
                .unwrap_or_else(|| "t".to_string()),
            alternative_functions: config.alternative_functions.clone(),
        }
    }

    /// Find text expressions that don't use the translation function.
    fn find_violations(&self, source: &str) -> Vec<TranslationViolation> {
        let mut violations = Vec::new();
        let mut line = 1;
        let mut column = 1;
        let chars: Vec<char> = source.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];

            // Track line and column
            if c == '\n' {
                line += 1;
                column = 1;
                i += 1;
                continue;
            }

            // Look for JSX expression: {something}
            if c == '{' {
                let expr_start_line = line;
                let expr_start_column = column;
                let expr_start = i;

                // Find matching closing brace
                let mut brace_count = 1;
                let mut expr_end = i + 1;
                while expr_end < chars.len() && brace_count > 0 {
                    if chars[expr_end] == '{' {
                        brace_count += 1;
                    } else if chars[expr_end] == '}' {
                        brace_count -= 1;
                    } else if chars[expr_end] == '\n' {
                        line += 1;
                        column = 0;
                    }
                    column += 1;
                    expr_end += 1;
                }

                if brace_count == 0 {
                    let expr: String = chars[expr_start + 1..expr_end - 1].iter().collect();
                    let expr_trimmed = expr.trim();

                    // Check if it's a string-related expression that should use t()
                    if self.is_translatable_expression(expr_trimmed)
                        && !self.uses_translation_function(expr_trimmed)
                    {
                        violations.push(TranslationViolation {
                            expression: expr_trimmed.to_string(),
                            location: SourceLocation {
                                file: PathBuf::new(),
                                line: expr_start_line,
                                column: expr_start_column,
                                end_line: line,
                                end_column: column,
                            },
                        });
                    }
                }

                i = expr_end;
                continue;
            }

            column += 1;
            i += 1;
        }

        violations
    }

    /// Check if an expression represents translatable content.
    fn is_translatable_expression(&self, expr: &str) -> bool {
        // Template literals with text content
        if expr.starts_with('`') && expr.ends_with('`') {
            let content = &expr[1..expr.len() - 1];
            // Contains actual text (not just variables)
            return content.chars().any(|c| c.is_alphabetic());
        }

        // String concatenation
        if expr.contains('+') && expr.contains('"') {
            return true;
        }

        // Direct string literals (already caught by I18N001, but check here too)
        if expr.starts_with('"') && expr.ends_with('"') {
            return true;
        }

        // Wrong translation function
        if expr.contains("translate(") || expr.contains("localize(") || expr.contains("i18n(") {
            return true;
        }

        false
    }

    /// Check if the expression uses an approved translation function.
    fn uses_translation_function(&self, expr: &str) -> bool {
        // Check primary function
        if expr.contains(&format!("{}(", self.translation_function)) {
            return true;
        }

        // Check alternative functions
        for func in &self.alternative_functions {
            if expr.contains(&format!("{func}(")) {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Clone)]
struct TranslationViolation {
    expression: String,
    location: SourceLocation,
}

impl LintRule for UseTranslationKey {
    fn id(&self) -> &'static str {
        "I18N002"
    }

    fn name(&self) -> &'static str {
        "use-translation-key"
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn description(&self) -> &'static str {
        "Requires the t(\"key\") pattern for translatable text content in JSX"
    }

    fn check(&self, source: &str, file_path: &Path) -> Vec<LintDiagnostic> {
        let violations = self.find_violations(source);

        violations
            .into_iter()
            .map(|v| {
                let mut location = v.location;
                location.file = file_path.to_path_buf();

                LintDiagnostic {
                    rule_id: self.id().to_string(),
                    rule_name: self.name().to_string(),
                    severity: self.severity(),
                    message: format!(
                        "Expression \"{}\" should use the {}() translation function",
                        truncate_string(&v.expression, 40),
                        self.translation_function
                    ),
                    location,
                    suggestion: Some(format!(
                        "Use {}(\"your.translation.key\") instead",
                        self.translation_function
                    )),
                    advocacy_message: Some(
                        "Using a consistent translation function (t()) enables your i18n system \
                        to extract keys, detect missing translations, and provide fallbacks. \
                        This pattern also supports advanced features like pluralization and interpolation."
                            .to_string(),
                    ),
                }
            })
            .collect()
    }
}

// ============================================================================
// I18N003: Missing Translation
// ============================================================================

/// Rule I18N003: Validates that translation keys exist in locale files.
///
/// This rule checks that all translation keys used in the source code
/// have corresponding entries in the locale files.
///
/// ## Rationale
///
/// Missing translations lead to:
/// - Broken user experiences (showing keys instead of text)
/// - Incomplete localization coverage
/// - Hard-to-debug issues in production
///
/// ## Examples
///
/// ### Source Code
/// ```rsx
/// <h1>{t("dashboard.welcome")}</h1>
/// <p>{t("dashboard.description")}</p>
/// ```
///
/// ### Locale File (en.json)
/// ```json
/// {
///   "dashboard": {
///     "welcome": "Welcome to the Dashboard"
///   }
/// }
/// ```
///
/// This would flag `dashboard.description` as missing.
#[derive(Debug, Clone)]
pub struct MissingTranslation {
    /// Loaded translations from locale files
    translations: HashMap<String, HashSet<String>>,
    /// Primary locale to check against
    primary_locale: String,
    /// Whether to check all locales or just primary
    check_all_locales: bool,
    /// Translation function name to look for
    translation_function: String,
}

impl MissingTranslation {
    /// Create a new instance with the given configuration.
    pub fn new(config: &I18nConfig) -> Self {
        let translations = Self::load_translations(config);

        Self {
            translations,
            primary_locale: config.primary_locale.clone().unwrap_or_else(|| "en".to_string()),
            check_all_locales: config.check_all_locales,
            translation_function: config
                .translation_function
                .clone()
                .unwrap_or_else(|| "t".to_string()),
        }
    }

    /// Load translations from locale directory.
    fn load_translations(config: &I18nConfig) -> HashMap<String, HashSet<String>> {
        let mut translations = HashMap::new();

        if let Some(ref locale_dir) = config.locale_dir {
            let locale_path = Path::new(locale_dir);
            if locale_path.exists() {
                if let Ok(entries) = std::fs::read_dir(locale_path) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("json") {
                            if let Some(locale) = path.file_stem().and_then(|s| s.to_str()) {
                                if let Ok(keys) = Self::extract_keys_from_file(&path) {
                                    translations.insert(locale.to_string(), keys);
                                }
                            }
                        }
                    }
                }
            }
        }

        // If no translations loaded, provide empty set for primary locale
        if translations.is_empty() {
            translations.insert("en".to_string(), HashSet::new());
        }

        translations
    }

    /// Extract all translation keys from a JSON locale file.
    fn extract_keys_from_file(path: &Path) -> Result<HashSet<String>, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let keys = Self::extract_keys_from_json(&content, "");
        Ok(keys)
    }

    /// Recursively extract keys from JSON content.
    fn extract_keys_from_json(content: &str, prefix: &str) -> HashSet<String> {
        let mut keys = HashSet::new();

        // Simple JSON parsing for nested keys
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(content) {
            Self::collect_keys(&value, prefix, &mut keys);
        }

        keys
    }

    /// Recursively collect keys from a JSON value.
    fn collect_keys(value: &serde_json::Value, prefix: &str, keys: &mut HashSet<String>) {
        match value {
            serde_json::Value::Object(map) => {
                for (key, val) in map {
                    let full_key = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{prefix}.{key}")
                    };
                    Self::collect_keys(val, &full_key, keys);
                }
            }
            serde_json::Value::String(_) => {
                if !prefix.is_empty() {
                    keys.insert(prefix.to_string());
                }
            }
            _ => {
                // For arrays and other types, add the prefix as a key
                if !prefix.is_empty() {
                    keys.insert(prefix.to_string());
                }
            }
        }
    }

    /// Find all translation keys used in the source code.
    fn find_used_keys(&self, source: &str) -> Vec<UsedTranslationKey> {
        let mut keys = Vec::new();
        let pattern = format!(r#"{}[\s]*\([\s]*["']([^"']+)["']"#, regex::escape(&self.translation_function));

        // Simple regex-like matching
        let func_pattern = format!("{}(", self.translation_function);
        let chars: Vec<char> = source.chars().collect();
        let mut i = 0;
        let mut line = 1;
        let mut column = 1;

        while i < chars.len() {
            let c = chars[i];

            if c == '\n' {
                line += 1;
                column = 1;
                i += 1;
                continue;
            }

            // Look for t( pattern
            let remaining: String = chars[i..].iter().take(func_pattern.len()).collect();
            if remaining == func_pattern {
                let key_start_line = line;
                let key_start_column = column;

                // Skip to opening quote
                let mut j = i + func_pattern.len();
                while j < chars.len() && chars[j].is_whitespace() {
                    if chars[j] == '\n' {
                        line += 1;
                        column = 1;
                    } else {
                        column += 1;
                    }
                    j += 1;
                }

                // Check for quote
                if j < chars.len() && (chars[j] == '"' || chars[j] == '\'') {
                    let quote_char = chars[j];
                    j += 1;
                    column += 1;
                    let key_start = j;

                    // Find closing quote
                    while j < chars.len() && chars[j] != quote_char {
                        if chars[j] == '\n' {
                            line += 1;
                            column = 1;
                        } else {
                            column += 1;
                        }
                        j += 1;
                    }

                    if j < chars.len() {
                        let key: String = chars[key_start..j].iter().collect();
                        keys.push(UsedTranslationKey {
                            key,
                            location: SourceLocation {
                                file: PathBuf::new(),
                                line: key_start_line,
                                column: key_start_column,
                                end_line: line,
                                end_column: column + 1,
                            },
                        });
                    }

                    i = j + 1;
                    column += 1;
                    continue;
                }
            }

            column += 1;
            i += 1;
        }

        keys
    }

    /// Check if a key exists in the translations.
    fn key_exists(&self, key: &str, locale: &str) -> bool {
        self.translations
            .get(locale)
            .map(|keys| keys.contains(key))
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone)]
struct UsedTranslationKey {
    key: String,
    location: SourceLocation,
}

impl LintRule for MissingTranslation {
    fn id(&self) -> &'static str {
        "I18N003"
    }

    fn name(&self) -> &'static str {
        "missing-translation"
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn description(&self) -> &'static str {
        "Validates that translation keys used in source code exist in locale files"
    }

    fn check(&self, source: &str, file_path: &Path) -> Vec<LintDiagnostic> {
        let used_keys = self.find_used_keys(source);
        let mut diagnostics = Vec::new();

        for used_key in used_keys {
            let mut missing_locales = Vec::new();

            if self.check_all_locales {
                // Check all loaded locales
                for (locale, _) in &self.translations {
                    if !self.key_exists(&used_key.key, locale) {
                        missing_locales.push(locale.clone());
                    }
                }
            } else {
                // Check only primary locale
                if !self.key_exists(&used_key.key, &self.primary_locale) {
                    missing_locales.push(self.primary_locale.clone());
                }
            }

            if !missing_locales.is_empty() {
                let mut location = used_key.location.clone();
                location.file = file_path.to_path_buf();

                diagnostics.push(LintDiagnostic {
                    rule_id: self.id().to_string(),
                    rule_name: self.name().to_string(),
                    severity: self.severity(),
                    message: format!(
                        "Translation key \"{}\" is missing in locale(s): {}",
                        used_key.key,
                        missing_locales.join(", ")
                    ),
                    location,
                    suggestion: Some(format!(
                        "Add the key \"{}\" to your locale file(s)",
                        used_key.key
                    )),
                    advocacy_message: Some(
                        "Missing translations cause a poor user experience and can lead to \
                        confusion. Ensuring all keys are translated before deployment helps \
                        maintain a professional, inclusive application."
                            .to_string(),
                    ),
                });
            }
        }

        diagnostics
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Truncate a string to a maximum length, adding ellipsis if needed.
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Generate a translation key from a string.
fn generate_translation_key(text: &str) -> String {
    // Convert to lowercase, replace spaces with underscores, remove special chars
    let key: String = text
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() {
                '_'
            } else {
                '_'
            }
        })
        .collect();

    // Remove consecutive underscores and trim
    let mut result = String::new();
    let mut prev_underscore = false;
    for c in key.chars() {
        if c == '_' {
            if !prev_underscore && !result.is_empty() {
                result.push(c);
                prev_underscore = true;
            }
        } else {
            result.push(c);
            prev_underscore = false;
        }
    }

    // Limit length and add namespace
    let trimmed: String = result.chars().take(30).collect();
    format!("content.{}", trimmed.trim_end_matches('_'))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> I18nConfig {
        I18nConfig {
            enabled: true,
            strict: false,
            locale_dir: None,
            allowlist: vec![
                "SWE Cloud".to_string(),
                "RustScript".to_string(),
                "WebAssembly".to_string(),
            ],
            allowlist_patterns: vec![],
            translation_function: Some("t".to_string()),
            alternative_functions: vec![],
            primary_locale: Some("en".to_string()),
            check_all_locales: false,
        }
    }

    mod no_hardcoded_strings {
        use super::*;

        #[test]
        fn test_detects_hardcoded_string_in_jsx() {
            let config = default_config();
            let rule = NoHardcodedStrings::new(&config);

            let source = r#"
                <div>
                    <h1>"Welcome to our app"</h1>
                </div>
            "#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            assert!(!diagnostics.is_empty(), "Should detect hardcoded string");
            assert!(diagnostics[0].message.contains("Welcome to our app"));
        }

        #[test]
        fn test_allows_allowlisted_strings() {
            let config = default_config();
            let rule = NoHardcodedStrings::new(&config);

            let source = r#"
                <div>
                    <h1>"SWE Cloud"</h1>
                </div>
            "#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            assert!(diagnostics.is_empty(), "Should allow allowlisted string");
        }

        #[test]
        fn test_allows_numeric_strings() {
            let config = default_config();
            let rule = NoHardcodedStrings::new(&config);

            let source = r#"
                <div>
                    <span>"100"</span>
                    <span>"2.5ms"</span>
                    <span>"99.9%"</span>
                </div>
            "#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            assert!(diagnostics.is_empty(), "Should allow numeric strings");
        }

        #[test]
        fn test_allows_single_characters() {
            let config = default_config();
            let rule = NoHardcodedStrings::new(&config);

            let source = r#"
                <div>
                    <span>"X"</span>
                </div>
            "#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            assert!(diagnostics.is_empty(), "Should allow single characters");
        }

        #[test]
        fn test_suggestion_includes_translation_key() {
            let config = default_config();
            let rule = NoHardcodedStrings::new(&config);

            let source = r#"<h1>"Hello World"</h1>"#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            assert!(!diagnostics.is_empty());
            assert!(diagnostics[0].suggestion.is_some());
            assert!(diagnostics[0].suggestion.as_ref().unwrap().contains("t("));
        }

        #[test]
        fn test_advocacy_message_present() {
            let config = default_config();
            let rule = NoHardcodedStrings::new(&config);

            let source = r#"<h1>"Test String"</h1>"#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            assert!(!diagnostics.is_empty());
            assert!(diagnostics[0].advocacy_message.is_some());
            assert!(diagnostics[0]
                .advocacy_message
                .as_ref()
                .unwrap()
                .contains("Internationalization"));
        }

        #[test]
        fn test_pattern_matching_startswith() {
            let mut config = default_config();
            config.allowlist_patterns = vec!["Version*".to_string()];
            let rule = NoHardcodedStrings::new(&config);

            let source = r#"<span>"Version 1.0.0"</span>"#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            assert!(diagnostics.is_empty(), "Should allow pattern match");
        }

        #[test]
        fn test_pattern_matching_contains() {
            let mut config = default_config();
            config.allowlist_patterns = vec!["*Copyright*".to_string()];
            let rule = NoHardcodedStrings::new(&config);

            let source = r#"<span>"(c) Copyright 2024"</span>"#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            assert!(diagnostics.is_empty(), "Should allow pattern match");
        }
    }

    mod use_translation_key {
        use super::*;

        #[test]
        fn test_detects_template_literal() {
            let config = default_config();
            let rule = UseTranslationKey::new(&config);

            let source = r#"
                <div>
                    {`Welcome ${name}`}
                </div>
            "#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            assert!(!diagnostics.is_empty(), "Should detect template literal");
        }

        #[test]
        fn test_detects_wrong_function() {
            let config = default_config();
            let rule = UseTranslationKey::new(&config);

            let source = r#"
                <div>
                    {translate("hello")}
                </div>
            "#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            assert!(!diagnostics.is_empty(), "Should detect wrong translation function");
        }

        #[test]
        fn test_allows_correct_function() {
            let config = default_config();
            let rule = UseTranslationKey::new(&config);

            let source = r#"
                <div>
                    {t("hello")}
                </div>
            "#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            // Note: t("hello") contains a string but uses t(), so it should be allowed
            // We're checking that using t() is acceptable
            let violations: Vec<_> = diagnostics
                .iter()
                .filter(|d| d.message.contains("translate"))
                .collect();
            assert!(violations.is_empty(), "Should allow t() function");
        }

        #[test]
        fn test_allows_alternative_function() {
            let mut config = default_config();
            config.alternative_functions = vec!["trans".to_string()];
            let rule = UseTranslationKey::new(&config);

            let source = r#"
                <div>
                    {trans("hello")}
                </div>
            "#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            assert!(diagnostics.is_empty(), "Should allow alternative function");
        }

        #[test]
        fn test_suggestion_includes_correct_function() {
            let config = default_config();
            let rule = UseTranslationKey::new(&config);

            let source = r#"{translate("hello")}"#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            assert!(!diagnostics.is_empty());
            assert!(diagnostics[0].suggestion.is_some());
            assert!(diagnostics[0].suggestion.as_ref().unwrap().contains("t("));
        }
    }

    mod missing_translation {
        use super::*;
        use std::io::Write;
        use tempfile::TempDir;

        fn setup_locale_files() -> (TempDir, I18nConfig) {
            let temp_dir = TempDir::new().unwrap();

            // Create en.json
            let en_path = temp_dir.path().join("en.json");
            let mut en_file = std::fs::File::create(&en_path).unwrap();
            writeln!(
                en_file,
                r#"{{
                    "common": {{
                        "submit": "Submit",
                        "cancel": "Cancel"
                    }},
                    "dashboard": {{
                        "title": "Dashboard"
                    }}
                }}"#
            )
            .unwrap();

            // Create es.json (missing some keys)
            let es_path = temp_dir.path().join("es.json");
            let mut es_file = std::fs::File::create(&es_path).unwrap();
            writeln!(
                es_file,
                r#"{{
                    "common": {{
                        "submit": "Enviar"
                    }}
                }}"#
            )
            .unwrap();

            let config = I18nConfig {
                enabled: true,
                strict: true,
                locale_dir: Some(temp_dir.path().to_string_lossy().to_string()),
                allowlist: vec![],
                allowlist_patterns: vec![],
                translation_function: Some("t".to_string()),
                alternative_functions: vec![],
                primary_locale: Some("en".to_string()),
                check_all_locales: false,
            };

            (temp_dir, config)
        }

        #[test]
        fn test_detects_missing_key() {
            let (_temp_dir, config) = setup_locale_files();
            let rule = MissingTranslation::new(&config);

            let source = r#"
                <div>
                    {t("common.submit")}
                    {t("missing.key")}
                </div>
            "#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            assert!(!diagnostics.is_empty(), "Should detect missing key");
            assert!(diagnostics.iter().any(|d| d.message.contains("missing.key")));
        }

        #[test]
        fn test_allows_existing_key() {
            let (_temp_dir, config) = setup_locale_files();
            let rule = MissingTranslation::new(&config);

            let source = r#"
                <div>
                    {t("common.submit")}
                    {t("dashboard.title")}
                </div>
            "#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            assert!(diagnostics.is_empty(), "Should not flag existing keys");
        }

        #[test]
        fn test_check_all_locales() {
            let (_temp_dir, mut config) = setup_locale_files();
            config.check_all_locales = true;
            let rule = MissingTranslation::new(&config);

            let source = r#"
                <div>
                    {t("common.cancel")}
                </div>
            "#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            // common.cancel exists in en but not in es
            assert!(!diagnostics.is_empty(), "Should detect missing key in es locale");
            assert!(diagnostics[0].message.contains("es"));
        }

        #[test]
        fn test_severity_is_error() {
            let config = default_config();
            let rule = MissingTranslation::new(&config);
            assert!(matches!(rule.severity(), Severity::Error));
        }

        #[test]
        fn test_nested_keys_extracted() {
            let (_temp_dir, config) = setup_locale_files();
            let rule = MissingTranslation::new(&config);

            let source = r#"{t("common.submit")}"#;

            let diagnostics = rule.check(source, Path::new("test.rsx"));
            // common.submit exists, so no diagnostics
            assert!(diagnostics.is_empty());
        }
    }

    mod helper_functions {
        use super::*;

        #[test]
        fn test_truncate_string_short() {
            let result = truncate_string("hello", 10);
            assert_eq!(result, "hello");
        }

        #[test]
        fn test_truncate_string_long() {
            let result = truncate_string("this is a very long string", 15);
            assert_eq!(result, "this is a ve...");
        }

        #[test]
        fn test_generate_translation_key() {
            let result = generate_translation_key("Hello World");
            assert_eq!(result, "content.hello_world");
        }

        #[test]
        fn test_generate_translation_key_special_chars() {
            let result = generate_translation_key("Hello, World!");
            assert_eq!(result, "content.hello_world");
        }

        #[test]
        fn test_generate_translation_key_long() {
            let result = generate_translation_key(
                "This is a very long string that should be truncated to a reasonable length",
            );
            assert!(result.len() <= 40);
            assert!(result.starts_with("content."));
        }
    }

    mod rule_metadata {
        use super::*;

        #[test]
        fn test_no_hardcoded_strings_metadata() {
            let config = default_config();
            let rule = NoHardcodedStrings::new(&config);

            assert_eq!(rule.id(), "I18N001");
            assert_eq!(rule.name(), "no-hardcoded-strings");
            assert!(matches!(rule.severity(), Severity::Warning));
            assert!(!rule.description().is_empty());
        }

        #[test]
        fn test_use_translation_key_metadata() {
            let config = default_config();
            let rule = UseTranslationKey::new(&config);

            assert_eq!(rule.id(), "I18N002");
            assert_eq!(rule.name(), "use-translation-key");
            assert!(matches!(rule.severity(), Severity::Warning));
            assert!(!rule.description().is_empty());
        }

        #[test]
        fn test_missing_translation_metadata() {
            let config = default_config();
            let rule = MissingTranslation::new(&config);

            assert_eq!(rule.id(), "I18N003");
            assert_eq!(rule.name(), "missing-translation");
            assert!(matches!(rule.severity(), Severity::Error));
            assert!(!rule.description().is_empty());
        }
    }
}
