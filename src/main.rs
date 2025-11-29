use std::time::Instant;
use log::{Level, info};

use racebudd_iracing_telemetry_parser::RaceBuddTelem;

pub fn main() {
    // Instantiate logging and set env_logger to terminal screen
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();

    info!("Starting telemetry parser");

    let file_name: &str = r"test_ibt_file.ibt";
    info!("Attempting to parse file {}", &file_name);

    let telem = RaceBuddTelem::new(file_name);

}