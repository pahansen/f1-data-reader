use crate::parquet_writers::{
    writer_packet_car_telemetry_data, writer_packet_laps_data, writer_packet_participants_data,
};
use crate::structs::packet_header::PacketHeader;
use binrw::BinRead;
use std::io::Seek;
use std::{fs::File, path::Path};

pub fn parse_recorded_file(f1_log_path: &str, parquet_folder_path: &str) -> std::io::Result<()> {
    let mut f1_log = File::open(f1_log_path)?;
    // Parquet setup
    // Car Telemetry Data
    let folder_path: String = parquet_folder_path.to_owned();
    let car_telemetry_file = folder_path + "/car_telemetry.parquet";
    let car_telemetry_path = Path::new(&car_telemetry_file);
    // Participants Data
    let folder_path: String = parquet_folder_path.to_owned();
    let participants_file = folder_path + "/participants.parquet";
    let participants_path = Path::new(&participants_file);
    // Lap Data
    let folder_path: String = parquet_folder_path.to_owned();
    let laps_file = folder_path + "/laps.parquet";
    let laps_path = Path::new(&laps_file);

    let mut car_telemetry_writer = writer_packet_car_telemetry_data::new(car_telemetry_path);
    let mut participants_writer = writer_packet_participants_data::new(participants_path);
    let mut laps_writer = writer_packet_laps_data::new(laps_path);

    while let Ok(message) = PacketHeader::read(&mut f1_log) {
        // Skip messages that are not implemented
        match message.m_packet_id {
            0 => f1_log.seek(std::io::SeekFrom::Current(1440))?,
            1 => f1_log.seek(std::io::SeekFrom::Current(608))?,
            2 => writer_packet_laps_data::write(&message, &f1_log, &mut laps_writer),
            //2 => f1_log.seek(std::io::SeekFrom::Current(948))?,
            3 => f1_log.seek(std::io::SeekFrom::Current(16))?,
            4 => {
                writer_packet_participants_data::write(&message, &f1_log, &mut participants_writer)
            }
            //4 => f1_log.seek(std::io::SeekFrom::Current(1233))?,
            5 => f1_log.seek(std::io::SeekFrom::Current(1078))?,
            //6 => file.seek(std::io::SeekFrom::Current(1323))?,
            6 => writer_packet_car_telemetry_data::write(
                &message,
                &f1_log,
                &mut car_telemetry_writer,
            ),
            7 => f1_log.seek(std::io::SeekFrom::Current(1034))?,
            8 => f1_log.seek(std::io::SeekFrom::Current(991))?,
            9 => f1_log.seek(std::io::SeekFrom::Current(1167))?,
            10 => f1_log.seek(std::io::SeekFrom::Current(924))?,
            11 => f1_log.seek(std::io::SeekFrom::Current(1131))?,

            _ => 0, // Do nothing
        };
    }
    car_telemetry_writer.close().unwrap();
    participants_writer.close().unwrap();
    laps_writer.close().unwrap();
    Ok(())
}
