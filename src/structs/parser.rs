use crate::structs::packet_header::PacketHeader;
use crate::parquet_writers::writer_packet_car_telemetry_data;
use std::{fs::File, path::Path};
use binrw::BinRead;
use std::io::Seek;

pub fn parse_recorded_file() -> std::io::Result<()> {
    let mut f1_log = File::open("/workspaces/f1-data-reader/f1_logs/foo1.bin")?;
    //Parquet setup
    let path = Path::new("/workspaces/f1-data-reader/f1_logs/sample.parquet");
   
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