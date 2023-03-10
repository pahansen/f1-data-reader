use crate::PacketCarTelemetryData;
use binrw::BinRead;
use std::{fs::{self, File}, path::Path, sync::Arc};
use parquet::{
    file::{
        properties::WriterProperties,
        writer::SerializedFileWriter,
    },
    data_type::FloatType,
    basic::Type as PhysicalType,
    schema::{parser::parse_message_type, printer, types::Type}
};

pub fn new(file_path: &Path) -> SerializedFileWriter<File>{
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
    let parquet_file = fs::File::create(file_path).unwrap();
    SerializedFileWriter::new(parquet_file, parsed_schema, props).unwrap()
}

pub fn write(mut file: &std::fs::File, writer: &mut SerializedFileWriter<File>) -> u64{
    let message = PacketCarTelemetryData::read(&mut file).unwrap();
    let mut m_brake_vec: Vec<f32> = Vec::new();
    let mut m_throttle_vec: Vec<f32> = Vec::new();
    for car_telemetry in message.m_car_telemetry_data {
        m_brake_vec.push(car_telemetry.m_brake);
        m_throttle_vec.push(car_telemetry.m_throttle);
    }
    
    let mut row_group_writer = writer.next_row_group().unwrap();
    if let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
        col_writer
        .typed::<FloatType>()
        .write_batch(&m_brake_vec, Some(&vec![1i16; m_brake_vec.len()][..]), None)
        .unwrap();
        col_writer.close().unwrap()
    }
    if let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
        col_writer
        .typed::<FloatType>()
        .write_batch(&m_throttle_vec, Some(&vec![1i16; m_throttle_vec.len()][..]), None)
        .unwrap();
        col_writer.close().unwrap()
    }
    row_group_writer.close().unwrap();
    1
}