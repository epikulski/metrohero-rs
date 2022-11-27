//! Schemas describing data returned by the MetroHero API.
use crate::errors::MetroHeroError;
use crossterm::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use strum_macros::{Display, EnumIter, EnumString};

/// Train reports are a map between AIMS ID (as strings) and their report tags.
pub type TrainReports = HashMap<String, TrainTags>;

/// Train predictions for entire Metrorail system, keyed by station ID.
pub type TrainPredictions = HashMap<String, Vec<TrainPrediction>>;

/// Station reports for entire Metrorail system, keyed by station ID.
pub type StationReports = HashMap<String, StationTags>;

lazy_static! {
    static ref STATION_CODE_TO_NAME: HashMap<StationCode, &'static str> = {
        let mut m = HashMap::new();
        m.insert(StationCode::A01, "Metro Center");
        m.insert(StationCode::A02, "Farragut North");
        m.insert(StationCode::A03, "Dupont Circle");
        m.insert(StationCode::A04, "Woodley Park-Zoo/Adams Morgan");
        m.insert(StationCode::A05, "Cleveland Park");
        m.insert(StationCode::A06, "Van Ness-UDC");
        m.insert(StationCode::A07, "Tenleytown-AU");
        m.insert(StationCode::A08, "Friendship Heights");
        m.insert(StationCode::A09, "Bethesda");
        m.insert(StationCode::A10, "Medical Center");
        m.insert(StationCode::A11, "Grosvenor-Strathmore");
        m.insert(StationCode::A12, "White Flint");
        m.insert(StationCode::A13, "Twinbrook");
        m.insert(StationCode::A14, "Rockville");
        m.insert(StationCode::A15, "Shady Grove");
        m.insert(StationCode::B01, "Gallery Pl-Chinatown");
        m.insert(StationCode::B02, "Judiciary Square");
        m.insert(StationCode::B03, "Union Station");
        m.insert(StationCode::B04, "Rhode Island Ave-Brentwood");
        m.insert(StationCode::B05, "Brookland-CUA");
        m.insert(StationCode::B06, "Fort Totten");
        m.insert(StationCode::B07, "Takoma");
        m.insert(StationCode::B08, "Silver Spring");
        m.insert(StationCode::B09, "Forest Glen");
        m.insert(StationCode::B10, "Wheaton");
        m.insert(StationCode::B11, "Glenmont");
        m.insert(StationCode::B35, "NoMa-Gallaudet U");
        m.insert(StationCode::C01, "Metro Center");
        m.insert(StationCode::C02, "McPherson Square");
        m.insert(StationCode::C03, "Farragut West");
        m.insert(StationCode::C04, "Foggy Bottom-GWU");
        m.insert(StationCode::C05, "Rosslyn");
        m.insert(StationCode::C06, "Arlington Cemetery");
        m.insert(StationCode::C07, "Pentagon");
        m.insert(StationCode::C08, "Pentagon City");
        m.insert(StationCode::C09, "Crystal City");
        m.insert(
            StationCode::C10,
            "Ronald Reagan Washington National Airport",
        );
        m.insert(StationCode::C11, "Potomac Yard");
        m.insert(StationCode::C12, "Braddock Road");
        m.insert(StationCode::C13, "King St-Old Town");
        m.insert(StationCode::C14, "Eisenhower Avenue");
        m.insert(StationCode::C15, "Huntington");
        m.insert(StationCode::D01, "Federal Triangle");
        m.insert(StationCode::D02, "Smithsonian");
        m.insert(StationCode::D03, "L'Enfant Plaza");
        m.insert(StationCode::D04, "Federal Center SW");
        m.insert(StationCode::D05, "Capitol South");
        m.insert(StationCode::D06, "Eastern Market");
        m.insert(StationCode::D07, "Potomac Ave");
        m.insert(StationCode::D08, "Stadium-Armory");
        m.insert(StationCode::D09, "Minnesota Ave");
        m.insert(StationCode::D10, "Deanwood");
        m.insert(StationCode::D11, "Cheverly");
        m.insert(StationCode::D12, "Landover");
        m.insert(StationCode::D13, "New Carrollton");
        m.insert(StationCode::E01, "Mt Vernon Sq 7th St-Convention Center");
        m.insert(StationCode::E02, "Shaw-Howard U");
        m.insert(
            StationCode::E03,
            "U Street/African-Amer Civil War Memorial/Cardozo",
        );
        m.insert(StationCode::E04, "Columbia Heights");
        m.insert(StationCode::E05, "Georgia Ave-Petworth");
        m.insert(StationCode::E06, "Fort Totten");
        m.insert(StationCode::E07, "West Hyattsville");
        m.insert(StationCode::E08, "Prince George's Plaza");
        m.insert(StationCode::E09, "College Park-U of Md");
        m.insert(StationCode::E10, "Greenbelt");
        m.insert(StationCode::F01, "Gallery Pl-Chinatown");
        m.insert(StationCode::F02, "Archives-Navy Memorial-Penn Quarter");
        m.insert(StationCode::F03, "L'Enfant Plaza");
        m.insert(StationCode::F04, "Waterfront");
        m.insert(StationCode::F05, "Navy Yard-Ballpark");
        m.insert(StationCode::F06, "Anacostia");
        m.insert(StationCode::F07, "Congress Heights");
        m.insert(StationCode::F08, "Southern Avenue");
        m.insert(StationCode::F09, "Naylor Road");
        m.insert(StationCode::F10, "Suitland");
        m.insert(StationCode::F11, "Branch Ave");
        m.insert(StationCode::G01, "Benning Road");
        m.insert(StationCode::G02, "Capitol Heights");
        m.insert(StationCode::G03, "Addison Road-Seat Pleasant");
        m.insert(StationCode::G04, "Morgan Boulevard");
        m.insert(StationCode::G05, "Largo Town Center");
        m.insert(StationCode::J02, "Van Dorn Street");
        m.insert(StationCode::J03, "Franconia-Springfield");
        m.insert(StationCode::K01, "Court House");
        m.insert(StationCode::K02, "Clarendon");
        m.insert(StationCode::K03, "Virginia Square-GMU");
        m.insert(StationCode::K04, "Ballston-MU");
        m.insert(StationCode::K05, "East Falls Church");
        m.insert(StationCode::K06, "West Falls Church-VT/UVA");
        m.insert(StationCode::K07, "Dunn Loring-Merrifield");
        m.insert(StationCode::K08, "Vienna/Fairfax-GMU");
        m.insert(StationCode::N01, "McLean");
        m.insert(StationCode::N02, "Tysons Corner");
        m.insert(StationCode::N03, "Greensboro");
        m.insert(StationCode::N04, "Spring Hill");
        m.insert(StationCode::N06, "Wiehle-Reston East");
        m.insert(StationCode::N07, "Reston Town Center");
        m.insert(StationCode::N08, "Herndon");
        m.insert(StationCode::N09, "Innovation Center");
        m.insert(StationCode::N10, "Dulles International Airport");
        m.insert(StationCode::N11, "Loudoun Gateway");
        m.insert(StationCode::N12, "Ashburn");
        m.insert(StationCode::UNKNOWN, "UNKNOWN");
        m
    };
}

lazy_static! {
    static ref STATION_NAME_TO_CODE: HashMap<&'static str, StationCode> = {
        let mut m = HashMap::new();
        // TODO: Figure out how to deal with stations with more than one code.
        m.insert("Metro Center", StationCode::A01);
        m.insert("Farragut North", StationCode::A02);
        m.insert("Dupont Circle", StationCode::A03);
        m.insert("Dupont", StationCode::A03);
        m.insert("Woodley Park-Zoo/Adams Morgan", StationCode::A04);
        m.insert("Adams Morgan", StationCode::A04);
        m.insert("Zoo", StationCode::A04);
        m.insert("Woodley Park", StationCode::A04);
        m.insert("Cleveland Park", StationCode::A05);
        m.insert("Van Ness-UDC", StationCode::A06);
        m.insert("Tenleytown-AU", StationCode::A07);
        m.insert("Friendship Heights", StationCode::A08);
        m.insert("Bethesda", StationCode::A09);
        m.insert("Medical Center", StationCode::A10);
        m.insert("Grosvenor-Strathmore", StationCode::A11);
        m.insert("White Flint", StationCode::A12);
        m.insert("Twinbrook", StationCode::A13);
        m.insert("Rockville", StationCode::A14);
        m.insert("Shady Grove", StationCode::A15);
        m.insert("Gallery Pl-Chinatown", StationCode::B01);
        m.insert("Chinatown", StationCode::B01);
        m.insert("Gallery Pl", StationCode::B01);
        m.insert("Judiciary Square", StationCode::B02);
        m.insert("Union Station", StationCode::B03);
        m.insert("Rhode Island Ave-Brentwood", StationCode::B04);
        m.insert("Brookland-CUA", StationCode::B05);
        m.insert("Fort Totten", StationCode::B06);
        m.insert("Takoma", StationCode::B07);
        m.insert("Silver Spring", StationCode::B08);
        m.insert("Forest Glen", StationCode::B09);
        m.insert("Wheaton", StationCode::B10);
        m.insert("Glenmont", StationCode::B11);
        m.insert("NoMa-Gallaudet U", StationCode::B35);
        m.insert("Metro Center", StationCode::C01);
        m.insert("McPherson Square", StationCode::C02);
        m.insert("Farragut West", StationCode::C03);
        m.insert("Foggy Bottom-GWU", StationCode::C04);
        m.insert("GWU", StationCode::C04);
        m.insert("Foggy Bottom", StationCode::C04);
        m.insert("Rosslyn", StationCode::C05);
        m.insert("Arlington Cemetery", StationCode::C06);
        m.insert("Pentagon", StationCode::C07);
        m.insert("Pentagon City", StationCode::C08);
        m.insert("Crystal City", StationCode::C09);
        m.insert(
            "Ronald Reagan Washington National Airport",
            StationCode::C10,
        );
        m.insert("DCA", StationCode::C10);
        m.insert("Potomac Yard", StationCode::C11);
        m.insert("Braddock Road", StationCode::C12);
        m.insert("King St-Old Town", StationCode::C13);
        m.insert("Eisenhower Avenue", StationCode::C14);
        m.insert("Huntington", StationCode::C15);
        m.insert("Federal Triangle", StationCode::D01);
        m.insert("Smithsonian", StationCode::D02);
        m.insert("L'Enfant Plaza", StationCode::D03);
        m.insert("Federal Center SW", StationCode::D04);
        m.insert("Capitol South", StationCode::D05);
        m.insert("Eastern Market", StationCode::D06);
        m.insert("Potomac Ave", StationCode::D07);
        m.insert("Stadium-Armory", StationCode::D08);
        m.insert("Minnesota Ave", StationCode::D09);
        m.insert("Deanwood", StationCode::D10);
        m.insert("Cheverly", StationCode::D11);
        m.insert("Landover", StationCode::D12);
        m.insert("New Carrollton", StationCode::D13);
        m.insert("Mt Vernon Sq 7th St-Convention Center", StationCode::E01);
        m.insert("Shaw-Howard U", StationCode::E02);
         m.insert(
            "U Street/African-Amer Civil War Memorial/Cardozo",
            StationCode::E03,
        );
        m.insert("African-Amer Civil War Memorial",StationCode::E03);
        m.insert("U Street",StationCode::E03);
        m.insert("Cardozo",StationCode::E03);
        m.insert("Columbia Heights", StationCode::E04);
        m.insert("Georgia Ave-Petworth", StationCode::E05);
        m.insert("Georgia Ave", StationCode::E05);
        m.insert("Petworth", StationCode::E05);
        m.insert("Fort Totten", StationCode::E06);
        m.insert("West Hyattsville", StationCode::E07);
        m.insert("Prince George's Plaza", StationCode::E08);
        m.insert("College Park-U of Md", StationCode::E09);
        m.insert("UMD", StationCode::E09);
        m.insert("College Park", StationCode::E09);
        m.insert("Greenbelt", StationCode::E10);
        m.insert("Chinatown", StationCode::F01);
        m.insert("Gallery Pl-Chinatown", StationCode::F01);
        m.insert("Gallery Pl", StationCode::F01);
        m.insert("Archives-Navy Memorial-Penn Quarter", StationCode::F02);
        m.insert("L'Enfant Plaza", StationCode::F03);
        m.insert("Waterfront", StationCode::F04);
        m.insert("Navy Yard-Ballpark", StationCode::F05);
        m.insert("Ballpark", StationCode::F05);
        m.insert("Navy Yard", StationCode::F05);
        m.insert("Anacostia", StationCode::F06);
        m.insert("Congress Heights", StationCode::F07);
        m.insert("Southern Avenue", StationCode::F08);
        m.insert("Naylor Road", StationCode::F09);
        m.insert("Suitland", StationCode::F10);
        m.insert("Branch Ave", StationCode::F11);
        m.insert("Benning Road", StationCode::G01);
        m.insert("Capitol Heights", StationCode::G02);
        m.insert("Addison Road-Seat Pleasant", StationCode::G03);
        m.insert("Morgan Boulevard", StationCode::G04);
        m.insert("Largo Town Center", StationCode::G05);
        m.insert("Van Dorn Street", StationCode::J02);
        m.insert("Franconia-Springfield", StationCode::J03);
        m.insert("Court House", StationCode::K01);
        m.insert("Clarendon", StationCode::K02);
        m.insert("Virginia Square-GMU", StationCode::K03);
        m.insert("GMU", StationCode::K03);
        m.insert("Virginia Square", StationCode::K03);
        m.insert("Ballston-MU", StationCode::K04);
        m.insert("MU", StationCode::K04);
        m.insert("Ballston", StationCode::K04);
        m.insert("East Falls Church", StationCode::K05);
        m.insert("West Falls Church-VT/UVA", StationCode::K06);
        m.insert("Dunn Loring-Merrifield", StationCode::K07);
        m.insert("Vienna/Fairfax-GMU", StationCode::K08);
        m.insert("Vienna", StationCode::K08);
        m.insert("McLean", StationCode::N01);
        m.insert("Tysons Corner", StationCode::N02);
        m.insert("Greensboro", StationCode::N03);
        m.insert("Spring Hill", StationCode::N04);
        m.insert("Wiehle-Reston East", StationCode::N06);
        m.insert("Wiehle", StationCode::N06);
        m.insert("Reston Town Center", StationCode::N07);
        m.insert("Reston", StationCode::N07);
        m.insert("Herndon", StationCode::N08);
        m.insert("Innovation Center", StationCode::N09);
        m.insert("Dulles International Airport", StationCode::N10);
        m.insert("IAD", StationCode::N10);
        m.insert("Loudoun Gateway", StationCode::N11);
        m.insert("Loudoun", StationCode::N11);
        m.insert("Ashburn", StationCode::N12);
        m.insert("UNKNOWN", StationCode::UNKNOWN);
        m
    };
}

/// The RTU code for a Metrorail station.
#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Display, EnumString, EnumIter,
)]
pub enum StationCode {
    A01,
    A02,
    A03,
    A04,
    A05,
    A06,
    A07,
    A08,
    A09,
    A10,
    A11,
    A12,
    A13,
    A14,
    A15,
    B01,
    B02,
    B03,
    B04,
    B05,
    B06,
    B07,
    B08,
    B09,
    B10,
    B11,
    B35,
    C01,
    C02,
    C03,
    C04,
    C05,
    C06,
    C07,
    C08,
    C09,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
    D01,
    D02,
    D03,
    D04,
    D05,
    D06,
    D07,
    D08,
    D09,
    D10,
    D11,
    D12,
    D13,
    E01,
    E02,
    E03,
    E04,
    E05,
    E06,
    E07,
    E08,
    E09,
    E10,
    F01,
    F02,
    F03,
    F04,
    F05,
    F06,
    F07,
    F08,
    F09,
    F10,
    F11,
    G01,
    G02,
    G03,
    G04,
    G05,
    K01,
    K02,
    K03,
    K04,
    K05,
    K06,
    K07,
    K08,
    N01,
    N02,
    N03,
    N04,
    N06,
    N07,
    N08,
    N09,
    N10,
    N11,
    N12,
    J02,
    J03,
    #[serde(other)]
    UNKNOWN,
}

impl StationCode {
    /// Given a station friendly name, look up its RTU code.
    pub fn from_name(name: &str) -> Result<StationCode, MetroHeroError> {
        let result = STATION_NAME_TO_CODE.get(name);
        match result {
            Some(code) => Ok(*code),
            None => Err(MetroHeroError::InvalidStation),
        }
    }

    /// Returns the friendly name of a station.
    pub fn to_name(&self) -> &'static str {
        let name = STATION_CODE_TO_NAME.get(self).unwrap();
        name
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Performance metrics for the overall Metrorail system.
pub struct SystemMetricsResponse {
    pub line_metrics_by_line: SystemMetrics,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Line-specific metrics by line code.
pub struct SystemMetrics {
    #[serde(rename = "RD")]
    pub rd: LineMetrics,
    #[serde(rename = "OR")]
    pub or: LineMetrics,
    #[serde(rename = "SV")]
    pub sv: LineMetrics,
    #[serde(rename = "BL")]
    pub bl: LineMetrics,
    #[serde(rename = "YL")]
    pub yl: LineMetrics,
    #[serde(rename = "GR")]
    pub gr: LineMetrics,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Line metrics for a specific line.
pub struct LineMetrics {
    pub line_code: LineCode,
    pub service_gaps: Vec<ServiceGaps>,
    pub direction_metrics_by_direction: DirectionMetricsByDirection,
    pub date: String,
    pub num_trains: i64,
    pub num_cars: i64,
    pub num_eight_car_trains: i64,
    pub num_delayed_trains: i64,
    pub expected_num_trains: i64,
    pub average_train_delay: Option<i64>,
    pub median_train_delay: Option<i64>,
    pub minimum_train_delay: Option<i64>,
    pub maximum_train_delay: Option<i64>,
    pub average_minimum_headways: Option<f64>,
    pub average_train_frequency: Option<f64>,
    pub expected_train_frequency: Option<f64>,
    pub average_platform_wait_time: Option<f64>,
    pub expected_platform_wait_time: Option<f64>,
    pub train_frequency_status: Option<String>,
    pub platform_wait_time_trend_status: Option<String>,
    pub average_headway_adherence: Option<f64>,
    pub average_schedule_adherence: Option<f64>,
    pub standard_deviation_train_frequency: Option<f64>,
    pub expected_standard_deviation_train_frequency: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Train metrics for a specific Metrorail line.
pub struct DirectionMetricsByDirection {
    #[serde(rename = "1")]
    pub d1: DirectionMetrics,
    #[serde(rename = "2")]
    pub d2: DirectionMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Direction metrics for a specific line and direction.
///
/// This may be null if there are no trains going in this direction.
pub struct DirectionMetrics {
    pub line_code: LineCode,
    pub direction_number: i64,
    pub direction: String,
    pub towards_station_name: String,
    pub date: String,
    pub num_trains: i64,
    pub num_cars: i64,
    pub num_eight_car_trains: i64,
    pub num_delayed_trains: i64,
    pub expected_num_trains: i64,
    pub average_train_delay: Option<i64>,
    pub median_train_delay: Option<i64>,
    pub minimum_train_delay: Option<i64>,
    pub maximum_train_delay: Option<i64>,
    pub average_minimum_headways: Option<f64>,
    pub average_train_frequency: Option<f64>,
    pub expected_train_frequency: Option<f64>,
    pub average_platform_wait_time: Option<f64>,
    pub expected_platform_wait_time: Option<f64>,
    pub train_frequency_status: Option<String>,
    pub platform_wait_time_trend_status: Option<String>,
    pub average_headway_adherence: Option<f64>,
    pub average_schedule_adherence: Option<f64>,
    pub standard_deviation_train_frequency: Option<f64>,
    pub expected_standard_deviation_train_frequency: Option<f64>,
}

/// Real-time and scheduled train predictions for a particular station.
///
/// This API returns all of the same data as WMATA's Real-Time Rail Predictions API, but with
/// additional real-time train predictions (including, optionally, scheduled train predictions),
/// and additional fields for those predictions, like estimated train speed and direction of travel.
/// We calculate our train predictions independently of WMATA by observing train movement over time.
/// Train predictions are returned in ascending order by minutesAway.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Travel information about a trip between two Metrorail stations.
pub struct TripInfo {
    pub from_station_name: String,
    pub from_station_code: StationCode,
    pub to_station_name: String,
    pub to_station_code: StationCode,
    pub trip_station_codes: Vec<StationCode>,
    pub line_codes: Vec<LineCode>,
    pub expected_ride_time: f64,
    pub predicted_ride_time: f64,
    pub time_since_last_train: f64,
    pub from_station_train_statuses: Vec<TrainPrediction>,
    pub date: String,
    pub time_until_next_train: Option<f64>,
    pub metro_alerts: Option<Vec<MetroAlert>>,
    pub metro_alert_keywords: Option<Vec<String>>,
    pub tweets: Option<Vec<Tweet>>,
    pub tweet_keywords: Option<Vec<String>>,
    pub from_station_elevator_outages: Option<Vec<ElevatorEscalatorOutage>>,
    pub from_station_escalator_outages: Option<Vec<ElevatorEscalatorOutage>>,
    pub to_station_elevator_outages: Option<Vec<ElevatorEscalatorOutage>>,
    pub to_station_escalator_outages: Option<Vec<ElevatorEscalatorOutage>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An alert issued by WMATA.
pub struct MetroAlert {
    pub description: String,
    pub station_codes: Vec<StationCode>,
    pub line_codes: Vec<LineCode>,
    pub keywords: Vec<String>,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Tweet referencing a Metrorail station, line, or train.
pub struct Tweet {
    pub twitter_id: i64,
    pub twitter_id_string: String,
    pub user_id: i64,
    pub text: String,
    pub station_codes: Vec<StationCode>,
    pub line_codes: Vec<LineCode>,
    pub keywords: Vec<String>,
    pub url: String,
    pub date: String,
}

/// A truncated tweet returned as a child on a TrainPrediction.
/// Not documented.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbridgedTweet {
    pub twitter_id: i64,
    pub twitter_id_string: String,
    pub user_id: i64,
    pub timestamp: i64,
    pub text: String,
}

/// Recent tweets about a specific train.
/// Not documented.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentTweets {
    pub keywords: String,
    pub tweets: Vec<AbridgedTweet>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An escalator or elevator outage reported by WMATA.
pub struct ElevatorEscalatorOutage {
    pub station_code: StationCode,
    pub station_name: String,
    pub location_description: String,
    pub symptom_description: String,
    pub unit_name: String,
    pub unit_type: String,
    pub out_of_service_date: String,
    pub updated_date: String,
    pub estimated_return_to_service_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Predicted arrival information about Metrorail train.
pub struct TrainPrediction {
    pub train_id: String,
    pub real_train_id: Option<String>,
    #[serde(rename = "Car")]
    pub car: String,
    #[serde(rename = "Destination")]
    pub destination: String,
    #[serde(rename = "DestinationCode")]
    pub destination_code: Option<StationCode>,
    #[serde(rename = "DestinationName")]
    pub destination_name: String,
    #[serde(rename = "Group")]
    pub group: String,
    #[serde(rename = "Line")]
    pub line: LineCode,
    #[serde(rename = "LocationCode")]
    pub location_code: Option<StationCode>,
    #[serde(rename = "LocationName")]
    pub location_name: Option<String>,
    #[serde(rename = "Min")]
    pub min: String,
    pub parent_min: Option<String>,
    pub minutes_away: Option<f64>,
    pub max_minutes_away: Option<f64>,
    pub direction_number: i64,
    pub is_scheduled: bool,
    pub num_positive_tags: i64,
    pub num_negative_tags: i64,
    pub track_number: i64,
    pub current_station_code: StationCode,
    pub current_station_name: String,
    #[serde(rename = "PreviousStationCode")]
    pub previous_station_code: Option<StationCode>,
    pub previous_station_name: Option<String>,
    pub seconds_since_last_moved: i64,
    pub is_currently_holding_or_slow: bool,
    pub seconds_off_schedule: i64,
    pub train_speed: Option<i64>,
    pub is_not_on_revenue_track: bool,
    pub is_keyed_down: bool,
    pub was_keyed_down: bool,
    pub distance_from_next_station: Option<i64>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub direction: Option<i64>,
    pub are_doors_open_on_left: Option<bool>,
    pub are_doors_open_on_right: Option<bool>,
    pub observed_date: String,
    pub recent_tweets: Option<RecentTweets>,
}
impl TrainPrediction {
    /// For a given train ETA value, render a more legible version.
    pub fn eta_minutes(&self) -> String {
        let mut eta_time = self.min.clone();

        let mut needs_min_invalidator = true;
        let non_time_min_values = vec!["ARR", "BRD", "?", ":"];
        for value in non_time_min_values {
            if self.min.as_str().contains(value) {
                needs_min_invalidator = false
            };
        }

        if needs_min_invalidator {
            eta_time.push('m');
        }
        eta_time
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Metrorail line codes.
pub enum LineCode {
    #[serde(rename = "SV")]
    Silver,
    #[serde(rename = "RD")]
    Red,
    #[serde(rename = "OR")]
    Orange,
    #[serde(rename = "BL")]
    Blue,
    #[serde(rename = "YL")]
    Yellow,
    #[serde(rename = "GR")]
    Green,
    #[serde(other)]
    #[serde(rename = "N/A")]
    NonRevenue,
}

impl LineCode {
    /// Get the terminal color for a given LineCode.
    pub fn get_color(&self) -> Color {
        match self {
            LineCode::Silver => Color::Grey,
            LineCode::Green => Color::Green,
            LineCode::Yellow => Color::Yellow,
            LineCode::Blue => Color::Red,
            LineCode::Orange => Color::DarkYellow,
            LineCode::Red => Color::Red,
            LineCode::NonRevenue => Color::DarkMagenta,
        }
    }
}

impl fmt::Display for LineCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LineCode::Silver => write!(f, "SV"),
            LineCode::Red => write!(f, "RD"),
            LineCode::Orange => write!(f, "OR"),
            LineCode::Blue => write!(f, "BL"),
            LineCode::Yellow => write!(f, "YL"),
            LineCode::Green => write!(f, "GR"),
            LineCode::NonRevenue => write!(f, "N/A"),
        }
    }
}
/// User-reported issues with a Metrorail station.
///
/// Note -- the schema here differs from the documentation at <https://dcmetrohero.com/apis#definition-StationTags>
/// but this reflects the actual shape of the data returned by the StationTags API.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// User-reported information about a Metrorail station.
pub struct StationTags {
    pub num_tags_by_type: NumStationTagsByType,
    pub num_positive_tags: i64,
    pub num_negative_tags: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Counts of user-reported issues with a Metrorail station.
pub struct NumStationTagsByType {
    #[serde(rename = "FRIENDLY_OR_HELPFUL_STAFF")]
    pub friendly_or_helpful_staff: i64,
    #[serde(rename = "UNCOMFORTABLE_TEMPS")]
    pub uncomfortable_temps: i64,
    #[serde(rename = "AMPLE_SECURITY")]
    pub ample_security: i64,
    #[serde(rename = "BROKEN_ELEVATOR")]
    pub broken_elevator: i64,
    #[serde(rename = "BROKEN_ESCALATOR")]
    pub broken_escalator: i64,
    #[serde(rename = "CROWDED")]
    pub crowded: i64,
    #[serde(rename = "EMPTY")]
    pub empty: i64,
    #[serde(rename = "FREE_HAND_SANITIZER_AVAILABLE")]
    pub free_hand_sanitizer_available: i64,
    #[serde(rename = "FREE_MASKS_AVAILABLE")]
    pub free_masks_available: i64,
    #[serde(rename = "LONG_WAITING_TIME")]
    pub long_waiting_time: i64,
    #[serde(rename = "NEEDS_WORK")]
    pub needs_work: i64,
    #[serde(rename = "NO_FREE_HAND_SANITIZER")]
    pub no_free_hand_sanitizer: i64,
    #[serde(rename = "NO_FREE_MASKS")]
    pub no_free_masks: i64,
    #[serde(rename = "POSTED_TIMES_INACCURATE")]
    pub posted_times_inaccurate: i64,
    #[serde(rename = "SMOKE_OR_FIRE")]
    pub smoke_or_fire: i64,
    #[serde(rename = "UNFRIENDLY_OR_UNHELPFUL_STAFF")]
    pub unfriendly_or_unhelpful_staff: i64,
}

impl NumStationTagsByType {
    pub fn bad_tags(&self) -> HashMap<&str, &i64> {
        let mut map = HashMap::new();
        map.insert("UNCOMFORTABLE_TEMPS", &self.uncomfortable_temps);
        map.insert("CROWDED", &self.crowded);
        map.insert("LONG_WAITING_TIME", &self.long_waiting_time);
        map.insert("NEEDS_WORK", &self.needs_work);
        map.insert("POSTED_TIMES_INACCURATE", &self.posted_times_inaccurate);
        map.insert("SMOKE_OR_FIRE", &self.smoke_or_fire);
        map.insert(
            "UNFRIENDLY_OR_UNHELPFUL_STAFF",
            &self.unfriendly_or_unhelpful_staff,
        );
        map
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Service disruption information for a given Metrorail line and direction.
pub struct ServiceGaps {
    pub line_code: LineCode,
    pub direction_number: i64,
    pub direction: String,
    pub from_station_code: StationCode,
    pub from_station_name: String,
    pub to_station_code: StationCode,
    pub to_station_name: String,
    pub from_train_id: String,
    pub to_train_id: String,
    pub time_between_trains: f64,
    pub scheduled_time_between_trains: f64,
    pub observed_date: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Counts of user-reported issues with a Metrorail train.
pub struct NumTrainTagsByType {
    #[serde(rename = "BAD_OPERATOR")]
    pub bad_operator: i64,
    #[serde(rename = "ISOLATED_CARS")]
    pub isolated_cars: i64,
    #[serde(rename = "NEW_TRAIN")]
    pub new_train: i64,
    #[serde(rename = "BROKEN_INTERCOM")]
    pub broken_intercom: i64,
    #[serde(rename = "CROWDED")]
    pub crowded: i64,
    #[serde(rename = "DISRUPTIVE_PASSENGER")]
    pub disruptive_passenger: i64,
    #[serde(rename = "EMPTY")]
    pub empty: i64,
    #[serde(rename = "GOOD_OPERATOR")]
    pub good_operator: i64,
    #[serde(rename = "GOOD_RIDE")]
    pub good_ride: i64,
    #[serde(rename = "NEEDS_WORK")]
    pub needs_work: i64,
    #[serde(rename = "RECENTLY_OFFLOADED")]
    pub recently_offloaded: i64,
    #[serde(rename = "UNCOMFORTABLE_RIDE")]
    pub uncomfortable_ride: i64,
    #[serde(rename = "UNCOMFORTABLE_TEMPS")]
    pub uncomfortable_temps: i64,
    #[serde(rename = "WRONG_DESTINATION")]
    pub wrong_destination: i64,
    #[serde(rename = "WRONG_NUM_CARS")]
    pub wrong_num_cars: i64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// User-reported information about a Metrorail train.
pub struct TrainTags {
    pub num_tags_by_type: NumTrainTagsByType,
    pub num_positive_tags: i64,
    pub num_negative_tags: i64,
}

#[cfg(test)]
pub mod tests {
    use crate::schemas::{
        StationReports, StationTags, TrainPrediction, TrainPredictions, TrainReports, TrainTags,
        TripInfo, Tweet,
    };
    use rstest::rstest;
    use std::path::PathBuf;
    use std::{env, fs};

    /// Get test data by filename.
    pub fn get_test_data(filename: String) -> String {
        let cargo_dir = format!(
            "{}/tests/data/{filename}",
            env::var("CARGO_MANIFEST_DIR").unwrap()
        );
        let test_data_path = PathBuf::from(cargo_dir);
        fs::read_to_string(test_data_path).unwrap()
    }

    /// Check that we can deserialize example TripInfo JSON from api.
    /// https://dcmetrohero.com/apis#operation--metrorail-trips--fromStationCode---toStationCode--get
    #[rstest]
    #[case::from_api("tripinfo_api.json")]
    #[case::from_api2("tripinfo_api2.json")] // New SV stations have fewer fields -- test parsing.
    #[case::from_api_na("tripinfo_api_na_trains.json")]
    #[case::from_docs("tripinfo_docs.json")]
    fn test_parse_tripinfo(#[case] filename: String) {
        let data = get_test_data(filename);
        let _test: TripInfo = serde_json::from_str(data.as_str()).unwrap();
    }

    /// Check that we can deserialize example Station Train Predictions from api
    /// https://dcmetrohero.com/apis#operation--metrorail-stations--stationCode--trains-get
    #[rstest]
    #[case::from_docs("station_strain_prediction_docs.json")]
    #[case::from_api("station_train_predictions_api.json")]
    fn test_station_train_predictions(#[case] filename: String) {
        let data = get_test_data(filename);
        let _predictions: Vec<TrainPrediction> = serde_json::from_str(data.as_str()).unwrap();
    }

    /// Check that we can deserialize StationTags from API
    /// https://dcmetrohero.com/apis#operation--metrorail-stations--stationCode--tags-get
    #[rstest]
    #[case::from_api("station_tags_api.json")]
    fn test_station_tags(#[case] filename: String) {
        let data = get_test_data(filename);
        let _tags: StationTags = serde_json::from_str(data.as_str()).unwrap();
    }

    /// Check that we can deserialize tweets from API.
    /// https://dcmetrohero.com/apis#operation--metrorail-tweets-get
    #[rstest]
    #[case::from_api("tweets_api.json")]
    #[case::from_docs("tweets_docs.json")]
    fn test_tweets(#[case] filename: String) {
        let data = get_test_data(filename);
        let _tweets: Vec<Tweet> = serde_json::from_str(data.as_str()).unwrap();
    }

    /// Check that we can deserialize global train positions from API
    /// https://dcmetrohero.com/apis#operation--metrorail-tweets-get
    #[rstest]
    #[case::from_api("train_predictions_api.json")]
    #[case::from_api("train_predictions_api2.json")]
    #[case::from_docs("train_predictions_docs.json")]
    fn test_train_positions(#[case] filename: String) {
        let data = get_test_data(filename);
        let _: Vec<TrainPrediction> = serde_json::from_str(data.as_str()).unwrap();
    }

    /// Check that we can deserialize global train tags from API.
    /// https://dcmetrohero.com/apis#definition-TrainTags
    #[rstest]
    #[case::from_api("global_train_reports_api.json")]
    // #[case::from_docs("train_reports_docs.json")]  # note -- i think there there is a typo in the example from the docs.
    fn test_get_global_train_reports(#[case] filename: String) {
        let data = get_test_data(filename);
        let _train_tags: TrainReports = serde_json::from_str(data.as_str()).unwrap();
    }

    #[rstest]
    #[case::from_docs("train_report_api.json")]
    // #[case::from_api("train_report_docs.json")]  # Note -- I think there is a typo in the example in the docs here. Array should be an object.
    fn test_get_train_report(#[case] filename: String) {
        let data = get_test_data(filename);
        let _train_reports: TrainTags = serde_json::from_str(data.as_str()).unwrap();
    }

    #[test]
    fn test_get_global_train_predictions() {
        let data = get_test_data(String::from("global_train_predictions.json"));
        let _global_train_predictions: TrainPredictions =
            serde_json::from_str(data.as_str()).unwrap();
    }

    #[test]
    fn test_parse_global_station_reports() {
        let data = get_test_data(String::from("global_station_reports.json"));
        let _global_station_reports: StationReports = serde_json::from_str(data.as_str()).unwrap();
    }
}
