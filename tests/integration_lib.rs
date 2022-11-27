use metrohero_rs::client::MetroHeroClient;
use metrohero_rs::schemas::{
    StationCode, StationReports, StationTags, SystemMetricsResponse, TrainPrediction,
    TrainPredictions, TrainReports, TrainTags, Tweet,
};

/// No CLI access to this library function, but we test it anyway.
#[test]
#[ignore]
fn get_system_info_test() {
    let client = MetroHeroClient::default();
    // Validate that we can fetch system metrics and that they can are parsed.
    let _: SystemMetricsResponse = client.get_system_metrics().unwrap();
}

#[test]
#[ignore]
fn get_tweets_test() {
    let client = MetroHeroClient::default();
    // Validate that we can fetch tweets and that they are parsed.
    let _: Vec<Tweet> = client.get_tweets().unwrap();
}

#[test]
#[ignore]
fn get_train_positions_test() {
    let client = MetroHeroClient::default();
    let _: Vec<TrainPrediction> = client.get_train_positions().unwrap();
}

#[test]
#[ignore]
fn get_train_reports_test() {
    let client = MetroHeroClient::default();
    let _: TrainReports = client.get_train_reports().unwrap();
}

#[test]
#[ignore]
fn get_train_report_test() {
    let client = MetroHeroClient::default();

    // Figure out a valid train ID to query.
    let global_train_positions = client.get_train_positions().unwrap();
    let train_id = &global_train_positions.get(0).unwrap().train_id;

    // Look up reports for that ID.
    let _: TrainTags = client.get_train_report(String::from(train_id)).unwrap();
}

#[test]
#[ignore]
fn get_global_train_predictions_test() {
    let client = MetroHeroClient::default();
    let _: TrainPredictions = client.get_train_predictions().unwrap();
}

#[test]
#[ignore]
fn get_train_predictions_test() {
    let client = MetroHeroClient::default();
    let _: TrainPredictions = client.get_train_predictions().unwrap();
}

#[test]
#[ignore]
fn get_station_train_predictions_test() {
    let client = MetroHeroClient::default();
    let _: Vec<TrainPrediction> = client
        .get_station_train_predictions(&StationCode::K05)
        .unwrap();
}

#[test]
#[ignore]
fn get_station_reports_test() {
    let client = MetroHeroClient::default();
    let _: StationReports = client.get_station_reports().unwrap();
}

#[test]
#[ignore]
fn get_station_report_test() {
    let client = MetroHeroClient::default();
    let _: StationTags = client.get_station_report(&StationCode::K05).unwrap();
}
