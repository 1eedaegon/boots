use anyhow::Result;
use boots_core::{ProjectGenerator, ProjectType, parse_options};
use clap::{Parser, Subcommand};
use std::env;

#[derive(Parser)]
#[command(name = "cargo-boots")]
#[command(bin_name = "cargo")]
#[command(author, version)]
#[command(about = "Bootstrap modular Rust projects")]
#[command(
    long_about = "A CLI tool for bootstrapping modular Rust project structures.\n\n\
    Creates workspace-based projects with optional modules like API, runtime, persistence, and more."
)]
#[command(after_help = "Examples:\n  \
    cargo boots service my-api --options postgres,grpc\n  \
    cargo boots cli my-tool --options client\n  \
    cargo boots lib my-crate")]
struct CargoBootsCli {
    #[command(subcommand)]
    command: CargoCommands,
}

#[derive(Subcommand)]
enum CargoCommands {
    Boots {
        #[command(subcommand)]
        subcommand: BootsCommands,
    },
}

#[derive(Parser)]
#[command(name = "boots")]
#[command(bin_name = "boots")]
#[command(author, version)]
#[command(about = "Bootstrap modular Rust projects")]
#[command(
    long_about = "A CLI tool for bootstrapping modular Rust project structures.\n\n\
    Creates workspace-based projects with optional modules like API, runtime, persistence, and more."
)]
#[command(after_help = "Examples:\n  \
    boots service my-api --options postgres,grpc\n  \
    boots cli my-tool --options client\n  \
    boots lib my-crate")]
struct BootsCli {
    #[command(subcommand)]
    command: BootsCommands,
}

#[derive(Subcommand, Clone)]
enum BootsCommands {
    /// Create a full-stack service project
    #[command(
        long_about = "Creates a service project with the following modules:\n  \
        - core: Business logic and domain types\n  \
        - api: HTTP/gRPC handlers and routes\n  \
        - runtime: Server startup and configuration\n  \
        - cli: Command-line interface\n  \
        - persistence: Database access layer"
    )]
    Service {
        /// Project name (e.g., my-api, user-service)
        #[arg(value_name = "NAME")]
        name: String,

        /// Comma-separated options: postgres, sqlite, grpc, http
        #[arg(short, long, value_name = "OPTIONS")]
        #[arg(help = "Additional features [possible: postgres, sqlite, grpc, http]")]
        options: Option<String>,
    },

    /// Create a CLI application project
    #[command(long_about = "Creates a CLI project with the following modules:\n  \
        - core: Business logic and domain types\n  \
        - cli: Command-line interface\n\n\
        Optional modules via --options:\n  \
        - client: HTTP client for external APIs\n  \
        - persistence: Local data storage")]
    Cli {
        /// Project name (e.g., my-tool, file-processor)
        #[arg(value_name = "NAME")]
        name: String,

        /// Comma-separated options: client, persistence
        #[arg(short, long, value_name = "OPTIONS")]
        #[arg(help = "Additional features [possible: client, persistence]")]
        options: Option<String>,
    },

    /// Create a library crate project
    #[command(long_about = "Creates a minimal library project with:\n  \
        - core: Library code with examples\n  \
        - Basic documentation setup\n  \
        - Example usage files")]
    Lib {
        /// Library name (e.g., my-lib, utils)
        #[arg(value_name = "NAME")]
        name: String,
    },

    /// Create a sample board application with RBAC
    #[command(
        long_about = "Creates a full-stack board (게시판) sample project with:\n  \
        - Role-based access control (Admin, Writer, Reader)\n  \
        - Posts: Writer/Admin can edit own/all posts\n  \
        - Comments: Reader/Admin can edit own/all comments\n  \
        - File upload with image preview\n  \
        - E2E tests with Playwright\n  \
        - PostgreSQL + MinIO (S3) + React SPA"
    )]
    Sample {
        /// Project name (e.g., my-board, community)
        #[arg(value_name = "NAME")]
        name: String,

        /// Options (use 'sample' to create full board project)
        #[arg(short, long, value_name = "OPTIONS")]
        #[arg(help = "Use 'sample' to create full board project (ignores other options)")]
        options: Option<String>,
    },
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let is_cargo_subcommand =
        (args.len() > 1 && args[1] == "boots") || args[0].ends_with("cargo-boots");

    let command = if is_cargo_subcommand {
        let cli = CargoBootsCli::parse();
        match cli.command {
            CargoCommands::Boots { subcommand } => subcommand,
        }
    } else {
        let cli = BootsCli::parse();
        cli.command
    };

    let config = match command {
        BootsCommands::Service { name, options } => {
            parse_options(ProjectType::Service, &name, options.as_deref())?
        }
        BootsCommands::Cli { name, options } => {
            parse_options(ProjectType::Cli, &name, options.as_deref())?
        }
        BootsCommands::Lib { name } => parse_options(ProjectType::Lib, &name, None)?,
        BootsCommands::Sample { name, options } => {
            // Default to 'sample' option if none provided
            let opts = options.unwrap_or_else(|| "sample".to_string());
            parse_options(ProjectType::Sample, &name, Some(&opts))?
        }
    };

    let generator = ProjectGenerator::new(config.clone());
    generator.generate(&env::current_dir()?)?;

    println!("Project '{}' created successfully!", config.name);
    Ok(())
}
