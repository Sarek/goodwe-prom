#![feature(iter_next_chunk)]
#![feature(iter_advance_by)]

use std::{process::ExitCode, str};

use axum::http::StatusCode;
use clap::{Parser, Subcommand};

mod discovery;
mod identify;
mod metrics;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// The IP address of the inverter to talk to
    #[clap(long, env)]
    target: Option<String>,
    /// The port number to serve the Prometheus metrics from (only required in the Prometheus mode)
    #[clap(long, env)]
    port: Option<u16>,
}

#[derive(Subcommand)]
enum Commands {
    /// Discover GoodWe inverters
    Discover,
    /// Identify the inverter and print the serial number and firmware version
    Identify,
    /// Metrics
    Metrics,
    /// Serve a metrics page that Prometheus can scrape
    Prometheus,
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    if let Some(target) = cli.target {
        match &cli.command {
            Commands::Discover => {
                let _ = discovery::discover_inverters();
                ExitCode::SUCCESS
            }
            Commands::Identify => match identify::query_id(&target) {
                Ok(id) => {
                    println!("Inverter Identification");
                    println!(" - Serial Number: {}", id.serial_number);
                    println!(" - Firmware: {}", id.firmware);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    println!("Error while identifying inverter: {e}");
                    ExitCode::FAILURE
                }
            },
            Commands::Metrics => {
                let metric_sets = vec![
                    metrics::et::base_metrics(),
                    metrics::et::battery_metrics(),
                    metrics::et::meter_metrics(),
                ];
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
                ExitCode::SUCCESS
            }
            Commands::Prometheus => {
                use axum::{routing::get, Router};

                let app = Router::new().route("/", get(|| async { all_metrics(target).await }));

                let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
                    .await
                    .unwrap();
                axum::serve(listener, app).await.unwrap();
                ExitCode::SUCCESS
            }
        }
    } else {
        println!("Please provide a target either as a command line argument or in the GOODWE_TARGET environment variable!");
        ExitCode::FAILURE
    }
}

type ResponseWithCode = (StatusCode, String);
type ResponseResult = Result<String, ResponseWithCode>;

async fn all_metrics(target: String) -> ResponseResult {
    use std::fmt::Write as _;

    let mut response = String::new();

    let metric_sets = vec![
        metrics::et::base_metrics(),
        metrics::et::battery_metrics(),
        metrics::et::meter_metrics(),
    ];
    for mut metric_set in metric_sets {
        match metrics::get_metrics(&target, &mut metric_set) {
            Ok(_) => match write!(&mut response, "{}", metric_set) {
                Ok(_) => (),
                Err(e) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to transform metrics: {e}"),
                    ));
                }
            },
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error retrieving metrics: {e}"),
                ));
            }
        }
    }
    Ok(response)
}
