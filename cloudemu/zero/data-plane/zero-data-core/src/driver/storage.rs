use zero_control_spi::{StorageDriver, ZeroResult, ZeroError, VolumeStatus};
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

pub struct FileSystemStorage {
    base_path: PathBuf,
}

impl FileSystemStorage {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }
}

#[async_trait]
impl StorageDriver for FileSystemStorage {
    async fn create_volume(&self, id: &str, _size_gb: i32) -> ZeroResult<VolumeStatus> {
        let path = self.base_path.join(id);
        fs::create_dir_all(&path).await
            .map_err(|e| ZeroError::Driver(format!("FS create error: {}", e)))?;
        
        Ok(VolumeStatus {
            id: id.to_string(),
            path: path.to_string_lossy().to_string(),
            state: "Available".to_string(),
        })
    }

    async fn delete_volume(&self, id: &str) -> ZeroResult<()> {
        let path = self.base_path.join(id);
        if path.exists() {
            fs::remove_dir_all(&path).await
                .map_err(|e| ZeroError::Driver(format!("FS delete error: {}", e)))?;
        }
        Ok(())
    }

    #[allow(clippy::suspicious_open_options)]
    async fn write_block(&self, volume_id: &str, offset: u64, data: Vec<u8>) -> ZeroResult<()> {
        use tokio::io::{AsyncWriteExt, AsyncSeekExt};
        let file_path = self.base_path.join(volume_id).join("data.bin");
        
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_path).await
            .map_err(|e| ZeroError::Driver(format!("FS open error: {}", e)))?;

        file.seek(std::io::SeekFrom::Start(offset)).await
            .map_err(|e| ZeroError::Driver(format!("FS seek error: {}", e)))?;
            
        file.write_all(&data).await
            .map_err(|e| ZeroError::Driver(format!("FS write error: {}", e)))?;
            
        Ok(())
    }

    async fn read_block(&self, volume_id: &str, offset: u64, length: u32) -> ZeroResult<Vec<u8>> {
        use tokio::io::{AsyncReadExt, AsyncSeekExt};
        let file_path = self.base_path.join(volume_id).join("data.bin");
        
        let mut file = fs::File::open(&file_path).await
            .map_err(|e| ZeroError::Driver(format!("FS open error: {}", e)))?;

        file.seek(std::io::SeekFrom::Start(offset)).await
            .map_err(|e| ZeroError::Driver(format!("FS seek error: {}", e)))?;
            
        let mut buffer = vec![0u8; length as usize];
        file.read_exact(&mut buffer).await
            .map_err(|e| ZeroError::Driver(format!("FS read error: {}", e)))?;
            
        Ok(buffer)
    }

    async fn list_volumes(&self) -> ZeroResult<Vec<VolumeStatus>> {
        let mut volumes = Vec::new();
        if let Ok(mut entries) = fs::read_dir(&self.base_path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if entry.file_type().await.map(|t| t.is_dir()).unwrap_or(false) {
                    let id = entry.file_name().to_string_lossy().to_string();
                    volumes.push(VolumeStatus {
                        id,
                        path: entry.path().to_string_lossy().to_string(),
                        state: "Available".to_string(),
                    });
                }
            }
        }
        Ok(volumes)
    }
}
