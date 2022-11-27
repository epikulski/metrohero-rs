//! A CLI for interacting with the MetroHero API.
use std::process;
use std::str::FromStr;

use clap::Parser;
use clap::{arg, Subcommand};

use crate::errors::MetroHeroError;
use crate::schemas::StationCode;
use crate::{client, format};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long, help = "MetroHero API key")]
    api_key: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get information about a route between two stations.
    Plan {
        #[arg()]
        start_station: String,
        #[arg()]
        end_station: String,
    },

    /// Get information about a Metrorail station.
    Departures {
        #[arg()]
        station: String,
    },

    /// Print a table of station names and their RTU codes.
    Stations {},
}

fn parse_user_station_input(input: &str) -> StationCode {
    //     First, see if it is an exact match for a station code.
    let code_attempt = StationCode::from_str(input);

    //     Next, see if it is an exact match for a station name.
    let name_attempt = StationCode::from_name(input);

    if let Ok(..) = code_attempt {
        code_attempt.unwrap()
    } else if let Ok(..) = name_attempt {
        name_attempt.unwrap()
    } else {
        eprintln!("{}", MetroHeroError::InvalidStation);
        process::exit(1);
    }
}

pub fn run_cli() {
    let cli = Cli::parse();

    let client = match cli.api_key {
        Some(api_key) => client::MetroHeroClient::new(api_key),
        _ => client::MetroHeroClient::default(),
    };

    match cli.command {
        Commands::Departures {
            station: station_input,
        } => {
            let station_code = parse_user_station_input(&station_input);
            let departures = match client.get_station_train_predictions(&station_code) {
                Ok(departures) => departures,
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            };
            let station_tags = client.get_station_report(&station_code).unwrap();
            format::print_departures(&station_code, departures, station_tags);
        }

        Commands::Plan {
            start_station: start_station_input,
            end_station: end_station_input,
        } => {
            let start_station = parse_user_station_input(&start_station_input);
            let end_station = parse_user_station_input(&end_station_input);
            let plan = client.get_trip_info(&start_station, &end_station);
            match plan {
                Ok(plan) => format::print_plan(plan),
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }
        }

        Commands::Stations {} => {
            format::print_stations();
        }
    }
}

#[test]
fn test_stations_name() {
    format::print_stations();
}
