mod recorder {
    pub mod udp_recorder;
}
mod structs {
    pub mod packet_car_telemetry_data;
    pub mod packet_header;
    pub mod parser;
}
mod parquet_writers {
    pub mod util_column_writer;
    pub mod writer_packet_car_telemetry_data;
}
use recorder::udp_recorder;
use std::env;
use structs::parser;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let function_call = &args[1];
    let f1_log_path = &args[2];
    let parsed_file_path = &args[3];
    if function_call == "record" {
        println!("Recording f1 log...");
        udp_recorder::record(f1_log_path).unwrap();
    } else {
        println!("Writing f1 log to parquet...");
        parser::parse_recorded_file(f1_log_path, parsed_file_path).unwrap();
    }
    Ok(())
}
