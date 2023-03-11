use crate::parquet_writers::writer_packet_car_telemetry_data;
use crate::structs::packet_header::PacketHeader;
use binrw::BinRead;
use std::io::Seek;
use std::{fs::File, path::Path};

pub fn parse_recorded_file(f1_log_path: &str, parquet_path: &str) -> std::io::Result<()> {
    let mut f1_log = File::open(f1_log_path)?;
    //Parquet setup
    let path = Path::new(parquet_path);

    let mut writer = writer_packet_car_telemetry_data::new(path);

    while let Ok(message) = PacketHeader::read(&mut f1_log) {
        // Skip messages that are not implemented
        match message.m_packet_id {
            0 => f1_log.seek(std::io::SeekFrom::Current(1440))?,
            1 => f1_log.seek(std::io::SeekFrom::Current(608))?,
            2 => f1_log.seek(std::io::SeekFrom::Current(948))?,
            3 => f1_log.seek(std::io::SeekFrom::Current(16))?,
            4 => f1_log.seek(std::io::SeekFrom::Current(1233))?,
            5 => f1_log.seek(std::io::SeekFrom::Current(1078))?,
            //6 => file.seek(std::io::SeekFrom::Current(1323))?,
            6 => writer_packet_car_telemetry_data::write(&message, &f1_log, &mut writer),
            7 => f1_log.seek(std::io::SeekFrom::Current(1034))?,
            8 => f1_log.seek(std::io::SeekFrom::Current(991))?,
            9 => f1_log.seek(std::io::SeekFrom::Current(1167))?,
            10 => f1_log.seek(std::io::SeekFrom::Current(924))?,
            11 => f1_log.seek(std::io::SeekFrom::Current(1131))?,

            _ => 0, // Do nothing
        };
    }
    writer.close().unwrap();
    Ok(())
}
