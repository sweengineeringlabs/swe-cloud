use zero_cli::{Cli, Commands, NetworkAction, execute_command};
use zero_control_core::ZeroProvider;
use zero_data_core::ZeroEngine;
use std::sync::Arc;

#[tokio::test]
async fn test_cli_network_create_flow() {
    // 1. Setup a Mock engine for reliable testing
    let compute = Arc::new(zero_data_core::driver::MockComputeDriver::new());
    let storage = Arc::new(zero_data_core::driver::FileSystemStorage::new(
        tempfile::tempdir().unwrap().path().to_path_buf()
    ));
    let network = Arc::new(zero_data_core::driver::MockNetworkDriver::new());
    let engine = ZeroEngine::new(compute, storage, network).unwrap();
    let provider = ZeroProvider::new(Arc::new(engine));

    // 2. Simulate the CLI command: network create --id "lab-subnet" --cidr "172.16.0.0/24"
    let command = Commands::Network {
        action: NetworkAction::Create {
            id: "lab-subnet".into(),
            cidr: "172.16.0.0/24".into(),
        },
    };

    // 3. Execute the CLI logic
    let result = execute_command(command, &provider).await;

    // 4. Verify the result
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_cli_native_flag_parsing() {
    use clap::Parser;
    
    // Simulate: zero --native network create --id test
    let args = vec!["zero", "--native", "network", "create", "--id", "test"];
    let cli = Cli::try_parse_from(args).unwrap();
    
    assert!(cli.native);
    if let Commands::Network { action } = cli.command {
        if let NetworkAction::Create { id, .. } = action {
            assert_eq!(id, "test");
        } else {
            panic!("Wrong action");
        }
    } else {
        panic!("Wrong command");
    }
}
