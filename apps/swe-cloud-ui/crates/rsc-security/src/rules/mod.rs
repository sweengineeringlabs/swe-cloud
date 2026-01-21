//! # Security Rules Module
//!
//! This module provides security-related lint rules for RustScript projects.
//! These rules help detect common security vulnerabilities and enforce
//! security best practices.
//!
//! ## Rule Categories
//!
//! - **SEC001-SEC099**: Input validation rules
//! - **SEC100-SEC199**: Authentication/authorization rules
//! - **SEC200-SEC299**: Data protection rules
//! - **SEC300-SEC399**: XSS prevention rules
//!
//! ## Future Rules
//!
//! The following rules are planned for future implementation:
//!
//! | ID | Name | Description |
//! |----|------|-------------|
//! | SEC001 | no-dangerously-set-inner-html | Prevent XSS via innerHTML |
//! | SEC002 | no-eval | Prevent code injection via eval |
//! | SEC003 | no-hardcoded-secrets | Detect hardcoded API keys/passwords |
//! | SEC004 | require-https | Enforce HTTPS for external URLs |

// Placeholder for future security rules
// Follow the same pattern as i18n rules in rsc-lint

/// Placeholder trait for security rules (shares LintRule from rsc-lint)
pub use rsc_lint::LintRule;

// Future implementations would go here:
// pub mod xss;
// pub mod secrets;
// pub mod auth;
