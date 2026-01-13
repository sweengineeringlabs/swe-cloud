//! Oracle Cloud client builder.

use cloudkit_spi::{CloudConfig, CloudResult, Region, CloudContext, ProviderType};
use std::sync::Arc;

/// Oracle Cloud client builder.
pub struct OracleBuilder {
    region: Option<Region>,
    config: Option<CloudConfig>,
    tenancy_ocid: Option<String>,
    compartment_ocid: Option<String>,
}

impl OracleBuilder {
    /// Create a new Oracle Cloud builder.
    pub fn new() -> Self {
        Self {
            region: None,
            config: None,
            tenancy_ocid: None,
            compartment_ocid: None,
        }
    }

    /// Set the Oracle Cloud region.
    pub fn region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    /// Set the tenancy OCID.
    pub fn tenancy(mut self, tenancy_ocid: impl Into<String>) -> Self {
        self.tenancy_ocid = Some(tenancy_ocid.into());
        self
    }

    /// Set the compartment OCID.
    pub fn compartment(mut self, compartment_ocid: impl Into<String>) -> Self {
        self.compartment_ocid = Some(compartment_ocid.into());
        self
    }

    /// Set the configuration.
    pub fn config(mut self, config: CloudConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Build the Oracle Cloud client.
    pub async fn build(self) -> CloudResult<OracleClient> {
        let mut config = self.config.unwrap_or_default();
        
        if let Some(region) = self.region {
            config.region = region;
        }

        let context = CloudContext::builder(ProviderType::Oracle)
            .config(config)
            .build()
            .await?;

        Ok(OracleClient {
            context: Arc::new(context),
            tenancy_ocid: self.tenancy_ocid,
            compartment_ocid: self.compartment_ocid,
        })
    }
}

impl Default for OracleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Oracle Cloud client.
pub struct OracleClient {
    context: Arc<CloudContext>,
    tenancy_ocid: Option<String>,
    compartment_ocid: Option<String>,
}

impl OracleClient {
    /// Get the underlying context.
    pub fn context(&self) -> &CloudContext {
        &self.context
    }

    /// Get the tenancy OCID.
    pub fn tenancy_ocid(&self) -> Option<&str> {
        self.tenancy_ocid.as_deref()
    }

    /// Get the compartment OCID.
    pub fn compartment_ocid(&self) -> Option<&str> {
        self.compartment_ocid.as_deref()
    }
}

