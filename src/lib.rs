mod parser;
mod headers;
mod telemetry_data;
mod utils;

use std::{collections::HashMap, thread::current};

use headers::{FileInfo, SessionInfo, VarInfo};
use parser::Parser;
use telemetry_data::{Tick, Ticks, ChannelValue};

#[derive(Debug)]
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

    // Returns all lap data in Race state for the given laps
    pub fn get_lap_data(&mut self, laps: &Vec<i32>) -> HashMap<i32, Vec<Tick>> {
        let lap_var = self.available_vars().iter().find(|var| var.name == "Lap").cloned().unwrap();
        let mut lap_data: HashMap<i32, Vec<Tick>> = HashMap::new();
        let mut telemetry_data: Ticks<'_> = self.telemetry_data();

        // Initialize the HashMap with empty vectors for each target lap
        for lap in laps {
            lap_data.insert(*lap, Vec::new());
        }

        
        while let Some(tick) = telemetry_data.next() {
            // StateInvalid: 0
            // StateGetInCar: 1
            // StateWarmup: 2
            // StateParadeLaps: 3
            // StateRacing: 4
            // StateCheckered: 5
            // StateCoolDown: 6
            let session_state = tick.session_state().int();

            // If we are in racing or checkered state, add the tick to the data
            // Is the last lap in a checkered state?
            if session_state == 4 {
                let current_lap = tick.get_value(&lap_var).unwrap().int();
                
                if laps.contains(&current_lap) {
                    if let Some(data) = lap_data.get_mut(&current_lap) {
                        data.push(tick);
                    }
                }
            }
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
        // If current tick is first tick of our current target lap and in racing state
        if tick.lap().int() == target_lap && tick.session_state().int() == 4 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_racebudd_telem_creation() {
        let telem = RaceBuddTelem::new("test_ibt_file.ibt");
        assert!(Path::new("test_ibt_file.ibt").exists(), "Test file should exist");
    }

    #[test]
    fn test_available_vars() {
        let mut telem = RaceBuddTelem::new("test_ibt_file.ibt");
        let vars = telem.available_vars();
        assert!(!vars.is_empty(), "Should have available variables");
        
        // Check for some expected variables
        let var_names: Vec<String> = vars.iter().map(|v| v.name.clone()).collect();
        assert!(var_names.contains(&"Lap".to_string()), "Should have Lap variable");
        assert!(var_names.contains(&"Speed".to_string()), "Should have Speed variable");
        assert!(var_names.contains(&"SessionState".to_string()), "Should have SessionState variable");
    }

    #[test]
    fn test_telemetry_data_iterator() {
        let mut telem = RaceBuddTelem::new("test_ibt_file.ibt");
        let mut telemetry = telem.telemetry_data();
        
        // Test that we can get at least one tick
        let first_tick = telemetry.next();
        assert!(first_tick.is_some(), "Should be able to read at least one tick");
        
        if let Some(tick) = first_tick {
            // Test some basic getters
            let speed = tick.speed();
            assert!(matches!(speed, ChannelValue::Float32(_)), "Speed should be a float32");
            
            let lap = tick.lap();
            assert!(matches!(lap, ChannelValue::Int(_)), "Lap should be an int");

            let session_state = tick.session_state();
            assert!(matches!(session_state, ChannelValue::Int(_)), "SessionState should be an int");
        }
    }

    #[test]
    fn test_lap_indices() {
        let mut telem = RaceBuddTelem::new("test_ibt_file.ibt");
        let lap_indices = telem.lap_indices();
        
        assert!(!lap_indices.is_empty(), "Should have lap indices");
        assert!(lap_indices.contains_key(&1), "Should have first lap");
        
        // Verify lap indices are in ascending order
        let mut prev_index = -1;
        for lap in 1..=lap_indices.len() as i32 {
            if let Some(&index) = lap_indices.get(&lap) {
                assert!(index > prev_index, "Lap indices should be in ascending order");
                prev_index = index;
            }
        }

        // Verify that all indexed laps are from racing state
        let mut telemetry: Ticks<'_> = telem.telemetry_data();
        
        // Convert HashMap to Vec and sort by index to ensure sequential access
        let mut lap_index_pairs: Vec<_> = lap_indices.iter().collect();
        lap_index_pairs.sort_by_key(|(_, index)| **index);

        let mut prev_index = 0;
        for (&lap, &index) in lap_index_pairs {
            // Calculate how many ticks to advance
            let ticks_to_advance = index as usize - prev_index;
            let tick = telemetry.nth(ticks_to_advance);

            if let Some(tick) = tick {
                assert_eq!(tick.session_state().int(), 4, "Lap {} should be in racing state", lap);
                assert_eq!(tick.lap().int(), lap, "Index {} should point to the correct lap {}", index, lap);
            }
            prev_index = index as usize + 1; // +1 since nth consumes the element
        }
    }

    #[test]
    fn test_get_lap_data() {
        let mut telem = RaceBuddTelem::new("test_ibt_file.ibt");
        let target_laps = vec![1, 2];
        let lap_data = telem.get_lap_data(&target_laps);
        
        assert_eq!(lap_data.len(), target_laps.len(), "Should have data for all requested laps");
        
        for lap in target_laps {
            assert!(lap_data.contains_key(&lap), "Should have data for lap {}", lap);
            let data = lap_data.get(&lap).unwrap();
            assert!(!data.is_empty(), "Lap {} should have telemetry data", lap);
            
            // Verify data points are from the correct lap and in racing state
            for tick in data {
                let tick_lap = tick.lap().int();
                let session_state = tick.session_state().int();
                
                assert_eq!(tick_lap, lap, "All data points should be from lap {}", lap);
                assert_eq!(session_state, 4, "All data points should be in racing state");
                
                // Verify lap distance percentage is within valid range
                let lap_dist_pct = tick.lap_dist_pct().float_32();
                assert!(lap_dist_pct >= 0.0 && lap_dist_pct <= 1.0, 
                    "Lap distance percentage should be between 0 and 1");
            }
        }
    }
}
