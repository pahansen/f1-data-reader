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
use binrw::BinRead;
use polars::prelude::*;
use std::io::Seek;
use structs::packet_car_telemetry_data::PacketCarTelemetryData;
use structs::packet_header::PacketHeader;

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("/workspaces/f1-data-reader/f1_logs/foo1.bin")?;
    while let Ok(message) = PacketHeader::read(&mut file) {
        println!(
            "packet_id: {}, session_uid: {}",
            message.m_packet_id, message.m_session_uid
        );
        // Skip messages that are not implemented
        match message.m_packet_id {
            0 => file.seek(std::io::SeekFrom::Current(1440))?,
            1 => file.seek(std::io::SeekFrom::Current(608))?,
            2 => file.seek(std::io::SeekFrom::Current(948))?,
            3 => file.seek(std::io::SeekFrom::Current(16))?,
            4 => file.seek(std::io::SeekFrom::Current(1233))?,
            5 => file.seek(std::io::SeekFrom::Current(1078))?,
            //6 => file.seek(std::io::SeekFrom::Current(1323))?,
            6 => print_car_telemetry(&file),
            7 => file.seek(std::io::SeekFrom::Current(1034))?,
            8 => file.seek(std::io::SeekFrom::Current(991))?,
            9 => file.seek(std::io::SeekFrom::Current(1167))?,
            10 => file.seek(std::io::SeekFrom::Current(924))?,
            11 => file.seek(std::io::SeekFrom::Current(1131))?,

            _ => 0, // Do nothing
        };
    }
    Ok(())
}

fn print_car_telemetry(mut file: &std::fs::File) -> u64 {
    let message = PacketCarTelemetryData::read(&mut file).unwrap();
    let mut m_brake_vec: Vec<f32> = Vec::new();
    for car_telemetry in message.m_car_telemetry_data {
        m_brake_vec.push(car_telemetry.m_brake);
    };
    let my_series = Series::new("m_suggested_gear", m_brake_vec);
    let mut df = df!("m_brake" => my_series).unwrap();
    let mut file = std::fs::File::create("/workspaces/f1-data-reader/test.parquet").unwrap();
    ParquetWriter::new(&mut file).finish(&mut df).unwrap();
    1
}
