use crate::parser::utils::util_column_writer::{
    write_bool_column, write_float_column, write_int32_column, write_u64_as_bytearray_column
};
use crate::parser::utils::packet_header::PacketHeader;
use crate::parser::car_status_data::struct_car_status_data::PacketCarStatusData;
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
        REQUIRED INT32 car_status_index;
        REQUIRED FLOAT m_fuel_in_tank;
        REQUIRED FLOAT m_fuel_capacity;
        REQUIRED FLOAT m_fuel_remaining_laps;
        REQUIRED INT32 m_actual_tyre_compund;
        REQUIRED INT32 m_tyres_age_laps;
        REQUIRED FLOAT m_ers_store_energy;
        REQUIRED FLOAT m_ers_harvested_this_lap_mguk;
        REQUIRED FLOAT m_ers_harvested_this_lap_mguh;
        REQUIRED FLOAT m_ers_deployed_this_lap;
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
    let message = PacketCarStatusData::read(&mut file).unwrap();
    let len_car_telemetry = message.m_car_status_data.len();
    let mut is_player_car_vec: Vec<bool> = vec![false; len_car_telemetry];
    if usize::from(packet_header.m_player_car_index) < len_car_telemetry {
        is_player_car_vec[usize::from(packet_header.m_player_car_index)] = true;
    }
    let mut is_secondary_player_car_vec: Vec<bool> = vec![false; len_car_telemetry];
    if usize::from(packet_header.m_secondary_player_car_index) < len_car_telemetry {
        is_secondary_player_car_vec[usize::from(packet_header.m_player_car_index)] = true;
    }
    let car_status_index: Vec<i32> = (0..22).collect();
    let mut m_fuel_in_tank_vec: Vec<f32> = Vec::new();
    let mut m_fuel_capacity_vec: Vec<f32> = Vec::new();
    let mut m_fuel_remaining_laps: Vec<f32> = Vec::new();
    let mut m_actual_tyre_compund_vec: Vec<i32> = Vec::new();
    let mut m_tyres_age_laps_vec: Vec<i32> = Vec::new();
    let mut m_ers_store_energy_vec: Vec<f32> = Vec::new();
    let mut m_ers_harvested_this_lap_mguk_vec: Vec<f32> = Vec::new();
    let mut m_ers_harvested_this_lap_mguh_vec: Vec<f32> = Vec::new();
    let mut m_ers_deployed_this_lap_vec: Vec<f32> = Vec::new();

    for car_status in message.m_car_status_data {
        m_fuel_in_tank_vec.push(car_status.m_fuel_in_tank);
        m_fuel_capacity_vec.push(car_status.m_fuel_capacity);
        m_fuel_remaining_laps.push(car_status.m_fuel_remaining_laps);
        m_actual_tyre_compund_vec.push(i32::from(car_status.m_actual_tyre_compound));
        m_tyres_age_laps_vec.push(i32::from(car_status.m_tyres_age_laps));
        m_ers_store_energy_vec.push(car_status.m_ers_store_energy);
        m_ers_harvested_this_lap_mguk_vec.push(car_status.m_ers_harvested_this_lap_mguk);
        m_ers_harvested_this_lap_mguh_vec.push(car_status.m_ers_harvested_this_lap_mguh);
        m_ers_deployed_this_lap_vec.push(car_status.m_ers_deployed_this_lap);
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
    write_int32_column(&mut row_group_writer, car_status_index, None, None);
    write_float_column(&mut row_group_writer, m_fuel_in_tank_vec, None, None);
    write_float_column(&mut row_group_writer, m_fuel_capacity_vec, None, None);
    write_float_column(&mut row_group_writer, m_fuel_remaining_laps, None, None);
    write_int32_column(&mut row_group_writer, m_actual_tyre_compund_vec, None, None);
    write_int32_column(&mut row_group_writer, m_tyres_age_laps_vec, None, None);
    write_float_column(&mut row_group_writer, m_ers_store_energy_vec, None, None);
    write_float_column(
        &mut row_group_writer,
        m_ers_harvested_this_lap_mguk_vec,
        None,
        None,
    );
    write_float_column(
        &mut row_group_writer,
        m_ers_harvested_this_lap_mguh_vec,
        None,
        None,
    );
    write_float_column(
        &mut row_group_writer,
        m_ers_deployed_this_lap_vec,
        None,
        None,
    );

    row_group_writer.close().unwrap();
    1
}
