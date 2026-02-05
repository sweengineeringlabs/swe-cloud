use zero_control_spi::{NetworkDriver, ZeroResult, ZeroError, NetworkStatus};
use async_trait::async_trait;
use std::process::Command;

/// Hyper-V Network Driver for Windows.
/// Manages Virtual Switches.
pub struct HyperVNetworkDriver;

impl Default for HyperVNetworkDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl HyperVNetworkDriver {
    pub fn new() -> Self {
        Self
    }

    fn run_powershell(&self, script: &str) -> ZeroResult<String> {
        let output = Command::new("powershell")
            .arg("-Command")
            .arg(script)
            .output()
            .map_err(|e| ZeroError::Driver(format!("Failed to execute powershell: {}", e)))?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(ZeroError::Driver(format!("Hyper-V Network command failed: {}", err)));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

#[async_trait]
impl NetworkDriver for HyperVNetworkDriver {
    async fn create_network(&self, id: &str, cidr: &str) -> ZeroResult<NetworkStatus> {
        // Internal switch for isolation
        let script = format!(
            "New-VMSwitch -Name '{}' -SwitchType Internal",
            id
        );
        self.run_powershell(&script)?;
        
        Ok(NetworkStatus {
            id: id.to_string(),
            cidr: cidr.to_string(),
            state: "Available".to_string(),
        })
    }

    async fn delete_network(&self, id: &str) -> ZeroResult<()> {
        let script = format!("Remove-VMSwitch -Name '{}' -Force", id);
        self.run_powershell(&script)?;
        Ok(())
    }

    async fn connect_workload(&self, workload_id: &str, network_id: &str) -> ZeroResult<String> {
        let script = format!(
            "Connect-VMNetworkAdapter -VMName '{}' -SwitchName '{}'",
            workload_id, network_id
        );
        self.run_powershell(&script)?;
        
        // This is a simplification; IP assignment usually happens via DHCP or static config inside VM
        Ok("DHCP_ASSIGNED".to_string())
    }

    async fn list_networks(&self) -> ZeroResult<Vec<NetworkStatus>> {
        let script = "Get-VMSwitch | Select-Object Name | ConvertTo-Json";
        let output = self.run_powershell(script)?;
        if output.is_empty() {
            return Ok(vec![]);
        }

        let switches: serde_json::Value = serde_json::from_str(&output)
            .map_err(|e| ZeroError::Driver(format!("JSON parse error: {}", e)))?;

        let mut networks = Vec::new();
        let items = if switches.is_array() {
            switches.as_array().unwrap().clone()
        } else {
            vec![switches]
        };

        for item in items {
            let id = item["Name"].as_str().unwrap_or("unknown").to_string();
            networks.push(NetworkStatus {
                id,
                cidr: "Unknown".into(), // Hyper-V switches don't have intrinsic CIDRs in this context
                state: "Available".into(),
            });
        }

        Ok(networks)
    }
}
