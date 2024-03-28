#![feature(iter_next_chunk)]
#![feature(iter_advance_by)]

use std::str;

use clap::{Parser, Subcommand};

mod discovery;
mod identify;

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
    /// Identify the inverter and print the serial number and firmware version
    Identify,
}

#[derive(Subcommand)]
enum Query {
    InverterInfo,
    RunningInfo,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Discover => {
            let _ = discovery::discover_inverters();
        }
        Commands::Identify => {
            cli.target
                .or_else(|| {
                    println!("When performing a request, a target must be provided!");
                    None
                })
                .and_then(|target| match identify::query_id(&target) {
                    Ok(id) => {
                        println!("Inverter Identification");
                        println!(" - Serial Number: {}", id.serial_number);
                        println!(" - Firmware: {}", id.firmware);
                        None::<String>
                    }
                    Err(e) => {
                        println!("Error while identifying inverter: {e}");
                        None::<String>
                    }
                });
        }
    }
}
