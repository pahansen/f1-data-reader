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
use clap::Parser;
use recorder::udp_recorder;
use structs::parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Function call that should be executed.
    #[arg(long)]
    function_call: String,
    /// File path of F1 log.
    #[arg(long)]
    f1_log_file_path: String,
    /// File path of parquet file.
    #[arg(long)]
    parquet_file_path: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.function_call == "record" {
        println!("Recording f1 log...");
        udp_recorder::record(&args.f1_log_file_path).unwrap();
    } else if args.parquet_file_path.is_some() {
        println!("Writing f1 log to parquet...");
        let parquet_file_path = args.parquet_file_path.unwrap();
        parser::parse_recorded_file(&args.f1_log_file_path, &parquet_file_path).unwrap();
    }
    Ok(())
}
