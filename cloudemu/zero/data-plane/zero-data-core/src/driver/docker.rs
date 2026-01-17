use zero_control_spi::{ComputeDriver, ZeroResult, ZeroError, WorkloadStatus};
use async_trait::async_trait;
use bollard::Docker;
use bollard::container::{CreateContainerOptions, Config, StartContainerOptions};

pub struct DockerDriver {
    client: Docker,
}

impl DockerDriver {
    pub fn new() -> ZeroResult<Self> {
        let client = Docker::connect_with_local_defaults()
            .map_err(|e| ZeroError::Driver(format!("Failed to connect to Docker: {}", e)))?;
        Ok(Self { client })
    }
}

#[async_trait]
impl ComputeDriver for DockerDriver {
    async fn create_workload(&self, id: &str, image: &str, _cpu: f32, _mem_mb: i32) -> ZeroResult<WorkloadStatus> {
        let options = Some(CreateContainerOptions {
            name: id,
            ..Default::default()
        });
        
        let config = Config {
            image: Some(image),
            ..Default::default()
        };

        self.client.create_container(options, config).await
            .map_err(|e| ZeroError::Driver(format!("Docker create error: {}", e)))?;

        self.client.start_container(id, None::<StartContainerOptions<String>>).await
            .map_err(|e| ZeroError::Driver(format!("Docker start error: {}", e)))?;

        Ok(WorkloadStatus {
            id: id.to_string(),
            state: "Running".to_string(),
            ip_address: None, // Can be fetched via inspect
        })
    }

    async fn delete_workload(&self, id: &str) -> ZeroResult<()> {
        self.client.remove_container(id, None).await
            .map_err(|e| ZeroError::Driver(format!("Docker remove error: {}", e)))?;
        Ok(())
    }

    async fn get_workload_status(&self, id: &str) -> ZeroResult<WorkloadStatus> {
        let inspect = self.client.inspect_container(id, None).await
            .map_err(|e| ZeroError::Driver(format!("Docker inspect error: {}", e)))?;

        let state = inspect.state.and_then(|s| s.status).map(|s| s.to_string()).unwrap_or("Unknown".into());
        
        Ok(WorkloadStatus {
            id: id.to_string(),
            state,
            ip_address: inspect.network_settings.and_then(|n| n.ip_address),
        })
    }

    async fn list_workloads(&self) -> ZeroResult<Vec<WorkloadStatus>> {
        let containers = self.client.list_containers::<String>(None).await
            .map_err(|e| ZeroError::Driver(format!("Docker list error: {}", e)))?;

        Ok(containers.into_iter().map(|c| WorkloadStatus {
            id: c.names.unwrap_or_default().get(0).cloned().unwrap_or_else(|| c.id.unwrap_or_default()),
            state: c.state.unwrap_or_else(|| "Unknown".into()),
            ip_address: None,
        }).collect())
    }

    async fn get_stats(&self) -> ZeroResult<zero_control_spi::NodeStats> {
        // Implementation for real Docker stats could use self.client.stats(...)
        // For now, return a placeholder to maintain SPI compliance
        Ok(zero_control_spi::NodeStats {
            cpu_usage_percent: 0.0,
            memory_used_mb: 0,
            memory_total_mb: 0,
            storage_used_gb: 0,
            storage_total_gb: 0,
        })
    }
}
