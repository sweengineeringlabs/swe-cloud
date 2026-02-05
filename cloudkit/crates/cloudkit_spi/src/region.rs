//! Cloud region definitions.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Cloud region identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Region {
    /// Provider name (aws, azure, gcp, oracle)
    provider: String,
    /// Region code (e.g., "us-east-1", "eastus", "us-central1")
    code: String,
    /// Human-readable name
    name: String,
}

impl Region {
    /// Create a new region.
    pub fn new(provider: impl Into<String>, code: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            code: code.into(),
            name: name.into(),
        }
    }

    /// Get the region code.
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Get the provider.
    pub fn provider(&self) -> &str {
        &self.provider
    }

    /// Get the human-readable name.
    pub fn name(&self) -> &str {
        &self.name
    }

    // =========================================================================
    // AWS Regions
    // =========================================================================

    /// AWS US East (N. Virginia)
    pub fn aws_us_east_1() -> Self {
        Self::new("aws", "us-east-1", "US East (N. Virginia)")
    }

    /// AWS US East (Ohio)
    pub fn aws_us_east_2() -> Self {
        Self::new("aws", "us-east-2", "US East (Ohio)")
    }

    /// AWS US West (N. California)
    pub fn aws_us_west_1() -> Self {
        Self::new("aws", "us-west-1", "US West (N. California)")
    }

    /// AWS US West (Oregon)
    pub fn aws_us_west_2() -> Self {
        Self::new("aws", "us-west-2", "US West (Oregon)")
    }

    /// AWS EU (Ireland)
    pub fn aws_eu_west_1() -> Self {
        Self::new("aws", "eu-west-1", "EU (Ireland)")
    }

    /// AWS EU (Frankfurt)
    pub fn aws_eu_central_1() -> Self {
        Self::new("aws", "eu-central-1", "EU (Frankfurt)")
    }

    /// AWS Africa (Cape Town)
    pub fn aws_af_south_1() -> Self {
        Self::new("aws", "af-south-1", "Africa (Cape Town)")
    }

    // =========================================================================
    // Azure Regions
    // =========================================================================

    /// Azure East US
    pub fn azure_east_us() -> Self {
        Self::new("azure", "eastus", "East US")
    }

    /// Azure West US
    pub fn azure_west_us() -> Self {
        Self::new("azure", "westus", "West US")
    }

    /// Azure West Europe
    pub fn azure_west_europe() -> Self {
        Self::new("azure", "westeurope", "West Europe")
    }

    /// Azure South Africa North
    pub fn azure_south_africa_north() -> Self {
        Self::new("azure", "southafricanorth", "South Africa North")
    }

    // =========================================================================
    // GCP Regions
    // =========================================================================

    /// GCP US Central
    pub fn gcp_us_central1() -> Self {
        Self::new("gcp", "us-central1", "Iowa")
    }

    /// GCP US East
    pub fn gcp_us_east1() -> Self {
        Self::new("gcp", "us-east1", "South Carolina")
    }

    /// GCP Europe West
    pub fn gcp_europe_west1() -> Self {
        Self::new("gcp", "europe-west1", "Belgium")
    }

    // =========================================================================
    // Oracle Cloud Regions
    // =========================================================================

    /// Oracle Cloud US East (Ashburn)
    pub fn oracle_us_ashburn_1() -> Self {
        Self::new("oracle", "us-ashburn-1", "US East (Ashburn)")
    }

    /// Oracle Cloud UK South (London)
    pub fn oracle_uk_london_1() -> Self {
        Self::new("oracle", "uk-london-1", "UK South (London)")
    }

    /// Oracle Cloud South Africa (Johannesburg)
    pub fn oracle_af_johannesburg_1() -> Self {
        Self::new("oracle", "af-johannesburg-1", "South Africa (Johannesburg)")
    }

    // =========================================================================
    // ZeroCloud Regions
    // =========================================================================

    /// ZeroCloud Local Region
    pub fn zero_local() -> Self {
        Self::new("zero", "local", "ZeroCloud Local")
    }

    /// ZeroCloud Default Region
    pub fn zero_default() -> Self {
        Self::new("zero", "default", "ZeroCloud Default")
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.provider, self.code)
    }
}

impl Default for Region {
    fn default() -> Self {
        Self::aws_us_east_1()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_display() {
        let region = Region::aws_us_east_1();
        assert_eq!(region.to_string(), "aws/us-east-1");
    }

    #[test]
    fn test_region_accessors() {
        let region = Region::azure_south_africa_north();
        assert_eq!(region.provider(), "azure");
        assert_eq!(region.code(), "southafricanorth");
        assert_eq!(region.name(), "South Africa North");
    }
}
