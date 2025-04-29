use racebudd_iracing_agent::RaceBuddTelem;

pub fn main() {
    println!("Starting...");
    let file_name: &str = r"C:\Users\Dheven\Documents\iRacing\telemetry\mercedesamggt4_brandshatch grandprix 2025-04-24 20-12-49.ibt";
    let mut telem = RaceBuddTelem::new(file_name);

    let target_laps = vec![1];
    let lap_data = telem.get_lap_data(target_laps);

    for lap in lap_data {
        for tick in lap.1 {
            println!("Throttle: {:?}", tick.throttle().float_32());
            println!("LF Brake line pressure: {:?}", tick.lf_brake_line_press().float_32());
            println!("RF Brake line pressure: {:?}", tick.rf_brake_line_press().float_32());
            println!("ABS Active: {:?}", tick.brake_abs_active().bool());
        }
    }
}

/*
Throttle: 1.0
LF Brake line pressure: 0.0
RF Brake line pressure: 0.0
ABS Active: false
Throttle: 1.0
LF Brake line pressure: 0.0
RF Brake line pressure: 0.0
ABS Active: false
Throttle: 1.0
LF Brake line pressure: 0.0
RF Brake line pressure: 0.0
ABS Active: false
Throttle: 1.0
LF Brake line pressure: 0.0
RF Brake line pressure: 0.0
ABS Active: false
*/