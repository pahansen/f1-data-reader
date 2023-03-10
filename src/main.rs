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
use std::{fs::{self, File}, path::Path, sync::Arc};
use parquet::{
    file::{
        properties::WriterProperties,
        writer::{SerializedFileWriter},
    },
    basic::{Type as PhysicalType},
    schema::{parser::parse_message_type, printer, types::Type}, data_type::FloatType
};
use binrw::BinRead;
use std::io::Seek;
use structs::packet_car_telemetry_data::PacketCarTelemetryData;
use structs::packet_header::PacketHeader;

fn main() -> std::io::Result<()> {
    let mut f1_log = std::fs::File::open("/workspaces/f1-data-reader/f1_logs/foo1.bin")?;
    //Parquet setup
    let path = Path::new("/workspaces/f1-data-reader/f1_logs/sample.parquet");
    let m_brake = Type::primitive_type_builder("m_brake", PhysicalType::FLOAT)
        .build()
        .unwrap();
    let m_throttle = Type::primitive_type_builder("m_throttle", PhysicalType::FLOAT)
        .build()
        .unwrap();

    let schema = Type::group_type_builder("schema")
        .with_fields(&mut vec![Arc::new(m_brake), Arc::new(m_throttle)])
        .build()
        .unwrap();

    let mut buf = Vec::new();
    printer::print_schema(&mut buf, &schema);
    // Parse schema from the string
    let string_schema = String::from_utf8(buf).unwrap();
    let parsed_schema = Arc::new(parse_message_type(&string_schema).unwrap());
    
    let props = Arc::new(WriterProperties::builder().build());
    let parquet_file = fs::File::create(path).unwrap();
    let mut writer = SerializedFileWriter::new(parquet_file, parsed_schema, props).unwrap();

    while let Ok(message) = PacketHeader::read(&mut f1_log) {
        println!(
            "packet_id: {}, session_uid: {}",
            message.m_packet_id, message.m_session_uid
        );
        // Skip messages that are not implemented
        match message.m_packet_id {
            0 => f1_log.seek(std::io::SeekFrom::Current(1440))?,
            1 => f1_log.seek(std::io::SeekFrom::Current(608))?,
            2 => f1_log.seek(std::io::SeekFrom::Current(948))?,
            3 => f1_log.seek(std::io::SeekFrom::Current(16))?,
            4 => f1_log.seek(std::io::SeekFrom::Current(1233))?,
            5 => f1_log.seek(std::io::SeekFrom::Current(1078))?,
            //6 => file.seek(std::io::SeekFrom::Current(1323))?,
            6 => print_car_telemetry(&f1_log, &mut writer),
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

fn print_car_telemetry(mut file: &std::fs::File, writer: &mut SerializedFileWriter<File>) -> u64 {
    let message = PacketCarTelemetryData::read(&mut file).unwrap();
    let mut m_brake_vec: Vec<f32> = Vec::new();
    let mut m_throttle_vec: Vec<f32> = Vec::new();
    for car_telemetry in message.m_car_telemetry_data {
        m_brake_vec.push(car_telemetry.m_brake);
        m_throttle_vec.push(car_telemetry.m_throttle);
    }
    
    let mut row_group_writer = writer.next_row_group().unwrap();
    if let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
        // ... write values to a column writer
        col_writer
        .typed::<FloatType>()
        .write_batch(&m_brake_vec, Some(&vec![1i16; m_brake_vec.len()][..]), None)
        .unwrap();
        col_writer.close().unwrap()
    }
    if let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
        // ... write values to a column writer
        col_writer
        .typed::<FloatType>()
        .write_batch(&m_throttle_vec, Some(&vec![1i16; m_throttle_vec.len()][..]), None)
        .unwrap();
        col_writer.close().unwrap()
    }
    row_group_writer.close().unwrap();
       
    1
}
