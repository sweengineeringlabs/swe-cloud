//! Emulator configuration

use std::path::PathBuf;

/// Emulator configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Host to bind to
    pub host: String,
    /// Port to listen on
    pub port: u16,
    /// Data directory for persistence
    pub data_dir: PathBuf,
    /// AWS region to emulate
    pub region: String,
    /// AWS account ID to use
    pub account_id: String,
    /// Enable request logging
    pub enable_logging: bool,
    /// Enable AWS Signature V4 validation
    pub validate_signatures: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 4566,
            data_dir: PathBuf::from(".cloudemu"),
            region: "us-east-1".to_string(),
            account_id: "000000000000".to_string(),
            enable_logging: true,
            validate_signatures: false, // Disabled by default for ease of use
        }
    }
}

impl Config {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        if let Ok(host) = std::env::var("CLOUDEMU_HOST") {
            config.host = host;
        }
        if let Ok(port) = std::env::var("CLOUDEMU_PORT") {
            if let Ok(p) = port.parse() {
            config.port = p;
            }
        }
        if let Ok(dir) = std::env::var("CLOUDEMU_DATA_DIR") {
            config.data_dir = PathBuf::from(dir);
        }
        if let Ok(region) = std::env::var("CLOUDEMU_REGION") {
            config.region = region;
        }
        if let Ok(account) = std::env::var("CLOUDEMU_ACCOUNT_ID") {
            config.account_id = account;
        }
        if let Ok(logging) = std::env::var("CLOUDEMU_LOGGING") {
            config.enable_logging = logging == "true" || logging == "1";
        }
        if let Ok(validate) = std::env::var("CLOUDEMU_VALIDATE_SIGNATURES") {
            config.validate_signatures = validate == "true" || validate == "1";
        }
        
        config
    }

    /// Builder-style host setter
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    /// Builder-style port setter
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Builder-style data_dir setter  
    pub fn data_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.data_dir = path.into();
        self
    }

    /// Builder-style region setter
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.region = region.into();
        self
    }
}

