//! Helpers for printing MetroHero CLI content to the console.
use std::cmp;

use comfy_table::{Cell, CellAlignment, Row, Table};
use crossterm::style::{Attribute, Color, Stylize};
use strum::IntoEnumIterator;

use crate::schemas::{StationCode, StationTags, TrainPrediction, TripInfo};

fn get_eta_cell(train_prediction: &TrainPrediction) -> Cell {
    let mut cell = Cell::new(train_prediction.eta_minutes());
    if train_prediction.min == "ARR" || train_prediction.min == "BRD" {
        cell = cell.add_attributes(vec![Attribute::SlowBlink, Attribute::Bold]);
    }
    cell
}

fn get_notes_cell(train_prediction: &TrainPrediction) -> Cell {
    let mut notes: Vec<String> = vec![];
    let mut cell_color = Color::White;

    if train_prediction.is_scheduled {
        notes.push(String::from("Scheduled (Not Live)"));
        cell_color = Color::Grey;
    }
    if train_prediction.is_currently_holding_or_slow {
        notes.push(String::from("Holding"));
        cell_color = Color::Yellow;
    }

    Cell::new(notes.join(", ")).fg(cell_color)
}

fn get_departures_table(departures: Vec<TrainPrediction>) -> Table {
    let mut table = Table::new();
    table.set_header(vec!["Line", "Destination", "ETA", "Notes"]);

    let count_rows_to_render = {
        if departures.len() >= 3 {
            3
        } else {
            departures.len()
        }
    };

    for departure in &departures[..count_rows_to_render] {
        table.add_row(vec![
            Cell::new(departure.line.to_string().as_str())
                .fg(departure.line.get_color())
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Center),
            Cell::new(departure.destination.as_str()),
            get_eta_cell(departure),
            get_notes_cell(departure),
        ]);
    }

    table
}

fn get_warnings_table(departures: StationTags) -> Table {
    let mut table = Table::new();
    table.set_header(vec!["Report", "Count"]);

    for (tag, count) in departures.num_tags_by_type.bad_tags() {
        if count > &0 {
            table.add_row(Row::from(vec![tag, &count.to_string()]));
        }
    }
    table
}

fn print_footer() {
    println!("Source: MetroHero API (https://www.dcmetrohero.com)")
}

pub fn print_plan(trip_info: TripInfo) {
    // Print a summary of the ride.
    let trip_header = format!(
        "{} --> {}",
        trip_info.from_station_name, trip_info.to_station_name
    );
    println!("{}", trip_header.bold());
    println!(
        "Expected ride:    {}m (normally {}m)",
        trip_info.predicted_ride_time as i64, trip_info.expected_ride_time as i64
    );

    let departure_times: Vec<String> = trip_info
        .from_station_train_statuses
        .iter()
        .map(|status| status.eta_minutes())
        .collect();

    println!(
        "Next train:       {}\n",
        departure_times[..cmp::min(departure_times.len(), 4)].join(", ")
    );

    // Print departures summary.
    let departures_table = get_departures_table(trip_info.from_station_train_statuses);
    println!("Departures from {}", trip_info.from_station_name);
    println!("{departures_table}");

    // If alerts, print them.
    if trip_info.metro_alerts.is_some() {
        let alerts = trip_info.metro_alerts.unwrap();
        let mut alert_table = Table::new();
        alert_table.set_header(vec!["Date", "Description"]);

        for alert in alerts {
            alert_table.add_row(vec![alert.date, alert.description]);
        }
        // Print warnings to console.
        println!("{}", "\nWMATA alerts may impact your trip:".bold().red());
        println!("{alert_table}");
        print_footer();
    }
}

pub fn print_departures(
    departure_station: &StationCode,
    departures: Vec<TrainPrediction>,
    station_tags: StationTags,
) {
    let departures_table = get_departures_table(departures);
    println!(
        "Departures for {} ({departure_station})",
        departure_station.to_name()
    );
    println!("{departures_table}");

    if station_tags.num_negative_tags > 0 {
        let warnings = get_warnings_table(station_tags);
        println!("{warnings}");
    }
    print_footer();
}

/// Render a table of Metrorail stations and their WMATA codes.
pub fn print_stations() {
    let mut table = Table::new();
    table.set_header(vec!["Code", "Name"]);
    let stations = StationCode::iter().filter(|station| station != &StationCode::UNKNOWN);

    for station in stations {
        table.add_row(vec![
            Cell::new(format!("{station}")),
            Cell::new(station.to_name().to_string()),
        ]);
    }

    println!("{}", "WMATA Metrorail Stations".bold());
    println!("{table}");
}
