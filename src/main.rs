mod parser;
mod headers;
mod utils;

use headers::{FileInfo, SessionInfo};
use parser::Parser;

fn main() {
    println!("Starting RaceBuddTelemetryAgent execution...");

    // iRacing ibt file path
    let file_path = String::from(r"C:\Users\Dheven\Documents\iRacing\telemetry\bmwm2csr_summit summit raceway 2025-03-22 17-32-33.ibt");
    println!("Parsing Telemetry File: {}", &file_path);

    // instantiate Struct for ParsingIbtFile
    let mut parser = Parser::new(&file_path);
    let file_info: FileInfo = parser.file_info();
    let session_info: SessionInfo = parser.session_info();


    println!("File info tick rate: {}", file_info.tick_rate);
    println!("Session Lap Count: {}", session_info.lap_count);

}


