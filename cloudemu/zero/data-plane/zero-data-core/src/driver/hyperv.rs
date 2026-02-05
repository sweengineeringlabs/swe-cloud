use zero_control_spi::{ComputeDriver, ZeroResult, ZeroError, WorkloadStatus};
use async_trait::async_trait;
use std::process::Command;

/// Hyper-V Driver for Windows-native virtualization.
/// Uses PowerShell internally to manage Virtual Machines.
pub struct HyperVDriver;

impl Default for HyperVDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl HyperVDriver {
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
            return Err(ZeroError::Driver(format!("Hyper-V command failed: {}", err)));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

#[async_trait]
impl ComputeDriver for HyperVDriver {
    async fn create_workload(&self, id: &str, _image: &str, _cpu: f32, mem_mb: i32) -> ZeroResult<WorkloadStatus> {
        // Image in Hyper-V context would usually be a VHDX path.
        // For emulation, we'll create a VM without a disk if not specified, 
        // or assume a base VHDX exists.
        
        let mem_bytes = (mem_mb as u64) * 1024 * 1024;
        let script = format!(
            "New-VM -Name '{}' -MemoryStartupBytes {} -Generation 2; Start-VM -Name '{}'",
            id, mem_bytes, id
        );

        self.run_powershell(&script)?;

        Ok(WorkloadStatus {
            id: id.to_string(),
            state: "Running".to_string(),
            ip_address: None,
        })
    }

    async fn delete_workload(&self, id: &str) -> ZeroResult<()> {
        let script = format!("Stop-VM -Name '{}' -Force; Remove-VM -Name '{}' -Force", id, id);
        self.run_powershell(&script)?;
        Ok(())
    }

    async fn get_workload_status(&self, id: &str) -> ZeroResult<WorkloadStatus> {
        let script = format!("(Get-VM -Name '{}').State", id);
        let state = self.run_powershell(&script)?;
        
        // Map Hyper-V states to ZeroCloud states
        let normalized_state = match state.as_str() {
            "Running" => "Running",
            "Off" => "Stopped",
            _ => "Unknown",
        };

        // Get IP (requires Integration Services)
        let ip_script = format!("(Get-VM -Name '{}' | Get-VMNetworkAdapter).IPAddresses[0]", id);
        let ip = self.run_powershell(&ip_script).ok();

        Ok(WorkloadStatus {
            id: id.to_string(),
            state: normalized_state.to_string(),
            ip_address: ip,
        })
    }

    async fn list_workloads(&self) -> ZeroResult<Vec<WorkloadStatus>> {
        let script = "Get-VM | Select-Object Name, State | ConvertTo-Json";
        let output = self.run_powershell(script)?;
        if output.is_empty() {
             return Ok(vec![]);
        }

        let vms: serde_json::Value = serde_json::from_str(&output)
            .map_err(|e| ZeroError::Driver(format!("JSON parse error: {}", e)))?;

        let mut workloads = Vec::new();
        let items = if vms.is_array() {
            vms.as_array().unwrap().clone()
        } else {
            vec![vms]
        };

        for item in items {
            let id = item["Name"].as_str().unwrap_or("unknown").to_string();
            let state = item["State"].as_i64().map(|s| match s {
                2 => "Running",
                3 => "Stopped",
                _ => "Unknown",
            }).unwrap_or("Unknown").to_string();

            workloads.push(WorkloadStatus {
                id,
                state,
                ip_address: None, // IP requires extra calls per VM
            });
        }

        Ok(workloads)
    }

    async fn get_stats(&self) -> ZeroResult<zero_control_spi::NodeStats> {
        let cpu_script = "(Get-Counter '\\Processor(_Total)\\% Processor Time').CounterSamples.CookedValue";
        let mem_script = "Get-CimInstance Win32_OperatingSystem | Select-Object TotalVisibleMemorySize, FreePhysicalMemory | ConvertTo-Json";
        
        let cpu = self.run_powershell(cpu_script).unwrap_or("0".into()).parse::<f32>().unwrap_or(0.0);
        let mem_json = self.run_powershell(mem_script).unwrap_or("{}".into());
        let mem: serde_json::Value = serde_json::from_str(&mem_json).unwrap_or(serde_json::Value::Null);
        
        let total_mem_kb = mem["TotalVisibleMemorySize"].as_u64().unwrap_or(0);
        let free_mem_kb = mem["FreePhysicalMemory"].as_u64().unwrap_or(0);
        
        Ok(zero_control_spi::NodeStats {
            cpu_usage_percent: cpu,
            memory_used_mb: (total_mem_kb - free_mem_kb) / 1024,
            memory_total_mb: total_mem_kb / 1024,
            storage_used_gb: 0, // Simplified
            storage_total_gb: 0,
        })
    }
}
