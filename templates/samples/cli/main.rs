use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "{{project_name}}")]
#[command(author, version, about = "Board sample application CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the HTTP server
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Run database migrations
    Migrate,
    /// Seed the database with sample data
    Seed,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { port } => {
            println!("Starting {{project_name}} server on port {}", port);
            {{project_name_snake}}_runtime::run(port).await?;
        }
        Commands::Migrate => {
            println!("Running database migrations...");
            // TODO: Implement migration logic
            println!("Migrations completed.");
        }
        Commands::Seed => {
            println!("Seeding database with sample data...");
            // TODO: Implement seed logic
            println!("Database seeded.");
        }
    }

    Ok(())
}
