//! # RustScript Security (rsc-security)
//!
//! Security-focused lint rules for RustScript projects.
//!
//! This crate extends rsc-lint with security-specific rules for detecting
//! vulnerabilities and enforcing security best practices.
//!
//! ## Usage
//!
//! Security rules integrate with the main lint engine from rsc-lint:
//!
//! ```rust
//! use rsc_lint::{LintEngine, LintConfig};
//! use rsc_security::rules;
//!
//! // Security rules are automatically registered when using LintConfig
//! let engine = LintEngine::new(LintConfig::default());
//! ```
//!
//! ## Rule Categories
//!
//! - **XSS Prevention** - Prevent cross-site scripting vulnerabilities
//! - **Secret Detection** - Find hardcoded credentials and API keys
//! - **Input Validation** - Enforce proper input sanitization
//!
//! ## Configuration
//!
//! ```toml
//! [lint.security]
//! enabled = true
//! strict = true
//!
//! # Allowlist for false positives
//! [lint.security.secrets]
//! allowlist_patterns = ["TEST_*", "*_PLACEHOLDER"]
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]

pub mod rules;

// Re-export from rsc-lint for convenience
pub use rsc_lint::{LintDiagnostic, LintRule, Severity, SourceLocation};
