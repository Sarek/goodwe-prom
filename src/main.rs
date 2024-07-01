#![feature(iter_next_chunk)]
#![feature(iter_advance_by)]

use std::str;

use clap::{Parser, Subcommand};
use metrics::MetricSet;

mod discovery;
mod identify;
mod metrics;

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
    /// Metrics
    Metrics,
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
        Commands::Metrics => {
            cli.target
                .or_else(|| {
                    println!("When performing a request, a target must be provided!");
                    None
                })
                /* `get_base_metrics` umbauen, sodass man ein `&mut MetricSet` reingeben
                 * kann. Dann hier einen Vektor aus Metric Sets bauen und drüber iterieren.
                 */
                .and_then(|target| {
                    let metric_sets =
                        vec![metrics::et::base_metrics(), metrics::et::battery_metrics()];
                    for mut metric_set in metric_sets {
                        match metrics::get_metrics(&target, &mut metric_set) {
                            Ok(_) => {
                                println!("{}", metric_set);
                            }
                            Err(e) => {
                                println!("Error retrieving metrics: {e}");
                            }
                        }
                    }
                    Some(())
                });
        }
    }
}
