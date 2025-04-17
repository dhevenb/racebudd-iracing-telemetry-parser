mod parser;
mod headers;
mod telemetry_data;
mod utils;

use std::{collections::HashMap, fs::File, time::Instant};

use headers::{FileInfo, SessionInfo, VarInfo};
use parser::Parser;
use telemetry_data::{Tick, Ticks};

use itelem::IbtReader;

fn main() {
    println!("Starting RaceBuddTelemetryAgent execution...");

    // iRacing ibt file path
    let file_path = String::from(r"C:\Users\Dheven\Documents\iRacing\telemetry\mx5 mx52016_limerock 2019 chicanes 2025-04-05 21-25-06.ibt");
    println!("Parsing Telemetry File: {}", file_path);

    // Instantiate parser struct
    let mut parser = Parser::new(file_path);

    let vars: Vec<VarInfo> = parser.channels();
    let lap_var: VarInfo = find_var(&vars, "Lap".to_string());

    // Sequential Search
    {
        let sequential_search = Instant::now();

        let mut telemetry_data: Ticks<'_> = parser.telemetry_data();

        let lap_indices: HashMap<i32, i32> = get_lap_indices_with_sequential_search(telemetry_data, &lap_var);

        println!("Sequential Search Method Duration: {:?}", sequential_search.elapsed());
        // Sequential Search Method Duration: Instant { t: 12907.3846103s }
        println!("{:?}", lap_indices);
    }

}

pub fn get_lap_indices_with_sequential_search(mut telemetry_data: Ticks, lap_var: &VarInfo) -> HashMap<i32, i32> {

    let mut lap_indices: HashMap<i32, i32> = HashMap::new();

    let mut target_lap: i32 = 1;
    let mut current_index: i32 = 0;

    while let Some(tick) = telemetry_data.next() {

        if tick.get_value(lap_var).unwrap().int() == target_lap {
            lap_indices.insert(target_lap, current_index);
            target_lap += 1;
        }

        current_index +=1; 
    }
    
    lap_indices
}

pub fn print_all_vars(vars: &Vec<VarInfo>) -> () {
    for var in vars {
        println!("{}", var.name);
    }
}

pub fn find_var(vars: &Vec<VarInfo>, name: String) -> VarInfo {
    vars.iter().find(|var| var.name == name).cloned().unwrap()
}


