// use std::fs::File;
// use std::io::Write;
// use std::net::UdpSocket;

// fn main() -> std::io::Result<()> {
//     let socket = UdpSocket::bind("0.0.0.0:20777")?; // for UDP4
//                                                     //let socket = UdpSocket::bind("[::]:2000")?;  // for UDP4/6
//     let mut buf = [0; 2048];
//     let mut file = File::create("/workspaces/f1-data-reader/f1_logs/foo1.binlog")?;

//     loop {
//         let (number_of_bytes, _src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
//         file.write_all(&buf[..number_of_bytes])?;
//     }
// }

mod structs {
    pub mod packet_car_telemetry_data;
    pub mod packet_header;
}
mod parquet_writers {
    pub mod m_car_telemetry_writer;
}
use std::{fs::File, path::Path};
use binrw::BinRead;
use parquet_writers::m_car_telemetry_writer;
use std::io::Seek;
use structs::packet_car_telemetry_data::PacketCarTelemetryData;
use structs::packet_header::PacketHeader;

fn main() -> std::io::Result<()> {
    let mut f1_log = File::open("/workspaces/f1-data-reader/f1_logs/foo1.bin")?;
    //Parquet setup
    let path = Path::new("/workspaces/f1-data-reader/f1_logs/sample.parquet");
   
    let mut writer = m_car_telemetry_writer::new(path);

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
            6 => m_car_telemetry_writer::write(&f1_log, &mut writer),
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