use crate::PacketCarTelemetryData;
use binrw::BinRead;
use std::{fs::{self, File}, path::Path, sync::Arc};
use parquet::{
    file::{
        properties::WriterProperties,
        writer::SerializedFileWriter,
    },
    data_type::{FloatType, Int32Type},
    schema::parser::parse_message_type
};

pub fn new(file_path: &Path) -> SerializedFileWriter<File>{
    let schema = "message schema {
        REQUIRED INT32 m_speed;
        REQUIRED FLOAT m_brake;
        REQUIRED FLOAT m_throttle;
      }";
    let parsed_schema = Arc::new(parse_message_type(schema).unwrap());
    
    let props = Arc::new(WriterProperties::builder().build());
    let parquet_file = fs::File::create(file_path).unwrap();
    SerializedFileWriter::new(parquet_file, parsed_schema, props).unwrap()
}

pub fn write(mut file: &std::fs::File, writer: &mut SerializedFileWriter<File>) -> u64{
    let message = PacketCarTelemetryData::read(&mut file).unwrap();
    let mut m_speed_vec: Vec<i32> = Vec::new();
    let mut m_brake_vec: Vec<f32> = Vec::new();
    let mut m_throttle_vec: Vec<f32> = Vec::new();
    for car_telemetry in message.m_car_telemetry_data {
        m_speed_vec.push(i32::from(car_telemetry.m_speed));
        m_brake_vec.push(car_telemetry.m_brake);
        m_throttle_vec.push(car_telemetry.m_throttle);
    }
    
    let mut row_group_writer = writer.next_row_group().unwrap();
    if let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
        col_writer
        .typed::<Int32Type>()
        .write_batch(&m_speed_vec, Some(&vec![1i16; m_speed_vec.len()][..]), None)
        .unwrap();
        col_writer.close().unwrap()
    }
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