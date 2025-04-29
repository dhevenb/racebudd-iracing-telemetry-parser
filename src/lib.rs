mod parser;
mod headers;
mod telemetry_data;
mod utils;

use std::{collections::HashMap};

use headers::{FileInfo, SessionInfo, VarInfo};
use parser::Parser;
use telemetry_data::{Tick, Ticks};

use itelem::IbtReader;

pub struct RaceBuddTelem {
    parser: Parser,
}

impl<'a> RaceBuddTelem {

    pub fn new(file_name: &str) -> Self {
        // Instantiate parser struct
        let parser = Parser::new(file_name);
        RaceBuddTelem { parser }
    }

    // Returns all available variables for this sessions
    pub fn available_vars(&mut self) -> Vec<VarInfo> {
        self.parser.channels()
    }

    // Return Telemetry Data Iterator
    pub fn telemetry_data(&mut self) -> Ticks<'_> {
        self.parser.telemetry_data()
    }

    // Returns a HashMap of all lap indices
    pub fn lap_indices(&mut self) -> HashMap<i32, i32> {
        let lap_var = self.available_vars().iter().find(|var| var.name == "Lap").cloned().unwrap();
        get_lap_indices_with_sequential_search(self.telemetry_data(), &lap_var)
    }

    // Returns all lap data for the given laps
    pub fn get_lap_data(&mut self, laps: Vec<i32>) -> HashMap<i32, Vec<Tick>> {
        // set lap_data to expected size of laps
        let mut lap_data: HashMap<i32, Vec<Tick>> = HashMap::new();
        let lap_indices = self.lap_indices();
        let mut telemetry_data = self.telemetry_data();

        // Iterate through each target lap
        for lap in laps {
            let mut data: Vec<Tick> = Vec::new();
            let lap_first_index = lap_indices[&lap];
            let lap_last_index = lap_indices[&(lap + 1)] - 1;

            // Jump straight to target lap first index
            if let Some(tick) = telemetry_data.nth(lap_first_index as usize) {
                for _ in lap_first_index..=lap_last_index {
                    if let Some(tick) = telemetry_data.next() {
                        data.push(tick);
                    }
                }
            }

            lap_data.insert(lap, data);
        }
        lap_data
    }
}

// Returns indices of each laps first Tick
// Uses Sequential search so is O(n) Time Complexity
// Benchmarked against Binary Search and was an average of 10ms faster on my hardware
// Additionally, binary search required loading the entire telemetry file into memory as oppose to this approach
pub fn get_lap_indices_with_sequential_search(mut telemetry_data: Ticks, lap_var: &VarInfo) -> HashMap<i32, i32> {
    let mut lap_indices: HashMap<i32, i32> = HashMap::new();
    let mut target_lap: i32 = 1;
    let mut current_index: i32 = 0;

    while let Some(tick) = telemetry_data.next() {
        // If current tick is first tick of our current target lap
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
