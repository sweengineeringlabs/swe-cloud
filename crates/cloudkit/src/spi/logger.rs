//! Logger SPI.

use async_trait::async_trait;
use std::collections::HashMap;

/// Log level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Trace level
    Trace,
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warning level
    Warn,
    /// Error level
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// Log entry.
#[derive(Debug, Clone)]
pub struct LogEntry {
    /// Log level
    pub level: LogLevel,
    /// Log message
    pub message: String,
    /// Target (usually module path)
    pub target: String,
    /// Structured fields
    pub fields: HashMap<String, String>,
}

impl LogEntry {
    /// Create a new log entry.
    pub fn new(level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            level,
            message: message.into(),
            target: String::new(),
            fields: HashMap::new(),
        }
    }

    /// Set the target.
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = target.into();
        self
    }

    /// Add a field.
    pub fn with_field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.insert(key.into(), value.into());
        self
    }
}

/// Logger trait for custom logging.
#[async_trait]
pub trait Logger: Send + Sync {
    /// Log an entry.
    async fn log(&self, entry: LogEntry);

    /// Check if a log level is enabled.
    fn is_enabled(&self, level: LogLevel) -> bool;
}

/// Default logger using tracing.
#[derive(Debug, Default, Clone)]
pub struct TracingLogger {
    min_level: LogLevel,
}

impl TracingLogger {
    /// Create a new tracing logger.
    pub fn new(min_level: LogLevel) -> Self {
        Self { min_level }
    }
}

#[async_trait]
impl Logger for TracingLogger {
    async fn log(&self, entry: LogEntry) {
        if !self.is_enabled(entry.level) {
            return;
        }

        match entry.level {
            LogLevel::Trace => tracing::trace!(target: "cloudkit", message = %entry.message, fields = ?entry.fields),
            LogLevel::Debug => tracing::debug!(target: "cloudkit", message = %entry.message, fields = ?entry.fields),
            LogLevel::Info => tracing::info!(target: "cloudkit", message = %entry.message, fields = ?entry.fields),
            LogLevel::Warn => tracing::warn!(target: "cloudkit", message = %entry.message, fields = ?entry.fields),
            LogLevel::Error => tracing::error!(target: "cloudkit", message = %entry.message, fields = ?entry.fields),
        }
    }

    fn is_enabled(&self, level: LogLevel) -> bool {
        level >= self.min_level
    }
}

/// No-op logger.
#[derive(Debug, Default, Clone)]
pub struct NoopLogger;

#[async_trait]
impl Logger for NoopLogger {
    async fn log(&self, _entry: LogEntry) {}

    fn is_enabled(&self, _level: LogLevel) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Error > LogLevel::Warn);
        assert!(LogLevel::Warn > LogLevel::Info);
        assert!(LogLevel::Info > LogLevel::Debug);
        assert!(LogLevel::Debug > LogLevel::Trace);
    }

    #[test]
    fn test_log_entry_builder() {
        let entry = LogEntry::new(LogLevel::Info, "Test message")
            .with_target("cloudkit::test")
            .with_field("key", "value");

        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.message, "Test message");
        assert_eq!(entry.target, "cloudkit::test");
        assert_eq!(entry.fields.get("key"), Some(&"value".to_string()));
    }
}
