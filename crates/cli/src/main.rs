use anyhow::Result;
use boots_core::{adder::add, generator::generate};
use clap::{Parser, Subcommand, command};

#[derive(Parser)]
#[command(name = "cargo-boots")]
#[command(bin_name = "cargo")]
#[command(author, version, about, long_about=None)]
struct CargoBootsCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Boots {
        #[command(subcommand)]
        subcommand: BootsCommands,
    },
}

#[derive(Parser)]
#[command(name = "boots")]
#[command(bin_name = "boots")]
#[command(author, version, about, long_about=None)]
struct BootsCli {
    #[command(subcommand)]
    command: BootsCommands,
}

#[derive(Subcommand)]
enum BootsCommands {
    Generate { name: Option<String> },
    Add { target: String },
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let command = if args[0].ends_with("cargo-boots") || args[0].ends_with("cargo") {
        // Must be annotated, why?? :[
        let cli = CargoBootsCli::parse();
        match cli.command {
            Commands::Boots { subcommand } => subcommand,
        }
    } else {
        // boots 형태로 실행
        let cli = BootsCli::parse();
        cli.command
    };

    match command {
        BootsCommands::Generate { name } => {
            generate(name)?;
        }
        BootsCommands::Add { target } => {
            add(&target)?;
        }
    }

    Ok(())
}
