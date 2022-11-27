use assert_cmd::Command;

fn get_cmd() -> Command {
    return Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
}

#[test]
fn get_trip_info_by_code() {
    let mut cmd = get_cmd();
    let output = cmd.args(vec!["plan", "K03", "C02"]).output().unwrap();
    let output_text = String::from_utf8(output.stdout).unwrap();
    let std_err = String::from_utf8(output.stderr).unwrap();
    println!("{}", std_err);
    assert_eq!(
        output_text.contains("Virginia Square-GMU --> McPherson Square"),
        true
    );
}

#[test]
fn get_trip_info_by_name() {
    let mut cmd = get_cmd();
    let output = cmd
        .args(vec!["plan", "GMU", "McPherson Square"])
        .output()
        .unwrap();
    let output_text = String::from_utf8(output.stdout).unwrap();
    let std_err = String::from_utf8(output.stderr).unwrap();
    println!("{}", std_err);
    assert_eq!(
        output_text.contains("Virginia Square-GMU --> McPherson Square"),
        true
    );
}

#[test]
fn get_departures_by_code() {
    let mut cmd = get_cmd();
    let output = cmd.args(vec!["departures", "K03"]).output().unwrap();
    let output_text = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        output_text.contains("Departures for Virginia Square-GMU (K03)"),
        true
    );
}

#[test]
fn get_departures_by_name() {
    let mut cmd = get_cmd();
    let output = cmd.args(vec!["departures", "GMU"]).output().unwrap();
    let output_text = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        output_text.contains("Departures for Virginia Square-GMU (K03)"),
        true
    );
}

#[test]
fn print_stations() {
    let mut cmd = get_cmd();
    let output = cmd.args(vec!["stations"]).output().unwrap();
    let output_text = String::from_utf8(output.stdout).unwrap();
    assert_eq!(output_text.contains("Rosslyn"), true)
}
