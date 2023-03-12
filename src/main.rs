mod recorder {
    pub mod udp_recorder;
}
mod structs {
    pub mod packet_car_telemetry_data;
    pub mod packet_header;
    pub mod packet_participants_data;
    pub mod parser;
}
mod parquet_writers {
    pub mod util_column_writer;
    pub mod writer_packet_car_telemetry_data;
    pub mod writer_packet_participants_data;
}
use clap::{Parser, ValueEnum};
use recorder::udp_recorder;
use structs::parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// What mode to run the program in
    #[arg(value_enum)]
    mode: Mode,
    /// File path of F1 log.
    #[arg(long)]
    f1_log_file_path: String,
    /// File path of parquet file.
    #[arg(long)]
    parquet_folder_path: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Record f1 log
    Recorder,
    /// Parse recorded f1 log
    Parser,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.mode {
        Mode::Recorder => {
            println!("Recording f1 log...");
            udp_recorder::record(&cli.f1_log_file_path).unwrap();
        }
        Mode::Parser => {
            println!("Writing f1 log to parquet...");
            let parquet_folder_path = cli.parquet_folder_path.unwrap();
            parser::parse_recorded_file(&cli.f1_log_file_path, &parquet_folder_path).unwrap();
        }
    }
    Ok(())
}
