use crate::parser::utils::util_column_writer::{
    write_bool_column, write_float_column, write_int32_column, write_u64_as_bytearray_column
};
use crate::parser::utils::packet_header::PacketHeader;
use crate::parser::car_telemetry_data::packet_car_telemetry_data::PacketCarTelemetryData;
use binrw::BinRead;
use parquet::{
    file::{properties::WriterProperties, writer::SerializedFileWriter},
    schema::parser::parse_message_type,
};
use std::{
    fs::{self, File},
    path::Path,
    sync::Arc,
};

pub fn new(file_path: &Path) -> SerializedFileWriter<File> {
    let schema = "message schema {
        REQUIRED INT32 m_packet_format;
        REQUIRED BYTE_ARRAY m_session_uid;
        REQUIRED FLOAT m_session_time;
        REQUIRED BOOLEAN is_player_car;
        REQUIRED BOOLEAN is_secondary_player_car;
        REQUIRED INT32 car_telemetry_index;
        REQUIRED INT32 m_speed;
        REQUIRED FLOAT m_throttle;
        REQUIRED FLOAT m_steer;
        REQUIRED FLOAT m_brake;
        REQUIRED INT32 m_clutch;
        REQUIRED INT32 m_gear;
        REQUIRED INT32 m_engine_rpm;
        REQUIRED INT32 m_drs;
        REQUIRED INT32 m_rev_lights_percent;
        REQUIRED INT32 m_rev_lights_bit_value;
      }";
    let parsed_schema = Arc::new(parse_message_type(schema).unwrap());

    let props = Arc::new(WriterProperties::builder().build());
    let parquet_file = fs::File::create(file_path).unwrap();
    SerializedFileWriter::new(parquet_file, parsed_schema, props).unwrap()
}

pub fn write(
    packet_header: &PacketHeader,
    mut file: &std::fs::File,
    writer: &mut SerializedFileWriter<File>,
) -> u64 {
    let message = PacketCarTelemetryData::read(&mut file).unwrap();
    let len_car_telemetry = message.m_car_telemetry_data.len();
    let mut is_player_car_vec: Vec<bool> = vec![false; len_car_telemetry];
    if usize::from(packet_header.m_player_car_index) < len_car_telemetry {
        is_player_car_vec[usize::from(packet_header.m_player_car_index)] = true;
    }
    let mut is_secondary_player_car_vec: Vec<bool> = vec![false; len_car_telemetry];
    if usize::from(packet_header.m_secondary_player_car_index) < len_car_telemetry {
        is_secondary_player_car_vec[usize::from(packet_header.m_player_car_index)] = true;
    }
    let car_telemetry_index: Vec<i32> = (0..22).collect();
    let mut m_speed_vec: Vec<i32> = Vec::new();
    let mut m_throttle_vec: Vec<f32> = Vec::new();
    let mut m_steer_vec: Vec<f32> = Vec::new();
    let mut m_brake_vec: Vec<f32> = Vec::new();
    let mut m_clutch_vec: Vec<i32> = Vec::new();
    let mut m_gear_vec: Vec<i32> = Vec::new();
    let mut m_engine_rpm_vec: Vec<i32> = Vec::new();
    let mut m_drs_vec: Vec<i32> = Vec::new();
    let mut m_rev_lights_percent_vec: Vec<i32> = Vec::new();
    let mut m_rev_lights_bit_value_vec: Vec<i32> = Vec::new();
    for car_telemetry in message.m_car_telemetry_data {
        m_speed_vec.push(i32::from(car_telemetry.m_speed));
        m_throttle_vec.push(car_telemetry.m_throttle);
        m_steer_vec.push(car_telemetry.m_steer);
        m_brake_vec.push(car_telemetry.m_brake);
        m_clutch_vec.push(i32::from(car_telemetry.m_clutch));
        m_gear_vec.push(i32::from(car_telemetry.m_gear));
        m_engine_rpm_vec.push(i32::from(car_telemetry.m_engine_rpm));
        m_drs_vec.push(i32::from(car_telemetry.m_drs));
        m_rev_lights_percent_vec.push(i32::from(car_telemetry.m_rev_lights_percent));
        m_rev_lights_bit_value_vec.push(i32::from(car_telemetry.m_rev_lights_bit_value));
    }

    let mut row_group_writer = writer.next_row_group().unwrap();
    write_int32_column(
        &mut row_group_writer,
        vec![i32::from(packet_header.m_packet_format); len_car_telemetry],
        None,
        None,
    );
    write_u64_as_bytearray_column(
        &mut row_group_writer,
        vec![packet_header.m_session_uid; len_car_telemetry],
        None,
        None,
    );
    write_float_column(
        &mut row_group_writer,
        vec![packet_header.m_session_time; len_car_telemetry],
        None,
        None,
    );
    write_bool_column(&mut row_group_writer, is_player_car_vec, None, None);
    write_bool_column(
        &mut row_group_writer,
        is_secondary_player_car_vec,
        None,
        None,
    );
    write_int32_column(&mut row_group_writer, car_telemetry_index, None, None);
    write_int32_column(&mut row_group_writer, m_speed_vec, None, None);
    write_float_column(&mut row_group_writer, m_throttle_vec, None, None);
    write_float_column(&mut row_group_writer, m_steer_vec, None, None);
    write_float_column(&mut row_group_writer, m_brake_vec, None, None);
    write_int32_column(&mut row_group_writer, m_clutch_vec, None, None);
    write_int32_column(&mut row_group_writer, m_gear_vec, None, None);
    write_int32_column(&mut row_group_writer, m_engine_rpm_vec, None, None);
    write_int32_column(&mut row_group_writer, m_drs_vec, None, None);
    write_int32_column(&mut row_group_writer, m_rev_lights_percent_vec, None, None);
    write_int32_column(
        &mut row_group_writer,
        m_rev_lights_bit_value_vec,
        None,
        None,
    );

    row_group_writer.close().unwrap();
    1
}
