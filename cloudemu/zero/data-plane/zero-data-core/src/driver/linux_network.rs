use zero_control_spi::{NetworkDriver, ZeroResult, ZeroError, NetworkStatus};
use async_trait::async_trait;
use std::process::Command;
use std::fs;

/// Linux Bridge Network Driver.
/// Manages Linux Bridges and Libvirt Networks.
pub struct LinuxNetworkDriver;

impl LinuxNetworkDriver {
    pub fn new() -> Self {
        Self
    }

    fn run_virsh(&self, args: Vec<&str>) -> ZeroResult<String> {
        let output = Command::new("virsh")
            .args(args)
            .output()
            .map_err(|e| ZeroError::Driver(format!("Failed to execute virsh: {}", e)))?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(ZeroError::Driver(format!("Linux Network command failed: {}", err)));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

#[async_trait]
impl NetworkDriver for LinuxNetworkDriver {
    async fn create_network(&self, id: &str, cidr: &str) -> ZeroResult<NetworkStatus> {
        // Create a libvirt XML definition for the network
        // Bridge name will match ID
        let xml = format!(r#"
<network>
  <name>{}</name>
  <bridge name='{}' stp='on' delay='0'/>
  <ip address='{}' netmask='255.255.255.0'>
    <dhcp>
      <range start='{}.100' end='{}.200'/>
    </dhcp>
  </ip>
</network>
"#, id, id, cidr.replace("/24", ".1"), cidr.replace(".0/24", ""), cidr.replace(".0/24", ""));

        let temp_xml = format!("/tmp/zero-net-{}.xml", id);
        fs::write(&temp_xml, xml).map_err(|e| ZeroError::Driver(e.to_string()))?;

        self.run_virsh(vec!["net-define", &temp_xml])?;
        self.run_virsh(vec!["net-start", id])?;
        self.run_virsh(vec!["net-autostart", id])?;

        Ok(NetworkStatus {
            id: id.to_string(),
            cidr: cidr.to_string(),
            state: "Available".to_string(),
        })
    }

    async fn delete_network(&self, id: &str) -> ZeroResult<()> {
        self.run_virsh(vec!["net-destroy", id]).ok();
        self.run_virsh(vec!["net-undefine", id])?;
        Ok(())
    }

    async fn connect_workload(&self, workload_id: &str, network_id: &str) -> ZeroResult<String> {
        // Attach interface to the VM
        let script = format!(
            "virsh attach-interface --domain {} --type network --source {} --model virtio --config --live",
            workload_id, network_id
        );
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(script)
            .output()
            .map_err(|e| ZeroError::Driver(format!("Failed to attach interface: {}", e)))?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(ZeroError::Driver(format!("Connect failed: {}", err)));
        }

        Ok("CONNECTED".to_string())
    }

    async fn list_networks(&self) -> ZeroResult<Vec<NetworkStatus>> {
        let output = self.run_virsh(vec!["net-list", "--all", "--name"])?;
        let mut networks = Vec::new();

        for name in output.lines() {
            let name = name.trim();
            if name.is_empty() { continue; }

            networks.push(NetworkStatus {
                id: name.to_string(),
                cidr: "Unknown".into(),
                state: "Available".into(),
            });
        }

        Ok(networks)
    }
}
