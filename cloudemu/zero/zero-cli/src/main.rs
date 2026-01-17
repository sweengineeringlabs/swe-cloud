use clap::Parser;
use zero_cli::{Cli, run_cli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run_cli(cli).await
}
