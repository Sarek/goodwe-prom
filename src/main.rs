#![feature(iter_next_chunk)]
#![feature(iter_advance_by)]

use std::str;

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
    /// Serve a metrics page that Prometheus can scrape
    Prometheus,
}

#[tokio::main]
async fn main() {
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
                    Some(())
                });
        }
        Commands::Prometheus => {
            use axum::{routing::get, Router};
            let target = cli
                .target
                .or_else(|| {
                    println!("When performing a request, a target must be provided!");
                    None
                })
                .unwrap();

            let app = Router::new().route("/", get(|| async { all_metrics(target).await }));

            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }
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
