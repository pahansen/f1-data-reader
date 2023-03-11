mod recorder {
    pub mod udp_recorder;
}
mod structs {
    pub mod packet_car_telemetry_data;
    pub mod packet_header;
    pub mod parser;
}
mod parquet_writers {
    pub mod writer_packet_car_telemetry_data;
    pub mod util_column_writer;
}
use std::env;
use structs::parser;
use recorder::udp_recorder;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let function_call = &args[1];
    if function_call == "record" {
        println!("Recording udp data...");
        udp_recorder::record().unwrap();
    } else {
        println!("Writing to recorded udp data to parquet...");
        parser::parse_recorded_file().unwrap();
    }
    Ok(())
}