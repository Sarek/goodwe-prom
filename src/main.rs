use std::{io, str};

use clap::{Parser, Subcommand};

mod discovery;
mod protocol;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Discover GoodWe inverters
    Discover,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Discover => discovery::discover_inverters().await,
    }
}
