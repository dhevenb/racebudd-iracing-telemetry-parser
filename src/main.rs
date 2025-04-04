mod parser;
mod headers;
mod telemetry_data;
mod utils;

use std::fs::File;

use headers::{FileInfo, SessionInfo, VarInfo};
use parser::Parser;
use telemetry_data::{Tick};

use itelem::IbtReader;

fn main() {
    println!("Starting RaceBuddTelemetryAgent execution...");

    // iRacing ibt file path
    let file_path = String::from(r"C:\Users\Dheven\Documents\iRacing\telemetry\toyotagr86_summit summit raceway 2025-03-31 18-44-45.ibt");
    println!("Parsing Telemetry File: {}", file_path);

    // Instantiate parser struct
    //let mut parser = Parser::new(file_path);

    // Instantiate file, session, and var info
    //let file_info: FileInfo = parser.file_info();
    // let session_info: SessionInfo = parser.session_info();
    //let var_info: VarInfo = parser.var_info();

    // Instantiate telemetry data
    //let telemetry_data: Vec<Tick> = parser.telemetry_data().collect();



    // TEST
    let file = File::open(r"C:\Users\Dheven\Documents\iRacing\telemetry\mx5 mx52016_summit summit raceway 2025-03-28 20-55-18.ibt").unwrap();
    let mut reader = IbtReader::new(Box::new(file));
    assert_eq!(reader.header.tick_rate, 60);

    let weekend_info = &reader.session_info.weekend_info;
    assert_eq!(weekend_info.track_name, "spielberg gp");

    let rpm = reader.find_var("RPM".to_string()).unwrap();
    let samples: Vec<_> = reader.samples().collect();

    // There are 3371 samples and with a 60 tick tick_rate
    // meaning that the telemetry file contains 56 seconds of data
    assert_eq!(samples.len(), 3371);
    let first_sample = samples[1001].get_by_header(&rpm).unwrap();
    println!("{:?}", first_sample);

}


