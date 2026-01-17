use clap::{Parser, Subcommand};
use zero_control_core::ZeroProvider;
use zero_data_core::ZeroEngine;
use zero_control_spi::{ZeroRequest, ZeroService};
use std::sync::Arc;
use colored::*;
use serde_json::json;

#[derive(Parser)]
#[command(name = "zero")]
#[command(about = "ZeroCloud Private Cloud CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Force the use of native OS drivers (Hyper-V / KVM) instead of Docker
    #[arg(long, global = true)]
    pub native: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage Workloads
    Workload {
        #[command(subcommand)]
        action: WorkloadAction,
    },
    /// Manage Volumes
    Volume {
        #[command(subcommand)]
        action: VolumeAction,
    },
    /// Manage Nodes
    Node {
        #[command(subcommand)]
        action: NodeAction,
    },
    /// Manage Networks
    Network {
        #[command(subcommand)]
        action: NetworkAction,
    },
    /// Manage Store (S3)
    Store {
        #[command(subcommand)]
        action: StoreAction,
    },
    /// Manage Database (DynamoDB)
    Db {
        #[command(subcommand)]
        action: DbAction,
    },
    /// Manage Functions (Lambda)
    Func {
        #[command(subcommand)]
        action: FuncAction,
    },
    /// Manage Queues (SQS)
    Queue {
        #[command(subcommand)]
        action: QueueAction,
    },
    /// Manage Identity (IAM)
    Iam {
        #[command(subcommand)]
        action: IamAction,
    },
    /// Manage Load Balancers
    Lb {
        #[command(subcommand)]
        action: LbAction,
    },
}

#[derive(Subcommand)]
pub enum WorkloadAction {
    /// Create a new workload
    Up {
        #[arg(short, long)]
        id: String,
        #[arg(short, long)]
        image: String,
    },
    /// Delete a workload
    Down {
        #[arg(short, long)]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum VolumeAction {
    /// Create a new volume
    Create {
        #[arg(short, long)]
        id: String,
        #[arg(short, long)]
        size: i32,
    },
}

#[derive(Subcommand)]
pub enum NodeAction {
    /// List all nodes
    List,
}

#[derive(Subcommand)]
pub enum NetworkAction {
    /// Create a new virtual network
    Create {
        #[arg(short, long)]
        id: String,
        #[arg(short, long, default_value = "10.0.0.0/24")]
        cidr: String,
    },
}

#[derive(Subcommand)]
pub enum StoreAction {
    /// Create a bucket
    Create { #[arg(short, long)] name: String },
    /// List buckets
    Ls,
}

#[derive(Subcommand)]
pub enum DbAction {
    /// Create a table
    Create { #[arg(short, long)] name: String, #[arg(short, long, default_value="id")] pk: String },
    /// List tables
    Ls,
}

#[derive(Subcommand)]
pub enum FuncAction {
    /// Deploy a function
    Deploy { #[arg(short, long)] name: String, #[arg(short, long)] code: String, #[arg(short, long)] handler: String },
    /// Invoke a function
    Invoke { #[arg(short, long)] name: String, #[arg(short, long)] payload: String },
    /// List functions
    Ls,
}

#[derive(Subcommand)]
pub enum QueueAction {
    /// Create a queue
    Create { #[arg(short, long)] name: String },
    /// Send a message
    Send { #[arg(short, long)] name: String, #[arg(short, long)] body: String },
    /// Receive messages (with Visibility Timeout)
    Receive { #[arg(short, long)] name: String },
    /// Delete a message (using ReceiptHandle)
    Delete { #[arg(short, long)] name: String, #[arg(long)] handle: String },
    /// List queues
    Ls,
}

#[derive(Subcommand)]
pub enum IamAction {
    /// Create a user
    CreateUser { #[arg(long)] username: String },
    /// Attach a policy
    AttachPolicy { #[arg(long)] username: String, #[arg(short, long)] policy: String }, 
    /// List users
    ListUsers,
    /// Create a role
    CreateRole { #[arg(long)] rolename: String },
    /// List roles
    ListRoles,
    /// Create a group
    CreateGroup { #[arg(long)] groupname: String },
    /// List groups
    ListGroups,
}

#[derive(Subcommand)]
pub enum LbAction {
    /// Create a Load Balancer
    Create { #[arg(short, long)] name: String, #[arg(short, long, default_value="application")] lb_type: String },
    /// Create a Target Group
    CreateTargetGroup { #[arg(short, long)] name: String, #[arg(short, long, default_value_t=80)] port: i32 },
    /// Register target to group
    Register { #[arg(long)] group: String, #[arg(long)] id: String, #[arg(short, long, default_value_t=80)] port: i32 },
    /// Create a Listener
    CreateListener { #[arg(long)] lb: String, #[arg(short, long)] port: i32, #[arg(long)] target_group: String },
    /// List Load Balancers
    Ls,
}

pub async fn run_cli(cli: Cli) -> anyhow::Result<()> {
    check_wsl_preflight();
    let engine = if cli.native {
        println!("{} forcing native OS drivers...", "🔧".blue());
        ZeroEngine::native()
    } else {
        ZeroEngine::auto()
    }.map_err(|e| anyhow::anyhow!("Failed to init ZeroEngine: {}", e))?;
    
    let provider = ZeroProvider::new(Arc::new(engine));
    execute_command(cli.command, &provider).await
}

fn check_wsl_preflight() {
    #[cfg(target_os = "linux")]
    {
        let is_wsl = std::fs::read_to_string("/proc/version")
            .map(|v| v.to_lowercase().contains("microsoft") || v.to_lowercase().contains("wsl"))
            .unwrap_or(false);

        if is_wsl {
             let kvm_exists = std::path::Path::new("/dev/kvm").exists();
             if !kvm_exists {
                 println!("{}", "⚠️  WSL 2 detected but /dev/kvm is missing!".yellow().bold());
                 println!("{}", "To enable KVM on WSL 2, ensure nested virtualization is enabled in your .wslconfig:".yellow());
                 println!("{}", "  [wsl2]\n  nestedVirtualization=true".white());
                 println!("{}", "Then run 'wsl --shutdown' in PowerShell and restart your terminal.".yellow());
                 println!();
             }
        }
    }
}

pub async fn execute_command(command: Commands, provider: &ZeroProvider) -> anyhow::Result<()> {
    match command {
        Commands::Workload { action } => match action {
            WorkloadAction::Up { id, image } => {
                println!("{} Workload {} with image {}...", "🚀 Starting".green(), id.bold(), image.cyan());
                let req = ZeroRequest {
                    method: "POST".into(),
                    path: "/v1/workloads".into(),
                    headers: std::collections::HashMap::new(),
                    body: json!({ "id": id, "image": image }).to_string().into_bytes(),
                };
                let resp = provider.handle_request(req).await?;
                println!("{} Response: {}", "✅".green(), String::from_utf8_lossy(&resp.body));
            }
            WorkloadAction::Down { id } => {
                println!("{} Workload {}...", "🛑 Stopping".red(), id.bold());
                let req = ZeroRequest {
                    method: "DELETE".into(),
                    path: "/v1/workloads".into(),
                    headers: std::collections::HashMap::new(),
                    body: json!({ "id": id }).to_string().into_bytes(),
                };
                let resp = provider.handle_request(req).await?;
                println!("{} Response: {}", "✅".green(), String::from_utf8_lossy(&resp.body));
            }
        },
        Commands::Volume { action } => match action {
            VolumeAction::Create { id, size } => {
                println!("{} Volume {} ({} GB)...", "📂 Provisioning".blue(), id.bold(), size);
                let req = ZeroRequest {
                    method: "POST".into(),
                    path: "/v1/volumes".into(),
                    headers: std::collections::HashMap::new(),
                    body: json!({ "id": id, "size_gb": size }).to_string().into_bytes(),
                };
                let resp = provider.handle_request(req).await?;
                println!("{} Response: {}", "✅".green(), String::from_utf8_lossy(&resp.body));
            }
        },
        Commands::Node { action } => match action {
            NodeAction::List => {
                let req = ZeroRequest {
                    method: "GET".into(),
                    path: "/v1/nodes".into(),
                    headers: std::collections::HashMap::new(),
                    body: vec![],
                };
                let resp = provider.handle_request(req).await?;
                println!("{}", "📋 Local Compute Nodes:".bold().underline());
                println!("{}", String::from_utf8_lossy(&resp.body));
            }
        },
        Commands::Network { action } => match action {
            NetworkAction::Create { id, cidr } => {
                println!("{} Network {} with CIDR {}...", "🌐 Creating".cyan(), id.bold(), cidr.yellow());
                let req = ZeroRequest {
                    method: "POST".into(),
                    path: "/v1/networks".into(),
                    headers: std::collections::HashMap::new(),
                    body: json!({ "id": id, "cidr": cidr }).to_string().into_bytes(),
                };
                let resp = provider.handle_request(req).await?;
                println!("{} Response: {}", "✅".green(), String::from_utf8_lossy(&resp.body));
            }
        },
        Commands::Store { action } => match action {
            StoreAction::Create { name } => {
                println!("{} Bucket {}...", "📦 Creating".blue(), name);
                let req = ZeroRequest {
                     method: "POST".into(),
                     path: "/v1/store/buckets".into(),
                     headers: std::collections::HashMap::new(),
                     body: json!({ "name": name }).to_string().into_bytes()
                 };
                let resp = provider.handle_request(req).await?;
                println!("{}", String::from_utf8_lossy(&resp.body));
            }
            StoreAction::Ls => {
                let req = ZeroRequest {
                    method: "GET".into(),
                    path: "/v1/store/buckets".into(),
                    headers: std::collections::HashMap::new(),
                    body: vec![],
                };
                let resp = provider.handle_request(req).await?;
                println!("{}", String::from_utf8_lossy(&resp.body));
            }
        },
        Commands::Db { action } => match action {
             DbAction::Create { name, pk } => {
                 println!("{} Table {} (PK: {})...", "📊 Creating".blue(), name, pk);
                 let req = ZeroRequest {
                     method: "POST".into(), 
                     path: "/v1/db/tables".into(),
                     headers: std::collections::HashMap::new(),
                     body: json!({ "name": name, "pk": pk }).to_string().into_bytes()
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
             }
             DbAction::Ls => {
                 let req = ZeroRequest {
                     method: "GET".into(), 
                     path: "/v1/db/tables".into(),
                     headers: std::collections::HashMap::new(),
                     body: vec![]
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
             }
        },
        Commands::Func { action } => match action {
            FuncAction::Deploy { name, code, handler } => {
                 let code_content = if std::path::Path::new(&code).exists() {
                     std::fs::read_to_string(&code).unwrap_or(code)
                 } else {
                     code
                 };
                 println!("{} Function {}...", "⚡ Deploying".yellow(), name);
                 let req = ZeroRequest {
                     method: "POST".into(),
                     path: "/v1/func/functions".into(),
                     headers: std::collections::HashMap::new(),
                     body: json!({ "name": name, "code": code_content, "handler": handler }).to_string().into_bytes()
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
            }
            FuncAction::Invoke { name, payload } => {
                 println!("{} Function {}...", "▶️ Invoking".green(), name);
                 let req = ZeroRequest {
                     method: "POST".into(),
                     path: format!("/v1/func/functions/{}/invocations", name),
                     headers: std::collections::HashMap::new(),
                     body: payload.into_bytes()
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
            }
            FuncAction::Ls => {
                 let req = ZeroRequest {
                     method: "GET".into(), 
                     path: "/v1/func/functions".into(),
                     headers: std::collections::HashMap::new(),
                     body: vec![]
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
            }
        },
        Commands::Queue { action } => match action {
            QueueAction::Create { name } => {
                println!("{} Queue {}...", "📥 Creating".magenta(), name);
                let req = ZeroRequest {
                    method: "POST".into(),
                    path: "/v1/queue/queues".into(),
                    headers: std::collections::HashMap::new(),
                    body: json!({ "name": name }).to_string().into_bytes()
                };
                let resp = provider.handle_request(req).await?;
                println!("{}", String::from_utf8_lossy(&resp.body));
            }
            QueueAction::Send { name, body } => {
                 println!("{} Message to {}...", "📨 Sending".magenta(), name);
                 let req = ZeroRequest {
                     method: "POST".into(),
                     path: format!("/v1/queue/queues/{}/messages", name),
                     headers: std::collections::HashMap::new(),
                     body: json!({ "body": body }).to_string().into_bytes()
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
            }
            QueueAction::Receive { name } => {
                 let req = ZeroRequest {
                     method: "GET".into(),
                     path: format!("/v1/queue/queues/{}/messages", name),
                     headers: std::collections::HashMap::new(),
                     body: vec![]
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
            }
            QueueAction::Delete { name, handle } => {
                 let req = ZeroRequest {
                     method: "DELETE".into(),
                     path: format!("/v1/queue/queues/{}/messages/{}", name, handle),
                     headers: std::collections::HashMap::new(),
                     body: vec![]
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
            }
            QueueAction::Ls => {
                 let req = ZeroRequest {
                     method: "GET".into(),
                     path: "/v1/queue/queues".into(),
                     headers: std::collections::HashMap::new(),
                     body: vec![]
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
            }
        },
        Commands::Iam { action } => match action {
             IamAction::CreateUser { username } => {
                 println!("{} User {}...", "👤 Creating".cyan(), username);
                 let req = ZeroRequest {
                     method: "POST".into(),
                     path: "/v1/iam/users".into(),
                     headers: std::collections::HashMap::new(),
                     body: json!({ "username": username }).to_string().into_bytes()
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
             }
             IamAction::CreateRole { rolename } => {
                 println!("{} Role {}...", "🎭 Creating".cyan(), rolename);
                 let req = ZeroRequest {
                     method: "POST".into(),
                     path: "/v1/iam/roles".into(),
                     headers: std::collections::HashMap::new(),
                     body: json!({ "Rolename": rolename }).to_string().into_bytes()
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
             }
             IamAction::CreateGroup { groupname } => {
                 println!("{} Group {}...", "👥 Creating".cyan(), groupname);
                 let req = ZeroRequest {
                     method: "POST".into(),
                     path: "/v1/iam/groups".into(),
                     headers: std::collections::HashMap::new(),
                     body: json!({ "Groupname": groupname }).to_string().into_bytes()
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
             }
             IamAction::AttachPolicy { username, policy } => {
                 println!("{} Policy to {}...", "🔐 Attaching".cyan(), username);
                 let req = ZeroRequest {
                     method: "POST".into(),
                     path: format!("/v1/iam/users/{}/policy", username),
                     headers: std::collections::HashMap::new(),
                     body: json!({ "PolicyDocument": policy }).to_string().into_bytes()
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
             }
             IamAction::ListUsers => {
                 let req = ZeroRequest {
                     method: "GET".into(),
                     path: "/v1/iam/users".into(),
                     headers: std::collections::HashMap::new(),
                     body: vec![]
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
             }
             IamAction::ListRoles => {
                 let req = ZeroRequest {
                     method: "GET".into(),
                     path: "/v1/iam/roles".into(),
                     headers: std::collections::HashMap::new(),
                     body: vec![]
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
             }
             IamAction::ListGroups => {
                 let req = ZeroRequest {
                     method: "GET".into(),
                     path: "/v1/iam/groups".into(),
                     headers: std::collections::HashMap::new(),
                     body: vec![]
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
             }
        },
        Commands::Lb { action } => match action {
             LbAction::Create { name, lb_type } => {
                 println!("{} LB {}...", "⚖️ Creating".white(), name);
                 let req = ZeroRequest {
                     method: "POST".into(),
                     path: "/v1/network/loadbalancers".into(),
                     headers: std::collections::HashMap::new(),
                     body: json!({ "name": name, "type": lb_type }).to_string().into_bytes()
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
             }
             LbAction::CreateTargetGroup { name, port } => {
                 println!("{} Target Group {} on port {}...", "🎯 Creating".white(), name, port);
                 let req = ZeroRequest {
                     method: "POST".into(),
                     path: "/v1/network/targetgroups".into(),
                     headers: std::collections::HashMap::new(),
                     body: json!({ "name": name, "port": port }).to_string().into_bytes()
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
             }
             LbAction::Register { group, id, port } => {
                 println!("{} Target {} to group {}...", "🔗 Registering".white(), id, group);
                 let req = ZeroRequest {
                     method: "POST".into(),
                     path: format!("/v1/network/targetgroups/{}/targets", group),
                     headers: std::collections::HashMap::new(),
                     body: json!({ "id": id, "port": port }).to_string().into_bytes()
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
             }
             LbAction::CreateListener { lb, port, target_group } => {
                 println!("{} Listener for {} on port {}...", "👂 Creating".white(), lb, port);
                 let req = ZeroRequest {
                     method: "POST".into(),
                     path: "/v1/network/listeners".into(),
                     headers: std::collections::HashMap::new(),
                     body: json!({ "load_balancer_name": lb, "port": port, "target_group_arn": target_group }).to_string().into_bytes()
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
             }
             LbAction::Ls => {
                 let req = ZeroRequest {
                     method: "GET".into(),
                     path: "/v1/network/loadbalancers".into(),
                     headers: std::collections::HashMap::new(),
                     body: vec![]
                 };
                 let resp = provider.handle_request(req).await?;
                 println!("{}", String::from_utf8_lossy(&resp.body));
             }
        },
    }

    Ok(())
}
