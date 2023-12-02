mod commands;

use clap::{Parser, Subcommand};

const YEAR: u32 = 2023;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Load inputs of a given day
    Inputs { day: u32, year: Option<u32> },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Inputs { day, year } => {
            commands::inputs::load_inputs(year.unwrap_or(YEAR), day).await?
        }
    };

    Ok(())
}
