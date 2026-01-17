use zero_control_spi::{ComputeDriver, ZeroResult, ZeroError, WorkloadStatus};
use async_trait::async_trait;
use std::process::Command;

/// KVM Driver for Linux-native virtualization.
/// Uses libvirt/virsh internally to manage Virtual Machines.
pub struct KvmDriver;

impl KvmDriver {
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
            return Err(ZeroError::Driver(format!("KVM command failed: {}", err)));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

#[async_trait]
impl ComputeDriver for KvmDriver {
    async fn create_workload(&self, id: &str, image: &str, _cpu: f32, mem_mb: i32) -> ZeroResult<WorkloadStatus> {
        // virt-install is usually cleaner for creation
        let mem_str = mem_mb.to_string();
        let output = Command::new("virt-install")
            .arg("--name").arg(id)
            .arg("--memory").arg(mem_str)
            .arg("--vcpus").arg("1")
            .arg("--disk").arg(format!("path=/var/lib/libvirt/images/{}.qcow2,size=10", id))
            .arg("--import")
            .arg("--noautoconsole")
            .arg("--graphics").arg("none")
            .output()
            .map_err(|e| ZeroError::Driver(format!("Failed to execute virt-install: {}", e)))?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(ZeroError::Driver(format!("KVM Create failed: {}", err)));
        }

        Ok(WorkloadStatus {
            id: id.to_string(),
            state: "Running".to_string(),
            ip_address: None,
        })
    }

    async fn delete_workload(&self, id: &str) -> ZeroResult<()> {
        // Force stop and undefine
        self.run_virsh(vec!["destroy", id]).ok(); // ignore if already stopped
        self.run_virsh(vec!["undefine", id, "--remove-all-storage"])?;
        Ok(())
    }

    async fn get_workload_status(&self, id: &str) -> ZeroResult<WorkloadStatus> {
        let state = self.run_virsh(vec!["domstate", id])?;
        
        let normalized_state = match state.as_str() {
            "running" => "Running",
            "shut off" => "Stopped",
            _ => "Unknown",
        };

        // Try to get IP
        let ip_output = self.run_virsh(vec!["domifaddr", id]).ok();
        let ip = ip_output.and_then(|out| {
            // Very basic parsing of virsh domifaddr output
            out.lines().nth(2).and_then(|line| {
                line.split_whitespace().nth(3).map(|ip_cidr| {
                    ip_cidr.split('/').next().unwrap_or(ip_cidr).to_string()
                })
            })
        });

        Ok(WorkloadStatus {
            id: id.to_string(),
            state: normalized_state.to_string(),
            ip_address: ip,
        })
    }
}
