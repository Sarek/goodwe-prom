use std::{io, str};

use clap::{Parser, Subcommand};
use protocol::RequestMessage;

mod discovery;
mod protocol;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    target: Option<String>
}

#[derive(Subcommand)]
enum Commands {
    /// Discover GoodWe inverters
    Discover,
    /// Request a specific register
    #[command(subcommand)]
    Request(Query),
}

#[derive(Subcommand)]
enum Query {
    InverterInfo,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Discover => discovery::discover_inverters(),
        _ => todo!("Function not implemented yet!"),
    }
}
