use anyhow::Result;
use boots_core::{ProjectGenerator, ProjectType, parse_options};
use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
use std::env;

const ABOUT: &str = "Bootstrap modular Rust projects";
const LONG_ABOUT: &str = "A CLI tool for bootstrapping modular Rust project structures.\n\n\
    Creates workspace-based projects with optional modules like API, runtime, persistence, and more.";

fn examples(prefix: &str) -> String {
    format!(
        "Examples:\n  \
        {prefix} service my-api --options postgres,grpc\n  \
        {prefix} cli my-tool --options client\n  \
        {prefix} lib my-crate"
    )
}

#[derive(Parser)]
#[command(name = "boots", bin_name = "boots")]
#[command(author, version, about = ABOUT, long_about = LONG_ABOUT)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
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

    // When called via `cargo boots`, cargo passes "boots" as first argument
    // Skip it so clap parses correctly
    let is_cargo_subcommand = args.len() > 1 && args[1] == "boots";
    let bin_name = if is_cargo_subcommand {
        "cargo boots"
    } else {
        "boots"
    };

    // Build command with dynamic bin_name and examples
    let cmd = Cli::command()
        .bin_name(bin_name)
        .after_help(examples(bin_name));

    let cli = if is_cargo_subcommand {
        let matches = cmd.get_matches_from(args.iter().take(1).chain(args.iter().skip(2)));
        Cli::from_arg_matches(&matches)?
    } else {
        let matches = cmd.get_matches();
        Cli::from_arg_matches(&matches)?
    };

    let config = match cli.command {
        Commands::Service { name, options } => {
            parse_options(ProjectType::Service, &name, options.as_deref())?
        }
        Commands::Cli { name, options } => {
            parse_options(ProjectType::Cli, &name, options.as_deref())?
        }
        Commands::Lib { name } => parse_options(ProjectType::Lib, &name, None)?,
        Commands::Sample { name, options } => {
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
