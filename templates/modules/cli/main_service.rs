use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "{{project_name}}")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Port to listen on
    #[arg(short, long, default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    {{project_name_snake}}_runtime::run(cli.port).await?;

    Ok(())
}
