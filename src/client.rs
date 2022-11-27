#![warn(missing_docs)]
//! A client for requesting data from the MetroHero API.
//!
//! API Documentation: <https://dcmetrohero.com/apis>
#[cfg(not(test))]
use std::env;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, ACCEPT};
use reqwest::tls;
use serde::de::DeserializeOwned;

use crate::errors::MetroHeroError;
use crate::schemas::{
    StationCode, StationReports, StationTags, SystemMetricsResponse, TrainPrediction,
    TrainPredictions, TrainReports, TrainTags, TripInfo, Tweet,
};

/// A client for requesting data from the MetroHero API.
///
/// # API Keys
/// When instantiated using [`MetroHeroClient::default()`], the client will attempt to fetch an API
/// key from the environment at `METROHERO_API_KEY`. To set a key explicitely, use [`MetroHeroClient::new()`]
///
/// # Example
///
/// ```
/// use metrohero_rs::MetroHeroClient;
/// let client = MetroHeroClient::default(); // Or set an explicit API key with `MetroHeroClient::new()`
/// let system_metrics = client.get_system_metrics().unwrap();
/// ```
pub struct MetroHeroClient {
    http_client: Client,
    api_url_base: String,
}

impl MetroHeroClient {
    /// Instantiate a new client for interacting with the MetroHero API.
    pub fn new(api_key: String) -> Self {
        #[cfg(not(test))]
        let api_url_base: &String = &String::from("https://dcmetrohero.com/api/v1");

        #[cfg(test)]
        let api_url_base: &String = &mockito::server_url();

        let mut headers = HeaderMap::new();
        headers.insert("apiKey", api_key.parse().unwrap());
        headers.insert(ACCEPT, "application/json".parse().unwrap());

        #[cfg(not(test))]
        let require_tls = true;

        #[cfg(test)]
        let require_tls = false;

        #[cfg(feature = "rustls")]
        let client = Client::builder()
            .default_headers(headers)
            .use_rustls_tls()
            .https_only(require_tls)
            .min_tls_version(tls::Version::TLS_1_2)
            .build()
            .unwrap();

        #[cfg(not(feature = "rustls"))]
        let client = Client::builder()
            .default_headers(headers)
            .https_only(require_tls)
            .min_tls_version(tls::Version::TLS_1_2)
            .build()
            .unwrap();

        Self {
            http_client: client,
            api_url_base: api_url_base.clone(),
        }
    }

    /// Send a request to the MetroHero API.
    fn send_request<T: DeserializeOwned>(&self, request_path: String) -> Result<T, MetroHeroError> {
        let request_url = format!("{}{}", self.api_url_base, request_path);
        let response = self.http_client.get(request_url).send()?;
        match response.status().as_u16() {
            200 => {
                let model: T = response.json().map_err(|_| MetroHeroError::ParseError)?;
                Ok(model)
            }
            400 => Err(MetroHeroError::InvalidRequest),
            401 => Err(MetroHeroError::AuthenticationError),
            503 => Err(MetroHeroError::RateLimited),
            _ => panic!("An unspecified status was received from th the server."),
        }
    }

    /// Gets real-time system-wide metrics, broken down by line and direction of travel.
    ///
    /// # Notes
    /// This includes everything from the number of trains and train cars to calculations like
    /// average minimum headways, train frequencies, platform wait times, and more. Data is updated
    /// about every 30 seconds.
    ///
    /// See: <https://dcmetrohero.com/apis#operation--metrorail-metrics-get>
    pub fn get_system_metrics(&self) -> Result<SystemMetricsResponse, MetroHeroError> {
        let api_path = String::from("/metrorail/metrics");
        let metrics: SystemMetricsResponse = self.send_request(api_path)?;
        Ok(metrics)
    }

    /// Gets real-time trip information given current conditions.
    ///
    /// # Notes
    ///
    /// The algorithms behind this API take, when available, both current conditions and conditions
    /// in the recent past into account--including any train delays and congestion--to make
    /// predictions about how long riders may be waiting or have been waiting for the next train to
    /// service the specified trip, as well as how long the trip might take once they're aboard.

    /// Trips with station transfers are not directly supported. For example, to get trip
    /// information from Glenmont to Vienna, split the trip up into segments (e.g. Glenmont to
    /// Metro Center, then Metro Center to Vienna) and perform a separate request to this API for
    /// each segment. You can then aggregate the results across the responses of each API request
    /// however you see fit to make your own derived predictions about the trip as a whole.
    ///
    /// See: <https://dcmetrohero.com/apis#operation--metrorail-trips--fromStationCode---toStationCode--get>
    pub fn get_trip_info(
        &self,
        from_station_code: &StationCode,
        to_station_code: &StationCode,
    ) -> Result<TripInfo, MetroHeroError> {
        let api_path = format!("/metrorail/trips/{}/{}", from_station_code, to_station_code);
        let result = self.send_request(api_path);
        match result {
            Ok(trip_info) => Ok(trip_info),
            // Invalid request here means one or more station codes were invalid.
            Err(MetroHeroError::InvalidRequest) => Err(MetroHeroError::InvalidItinerary),
            Err(e) => Err(e),
        }
    }
    /// Gets the last 30 minutes' worth of Metrorail-related tweets from Twitter.
    ///
    /// # Notes
    /// These tweets may be describing a problem with a particular station or train, a general
    /// problem on a given line, or nothing meaningful at all; while we do our best using various
    /// heuristics to only include relevant tweets, we make no guarantees.
    ///
    /// See: <https://dcmetrohero.com/apis#operation--metrorail-tweets-get>
    pub fn get_tweets(&self) -> Result<Vec<Tweet>, MetroHeroError> {
        let api_path = String::from("/metrorail/tweets");
        let tweets: Vec<Tweet> = self.send_request(api_path)?;
        Ok(tweets)
    }

    /// Gets real-time train predictions for the entire Metrorail system.
    ///
    /// # Notes
    ///
    /// These predictions are unique per train, i.e. exactly one prediction is returned per train.
    /// This API is intended to be used as an alternative to WMATA's Train Positions API, but does
    /// not return the exact same data, nor is it in the same format. We calculate our train
    /// predictions independently of WMATA by observing train movement over time.
    ///
    /// Train predictions are returned in no particular order.
    ///
    /// See: <https://dcmetrohero.com/apis#operation--metrorail-trains-get>
    pub fn get_train_positions(&self) -> Result<Vec<TrainPrediction>, MetroHeroError> {
        let api_path = String::from("/metrorail/trains");
        let train_positions: Vec<TrainPrediction> = self.send_request(api_path)?;
        Ok(train_positions)
    }

    /// Gets real-time rider reports, referred to as tags, for all trains.
    ///
    /// # Notes
    /// All tags are of predefined types (e.g. 'New Train', 'Crowded', 'Smooth Ride', etc) submitted
    /// by MetroHero users. These tags expire anywhere from 15 to 60 minutes after they've been
    /// created, depending on the type of tag; only current, unexpired tags are returned by this
    /// API.
    ///
    /// Train tags are ordered by tag type in descending order by current number of active tags.
    ///
    /// See: <https://dcmetrohero.com/apis#operation--metrorail-trains-tags-get>
    pub fn get_train_reports(&self) -> Result<TrainReports, MetroHeroError> {
        let api_path = String::from("/metrorail/trains/tags");
        let reports: TrainReports = self.send_request(api_path)?;
        Ok(reports)
    }

    /// Gets real-time rider reports about a particular train, referred to as tags.
    ///
    /// # Notes
    ///
    /// All tags are of predefined types (e.g. 'New Train', 'Crowded', 'Smooth Ride', etc) submitted
    /// by MetroHero users. These tags expire anywhere from 15 to 60 minutes after they've been
    /// created, depending on the type of tag; only current, unexpired tags are returned by this
    /// API.
    ///
    /// Train tags are ordered by tag type in descending order by current number of active tags.
    /// See: <https://dcmetrohero.com/apis#operation--metrorail-trains--trainId--tags-get>
    pub fn get_train_report(&self, train_id: String) -> Result<TrainTags, MetroHeroError> {
        let api_path = format!("/metrorail/trains/{train_id}/tags");
        let result: Result<TrainTags, MetroHeroError> = self.send_request(api_path);
        match result {
            Ok(train_tags) => Ok(train_tags),
            // Invalid request here can only mean the train ID was invalid.
            Err(MetroHeroError::InvalidRequest) => Err(MetroHeroError::InvalidTrainId),
            Err(e) => Err(e),
        }
    }

    /// Gets real-time and scheduled train predictions for all stations.
    ///
    /// # Notes
    ///
    /// For each station, this API returns all of the same data as WMATA's Real-Time Rail
    /// Predictions API, but with additional real-time train predictions (including, optionally,
    /// scheduled train predictions), and additional fields for those predictions, like estimated
    /// train speed and direction of travel. We calculate our train predictions independently of
    /// WMATA by observing train movement over time.
    ///
    /// Disclaimer: None of the above should be interpreted as a claim that our data is more
    /// accurate, more complete, or more timely than WMATA's data, we are simply stating some of the
    /// factual differences between the two datasets; as with all our APIs, we make no claims as to
    /// the accuracy of the data returned.
    ///
    /// Each set of train predictions for each station is in ascending order by minutesAway.
    /// When the request parameter includeScheduledPredictions is set to false, our data can be used
    /// as a drop-in substitute for WMATA's Real-Time Rail Predictions API.
    ///
    /// See: <https://dcmetrohero.com/apis#operation--metrorail-stations-trains-get>
    pub fn get_train_predictions(&self) -> Result<TrainPredictions, MetroHeroError> {
        let api_path = "/metrorail/stations/trains".to_string();
        let predictions: TrainPredictions = self.send_request(api_path)?;
        Ok(predictions)
    }

    /// Gets real-time and scheduled train predictions for a particular station.
    ///
    /// # Notes
    ///
    /// This API returns all of the same data as WMATA's Real-Time Rail Predictions API, but with
    /// additional real-time train predictions (including, optionally, scheduled train predictions),
    /// and additional fields for those predictions, like estimated train speed and direction of
    /// travel. We calculate our train predictions independently of WMATA by observing train
    /// movement over time.
    ///
    /// Disclaimer: None of the above should be interpreted as a claim that our data is more
    /// accurate, more complete, or more timely than WMATA's data, we are simply stating some of the
    /// factual differences between the two datasets; as with all our APIs, we make no claims as to
    /// the accuracy of the data returned.
    ///
    /// Train predictions are returned in ascending order by minutesAway. When the request parameter
    /// includeScheduledPredictions is set to false, our data can be used as a drop-in substitute
    /// for WMATA's Real-Time Rail Predictions API.
    ///
    /// See: <https://dcmetrohero.com/apis#operation--metrorail-stations--stationCode--trains-get>
    pub fn get_station_train_predictions(
        &self,
        station_code: &StationCode,
    ) -> Result<Vec<TrainPrediction>, MetroHeroError> {
        let api_path = format!(
            "/metrorail/stations/{}/trains?includeScheduledPredictions=true",
            station_code
        );

        let result: Result<Vec<TrainPrediction>, MetroHeroError> = self.send_request(api_path);
        match result {
            Ok(train_predictions) => Ok(train_predictions),
            // 400 Errors here indicate that the station ID was invalid.
            Err(MetroHeroError::InvalidRequest) => Err(MetroHeroError::InvalidStation),
            Err(e) => Err(e),
        }
    }

    /// Gets real-time rider reports, referred to as tags, for all stations.
    ///
    /// # Notes
    ///
    /// All tags are of predefined types (e.g. 'Friendly or Helpful Staff', 'Broken Escalator', etc)
    /// and are either explicitly submitted by MetroHero users, or implicitly derived from public
    /// WMATA-related tweets on Twitter by our algorithms. These tags expire anywhere from 15 to 180
    /// minutes after they've been created, depending on the type of tag; only current, unexpired
    /// tags are returned by this API.
    ///
    /// Station tags are ordered by tag type in descending order by current number of active tags.
    /// See: <https://dcmetrohero.com/apis#operation--metrorail-stations-tags-get>
    pub fn get_station_reports(&self) -> Result<StationReports, MetroHeroError> {
        let api_path = "/metrorail/stations/tags".to_string();
        let reports: StationReports = self.send_request(api_path)?;
        Ok(reports)
    }

    /// Gets real-time rider reports about a particular station, referred to as tags.
    /// All tags are of predefined types (e.g. 'Friendly or Helpful Staff', 'Broken Escalator', etc)
    /// and are either explicitly submitted by MetroHero users, or implicitly derived from public
    /// WMATA-related tweets on Twitter by our algorithms. These tags expire anywhere from 15 to 180
    /// minutes after they've been created, depending on the type of tag; only current, unexpired
    /// tags are returned by this API.
    ///
    /// Station tags are ordered by tag type in descending order by current number of active tags.
    /// See: <https://dcmetrohero.com/apis#operation--metrorail-stations--stationCode--tags-get>
    pub fn get_station_report(
        &self,
        station_code: &StationCode,
    ) -> Result<StationTags, MetroHeroError> {
        let api_path = format!("/metrorail/stations/{}/tags", station_code);
        let result: Result<StationTags, MetroHeroError> = self.send_request(api_path);
        match result {
            Ok(station_tags) => Ok(station_tags),
            // If request was invalid, only explanation is that the station code was invalid.
            Err(MetroHeroError::InvalidRequest) => Err(MetroHeroError::InvalidStation),
            Err(e) => Err(e),
        }
    }
}

impl Default for MetroHeroClient {
    /// Instantiate a MetroHeroClient using an API key from the `METROHERO_API_KEY` environment variable.
    fn default() -> Self {
        let api_key_env_var_name = "METROHERO_API_KEY";

        #[cfg(not(test))]
        let api_key = env::var(api_key_env_var_name)
            .unwrap_or_else(|_| panic!("Environment variable {api_key_env_var_name} is missing"));

        #[cfg(test)]
        let api_key = String::from(api_key_env_var_name);

        MetroHeroClient::new(api_key)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::MetroHeroClient;
    use crate::errors::MetroHeroError;
    use crate::schemas::tests::get_test_data;
    use crate::schemas::StationCode;
    use mockito::mock;

    #[test]
    fn test_get_station_train_predictions_invalid() {
        let mock = mock(
            "GET",
            "/metrorail/stations/UNKNOWN/trains?includeScheduledPredictions=true",
        )
        .with_status(400)
        .create();

        let client = MetroHeroClient::default();
        let err = client
            .get_station_train_predictions(&StationCode::UNKNOWN)
            .unwrap_err();
        assert_eq!(err, MetroHeroError::InvalidStation);
        mock.assert();
    }

    #[test]
    fn test_get_tripinfo() {
        let mock = mock("GET", "/metrorail/trips/K01/K02")
            .with_status(200)
            .with_body(get_test_data(String::from("tripinfo_api.json")))
            .create();
        let client = MetroHeroClient::default();
        client
            .get_trip_info(&StationCode::K01, &StationCode::K02)
            .unwrap();
        mock.assert()
    }

    #[test]
    fn test_get_tripinfo_invalid() {
        let mock = mock("GET", "/metrorail/trips/UNKNOWN/UNKNOWN")
            .with_status(400)
            .create();

        let client = MetroHeroClient::default();
        let error = client
            .get_trip_info(&StationCode::UNKNOWN, &StationCode::UNKNOWN)
            .unwrap_err();
        assert_eq!(error, MetroHeroError::InvalidItinerary);
        mock.assert()
    }

    #[test]
    fn test_get_system_metrics() {
        let mock = mock("GET", "/metrorail/metrics")
            .with_status(200)
            .with_body(get_test_data(String::from("system_metrics_api.json")))
            .create();

        let client = MetroHeroClient::default();
        client.get_system_metrics().unwrap();
        mock.assert();
    }

    #[test]
    fn test_get_tweets() {
        let mock = mock("GET", "/metrorail/tweets")
            .with_status(200)
            .with_body(get_test_data(String::from("tweets_api.json")))
            .create();
        let client = MetroHeroClient::default();
        client.get_tweets().unwrap();
        mock.assert();
    }

    #[test]
    fn test_get_train_positions() {
        let mock = mock("GET", "/metrorail/trains")
            .with_status(200)
            .with_body(get_test_data(String::from("train_predictions_api.json")))
            .create();
        let client = MetroHeroClient::default();
        client.get_train_positions().unwrap();
        mock.assert();
    }

    #[test]
    fn test_get_train_report_invalid() {
        let mock = mock("GET", "/metrorail/trains/123/tags")
            .with_status(400)
            .create();
        let client = MetroHeroClient::default();
        let error = client.get_train_report(String::from("123")).unwrap_err();
        assert_eq!(error, MetroHeroError::InvalidTrainId);
        mock.assert();
    }

    #[test]
    fn test_get_train_predictions() {
        let mock = mock("GET", "/metrorail/stations/trains")
            .with_status(200)
            .with_body(get_test_data(String::from("global_train_predictions.json")))
            .create();
        let client = MetroHeroClient::default();
        client.get_train_predictions().unwrap();
        mock.assert();
    }

    #[test]
    fn test_get_station_train_predictions() {
        let mock = mock(
            "GET",
            "/metrorail/stations/K01/trains?includeScheduledPredictions=true",
        )
        .with_status(200)
        .with_body(get_test_data(String::from(
            "station_strain_prediction_docs.json",
        )))
        .create();

        let client = MetroHeroClient::default();
        client
            .get_station_train_predictions(&StationCode::K01)
            .unwrap();
        mock.assert()
    }

    #[test]
    fn test_get_station_reports() {
        let mock = mock("GET", "/metrorail/stations/tags")
            .with_status(200)
            .with_body(get_test_data(String::from("global_station_reports.json")))
            .create();
        let client = MetroHeroClient::default();
        client.get_station_reports().unwrap();
        mock.assert();
    }

    #[test]
    fn test_get_station_report() {
        let mock = mock("GET", "/metrorail/stations/K05/tags")
            .with_status(200)
            .with_body(get_test_data("station_tags_api.json".to_string()))
            .create();
        let client = MetroHeroClient::default();
        client.get_station_report(&StationCode::K05).unwrap();
        mock.assert();
    }

    #[test]
    fn test_get_station_report_invalid() {
        let mock = mock("GET", "/metrorail/stations/UNKNOWN/tags")
            .with_status(400)
            .create();
        let client = MetroHeroClient::default();
        let err = client
            .get_station_report(&StationCode::UNKNOWN)
            .unwrap_err();
        assert_eq!(err, MetroHeroError::InvalidStation);
        mock.assert();
    }
}
