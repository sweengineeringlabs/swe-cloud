use clap::Parser;
use zero_control_facade::start_server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    #[arg(short, long)]
    native: bool,

    #[arg(short, long)]
    mock: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    
    if args.mock {
        println!("🧪 Starting ZeroCloud in MOCK mode...");
    } else if args.native {
        println!("🚀 Starting ZeroCloud in NATIVE mode (Hyper-V/KVM)...");
    } else {
        println!("🐋 Starting ZeroCloud in AUTO mode (Docker/Mock)...");
    }

    start_server(args.port, args.native, args.mock).await
}
