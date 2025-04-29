use racebudd_iracing_agent::RaceBuddTelem;

pub fn main() {
    println!("Starting...");
    let file_name: &str = r"C:\Users\Dheven\Documents\iRacing\telemetry\mercedesamggt4_brandshatch grandprix 2025-04-24 20-12-49.ibt";
    let mut telem = RaceBuddTelem::new(file_name);

    let target_laps = vec![1];
    let lap_data = telem.get_lap_data(target_laps);
}