use crate::PacketCarTelemetryData;
use crate::parquet_writers::util_column_writer::{write_float_column, write_int32_column};
use binrw::BinRead;
use std::{fs::{self, File}, path::Path, sync::Arc};
use parquet::{
    file::{
        properties::WriterProperties,
        writer::SerializedFileWriter,
    },
    schema::parser::parse_message_type
};

pub fn new(file_path: &Path) -> SerializedFileWriter<File>{
    let schema = "message schema {
        REQUIRED INT32 m_speed;
        REQUIRED FLOAT m_throttle;
        REQUIRED FLOAT m_steer;
        REQUIRED FLOAT m_brake;
        REQUIRED INT32 m_clutch;
        REQUIRED INT32 m_engine_rpm;
        REQUIRED INT32 m_drs;
      }";
    let parsed_schema = Arc::new(parse_message_type(schema).unwrap());
    
    let props = Arc::new(WriterProperties::builder().build());
    let parquet_file = fs::File::create(file_path).unwrap();
    SerializedFileWriter::new(parquet_file, parsed_schema, props).unwrap()
}

pub fn write(mut file: &std::fs::File, writer: &mut SerializedFileWriter<File>) -> u64{
    let message = PacketCarTelemetryData::read(&mut file).unwrap();
    let mut m_speed_vec: Vec<i32> = Vec::new();
    let mut m_throttle_vec: Vec<f32> = Vec::new();
    let mut m_steer_vec: Vec<f32> = Vec::new();
    let mut m_brake_vec: Vec<f32> = Vec::new();
    let mut m_clutch_vec: Vec<i32> = Vec::new();
    let mut m_engine_rpm_vec: Vec<i32> = Vec::new();
    let mut m_drs_vec: Vec<i32> = Vec::new();
    for car_telemetry in message.m_car_telemetry_data {
        m_speed_vec.push(i32::from(car_telemetry.m_speed));
        m_throttle_vec.push(car_telemetry.m_throttle);
        m_steer_vec.push(car_telemetry.m_steer);
        m_brake_vec.push(car_telemetry.m_brake);
        m_clutch_vec.push(i32::from(car_telemetry.m_clutch));
        m_engine_rpm_vec.push(i32::from(car_telemetry.m_engine_rpm));
        m_drs_vec.push(i32::from(car_telemetry.m_drs));
    }
    let mut row_group_writer = writer.next_row_group().unwrap();

    write_int32_column(&mut row_group_writer, m_speed_vec, None, None);
    write_float_column(&mut row_group_writer, m_throttle_vec, None, None);    
    write_float_column(&mut row_group_writer, m_steer_vec, None, None);
    write_float_column(&mut row_group_writer, m_brake_vec, None, None);
    write_int32_column(&mut row_group_writer, m_clutch_vec, None, None);
    write_int32_column(&mut row_group_writer, m_engine_rpm_vec, None, None);
    write_int32_column(&mut row_group_writer, m_drs_vec, None, None);
    
    row_group_writer.close().unwrap();
    1
}

