#![doc = include_str!("../README.md")]
//! # Examples
//!```rust
//! # #[cfg(test)]
//! # mod tests {
//! # use std::io;
//! # use std::{env, fs};
//! # use std::path::PathBuf;
//! # use mockito::mock;
//! #
//! # #[test]
//! # fn main() -> io::Result<()> {
//! # let tripinfo_mock = mock("GET", "/metrorail/trips/K04/K05")
//! #           .with_status(200)
//! #          .with_body(metrohero_rs::schemas::tests::get_test_data(String::from("tripinfo_api.json")))
//! #          .create();
//! #
//! # let train_predictions_mock = mock("GET", "/metrorail/stations/K04/trains?includeScheduledPredictions=true")
//! #           .with_status(200)
//! #          .with_body(metrohero_rs::schemas::tests::get_test_data(String::from("station_train_predictions_api.json")))
//! #          .create();
//! #
//! use metrohero_rs::schemas::TripInfo;
//! use metrohero_rs::{MetroHeroClient, StationCode};
//! use metrohero_rs::schemas::{TrainPrediction, TrainPredictions};
//!
//! // Instantiate a client
//! let client = MetroHeroClient::new("YOUR_API_KEY".to_string());
//!
//! // Look up a station
//! let my_station = StationCode::from_name("Ballston")?;
//! assert_eq!(my_station, StationCode::K04);
//!
//! // Find departures for a station
//! let departures: Vec<TrainPrediction> = client.get_station_train_predictions(&my_station)?;
//!
//! // Look up trip information and duration.
//! let trip_info: TripInfo = client.get_trip_info(&my_station, &StationCode::K05)?;
//! # Ok(())
//! # }
//! # }
//! ```
#[macro_use]
extern crate lazy_static;

use crate::cli::run_cli;

// Re-export some important structs
#[doc(inline)]
pub use client::MetroHeroClient;
#[doc(inline)]
pub use errors::MetroHeroError;

#[doc(inline)]
pub use schemas::{LineCode, StationCode, TrainPrediction, TripInfo};

mod cli;
pub mod client;
pub mod errors;
mod format;
pub mod schemas;

#[doc(hidden)]
pub fn run() {
    run_cli();
}
