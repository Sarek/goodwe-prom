use std::str;

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
    target: Option<String>,
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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Discover => {
            let _ = discovery::discover_inverters();
        }
        Commands::Request(Query::InverterInfo) => {
            cli.target
                .and_then(|target| {
                    protocol::send_request(&target, RequestMessage::QueryIdInfo).ok()
                })
                .or_else(|| {
                    println!("When performing a request, a target must be provided!");
                    None
                });
        }
    }
}
