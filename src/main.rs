mod recorder {
    pub mod udp_recorder;
}
mod parser {
    mod car_status_data {
        pub mod struct_car_status_data;
        pub mod writer_packet_car_status_data;
    }
    mod car_telemetry_data {
        pub mod struct_car_telemetry_data;
        pub mod writer_packet_car_telemetry_data;
    }
    mod lap_data {
        pub mod struct_lap_data;
        pub mod writer_packet_laps_data;
    }
    mod motion_data {
        pub mod struct_motion_data;
    }
    mod participants_data {
        pub mod struct_participants_data;
        pub mod writer_packet_participants_data;
    }
    mod utils {
        pub mod packet_header;
        pub mod util_column_writer;
    }
    pub mod parquet_parser;
}
use clap::{Parser, Subcommand};
use parser::parquet_parser;
use recorder::udp_recorder;

#[derive(Subcommand, Debug)]
enum Mode {
    /// Record udp stream to f1 log file.
    Recorder {
        #[arg(long)]
        /// File path for F1 log.
        f1_log_file_path: String,
    },
    /// Parse f1 log file to parquet files.
    Parser {
        #[arg(long)]
        /// File path of F1 log.
        f1_log_file_path: String,
        #[arg(long)]
        /// Folder path wher parquet files should be stored.
        parquet_folder_path: String,
    },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    mode: Mode,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.mode {
        Mode::Recorder { f1_log_file_path } => {
            println!("Recording f1 log...");
            udp_recorder::record(f1_log_file_path).unwrap();
        }
        Mode::Parser {
            f1_log_file_path,
            parquet_folder_path,
        } => {
            println!("Writing f1 log to parquet...");
            let parquet_folder_path = parquet_folder_path;
            parquet_parser::parse_recorded_file(f1_log_file_path, parquet_folder_path).unwrap();
        }
    }
    Ok(())
}
