use std::time::Instant;

use racebudd_iracing_agent::RaceBuddTelem;

pub fn main() {
    println!("Starting...");
    let file_name: &str = r"C:\Users\Dheven\Documents\iRacing\telemetry\toyotagr86_willow international 2025-05-05 19-21-42.ibt";
    let mut telem = RaceBuddTelem::new(file_name);

    println!("Lap indices: {:?}", telem.lap_indices());

    let target_laps = vec![1,2,3,4,5,6,7,8,9,10,11,12,13];
    let test1 = Instant::now();

    let lap_data = telem.get_lap_data(&target_laps);

    let mut lap_data_len = 0;
    for (_, data) in lap_data {
        lap_data_len += data.len();
    }

    println!("Lap data length: {:?}", lap_data_len);

    println!("Duration: {:?}", test1.elapsed());

}