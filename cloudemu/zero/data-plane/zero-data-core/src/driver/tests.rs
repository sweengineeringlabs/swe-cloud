use super::storage::FileSystemStorage;
use super::mock::MockComputeDriver;
use zero_control_spi::{StorageDriver, ComputeDriver, NetworkDriver};
use std::path::PathBuf;
use tempfile::tempdir;

#[tokio::test]
async fn test_file_system_storage_driver() {
    let dir = tempdir().unwrap();
    let storage = FileSystemStorage::new(dir.path().to_path_buf());
    
    // Test Volume Creation
    let vol = storage.create_volume("test-vol", 10).await.unwrap();
    assert_eq!(vol.id, "test-vol");
    assert!(PathBuf::from(&vol.path).exists());

    // Test Write/Read Block
    let data = vec![1, 2, 3, 4, 5];
    storage.write_block("test-vol", 0, data.clone()).await.unwrap();
    
    let read_data = storage.read_block("test-vol", 0, 5).await.unwrap();
    assert_eq!(read_data, data);

    // Test Offset Write/Read
    let offset_data = vec![9, 9, 9];
    storage.write_block("test-vol", 2, offset_data.clone()).await.unwrap();
    
    // File should now be [1, 2, 9, 9, 9]
    let full_data = storage.read_block("test-vol", 0, 5).await.unwrap();
    assert_eq!(full_data, vec![1, 2, 9, 9, 9]);

    // Test Delete
    storage.delete_volume("test-vol").await.unwrap();
    assert!(!PathBuf::from(&vol.path).exists());
}

#[tokio::test]
async fn test_mock_compute_driver() {
    let driver = MockComputeDriver::new();
    
    // Test Create
    let status = driver.create_workload("vm-1", "alpine", 1.0, 256).await.unwrap();
    assert_eq!(status.id, "vm-1");
    assert_eq!(status.state, "Running");

    // Test Get Status
    let current_status = driver.get_workload_status("vm-1").await.unwrap();
    assert_eq!(current_status.state, "Running");

    // Test Delete
    driver.delete_workload("vm-1").await.unwrap();
    let result = driver.get_workload_status("vm-1").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_mock_network_driver() {
    use super::mock::MockNetworkDriver;
    let driver = MockNetworkDriver::new();

    // Test Create Network
    let status = driver.create_network("net-1", "10.0.0.0/24").await.unwrap();
    assert_eq!(status.id, "net-1");
    assert_eq!(status.cidr, "10.0.0.0/24");

    // Test Connect Workload
    let ip = driver.connect_workload("vm-1", "net-1").await.unwrap();
    assert_eq!(ip, "10.0.0.50");

    // Test Delete Network
    driver.delete_network("net-1").await.unwrap();
}
