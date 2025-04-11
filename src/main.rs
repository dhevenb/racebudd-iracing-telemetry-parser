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

    // Benchmark Binary search
    {
        let binary_start = Instant::now();

        // Instantiate telemetry data and load it all into memory. 
        let telemetry_data: Vec<Tick> = parser.telemetry_data().collect();    
    
        // Find index for the beginning of each lap
        let lap_indices: HashMap<i32, i32> = get_lap_indices_with_binary_search(&telemetry_data, &lap_var);
    
        println!("Binary Search Method Duration: {:?}", binary_start);
        // Binary Search Method Duration: Instant { t: 12907.3078388s }
        println!("{:?}", lap_indices);
    }

    // Sequential Search
    {
        let sequential_search = Instant::now();

        // Instantiate telemetry data
        let mut telemetry_data: Ticks<'_> = parser.telemetry_data();

        let lap_indices: HashMap<i32, i32> = get_lap_indices_with_sequential_search(telemetry_data, &lap_var);

        println!("Sequential Search Method Duration: {:?}", sequential_search);
        // Sequential Search Method Duration: Instant { t: 12907.3846103s }
        println!("{:?}", lap_indices);
    }

}

pub fn get_lap_indices_with_sequential_search(mut telemetry_data: Ticks, lap_var: &VarInfo) -> HashMap<i32, i32> {

    let mut lap_indices: HashMap<i32, i32> = HashMap::new();

    // Always start looking for lap 1
    // When first loading into game lap is set to 0
    let mut target_lap: i32 = 1;
    let mut current_index: i32 = 0;

    while let Some(tick) = telemetry_data.next() {

        if tick.get_value(lap_var).unwrap().int() == target_lap {
            println!("Found first occurence for lap {} at index {}", target_lap, current_index);
            println!("With a lap value of  {:?}", tick.get_value(lap_var).unwrap().int());
            lap_indices.insert(target_lap, current_index);
            target_lap += 1;
        }

        current_index +=1; 
    }
    
    lap_indices
}

pub fn get_lap_indices_with_binary_search(telemetry_data: &Vec<Tick>, lap_var: &VarInfo) -> HashMap<i32, i32> {
    // Get total laps by checking last index
    let total_laps = telemetry_data[telemetry_data.len() - 1].get_value(lap_var).unwrap().int();

    let mut lap_indices: HashMap<i32, i32> = HashMap::with_capacity(total_laps as usize);

    // For each lap perform binary search the sequentially reverse search for first occurence
    for i in 1..=total_laps {
        // Binary Search
        let mut left: i32 = 0;
        let mut right: i32 = telemetry_data.len() as i32 + 1;

        while left <= right {
            // stop integer overflow
            let mid: i32 = left + (right - left) / 2;
    
            if telemetry_data[mid as usize].get_value(lap_var).unwrap().int() == i {
                // found lap number break to find first occurence
                let mut first_index: i32 = mid;
                
                while first_index > 0 && telemetry_data[first_index as usize].get_value(lap_var).unwrap().int() == i { 
                    if telemetry_data[first_index as usize - 1].get_value(lap_var).unwrap().int() < i {
                        // If we find the first occurence of lap update hash
                        println!("Found first occurence for lap {} at index {}", i, first_index);
                        println!("With a lap value of  {:?}", telemetry_data[first_index as usize].get_value(lap_var).unwrap().int());
                        lap_indices.insert(i, first_index);
                        break
                    } else {
                        // Otherwise move index back one
                        first_index -= 1;
                    }   
                }
            break               

            } else if telemetry_data[mid as usize].get_value(lap_var).unwrap().int() >= i {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
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


