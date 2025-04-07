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
    let file_path = String::from(r"C:\Users\Dheven\Documents\iRacing\telemetry\mx5 mx52016_limerock 2019 chicanes 2025-04-05 18-35-03.ibt");
    println!("Parsing Telemetry File: {}", file_path);

    // Instantiate parser struct
    let mut parser = Parser::new(file_path);

    let vars: Vec<VarInfo> = parser.channels();

    // Instantiate telemetry data
    let telemetry_data: Vec<Tick> = parser.telemetry_data().collect();

    print_all_vars(&vars);

}

pub fn print_all_vars(vars: &Vec<VarInfo>) -> () {
    for var in vars {
        println!("{}", var.name);
    }
}

pub fn find_var(vars: &Vec<VarInfo>, name: String) -> Option<VarInfo> {
    vars.iter().find(|var| var.name == name).cloned()
}


