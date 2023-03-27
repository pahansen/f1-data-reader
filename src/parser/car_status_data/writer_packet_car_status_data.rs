use crate::parser::car_status_data::struct_car_status_data::PacketCarStatusData;
use crate::parser::utils::packet_header::PacketHeader;
use crate::parser::utils::util_column_writer::{
    write_bool_column, write_float_column, write_int32_column, write_u64_as_bytearray_column,
};
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
        REQUIRED INT32 car_index;
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
    let number_of_cars = message.m_car_status_data.len();
    let mut is_player_car_vec: Vec<bool> = vec![false; number_of_cars];
    if usize::from(packet_header.m_player_car_index) < number_of_cars {
        is_player_car_vec[usize::from(packet_header.m_player_car_index)] = true;
    }
    let mut is_secondary_player_car_vec: Vec<bool> = vec![false; number_of_cars];
    if usize::from(packet_header.m_secondary_player_car_index) < number_of_cars {
        is_secondary_player_car_vec[usize::from(packet_header.m_secondary_player_car_index)] = true;
    }
    let car_index: Vec<i32> = (0..22).collect();
    let m_fuel_in_tank = message
        .m_car_status_data
        .iter()
        .map(|c| c.m_fuel_in_tank)
        .collect::<Vec<f32>>();
    let m_fuel_capacity = message
        .m_car_status_data
        .iter()
        .map(|c| c.m_fuel_capacity)
        .collect::<Vec<f32>>();
    let m_fuel_remaining_laps = message
        .m_car_status_data
        .iter()
        .map(|c| c.m_fuel_remaining_laps)
        .collect::<Vec<f32>>();
    let m_actual_tyre_compund = message
        .m_car_status_data
        .iter()
        .map(|c| i32::from(c.m_actual_tyre_compound))
        .collect::<Vec<i32>>();
    let m_tyres_age_laps = message
        .m_car_status_data
        .iter()
        .map(|c| i32::from(c.m_tyres_age_laps))
        .collect::<Vec<i32>>();
    let m_ers_store_energy = message
        .m_car_status_data
        .iter()
        .map(|c| c.m_ers_store_energy)
        .collect::<Vec<f32>>();
    let m_ers_harvested_this_lap_mguk = message
        .m_car_status_data
        .iter()
        .map(|c| c.m_ers_harvested_this_lap_mguk)
        .collect::<Vec<f32>>();
    let m_ers_harvested_this_lap_mguh = message
        .m_car_status_data
        .iter()
        .map(|c| c.m_ers_harvested_this_lap_mguh)
        .collect::<Vec<f32>>();
    let m_ers_deployed_this_lap = message
        .m_car_status_data
        .iter()
        .map(|c| c.m_ers_deployed_this_lap)
        .collect::<Vec<f32>>();

    let mut row_group_writer = writer.next_row_group().unwrap();
    write_int32_column(
        &mut row_group_writer,
        vec![i32::from(packet_header.m_packet_format); number_of_cars],
    );
    write_u64_as_bytearray_column(
        &mut row_group_writer,
        vec![packet_header.m_session_uid; number_of_cars],
    );
    write_float_column(
        &mut row_group_writer,
        vec![packet_header.m_session_time; number_of_cars],
    );
    write_bool_column(&mut row_group_writer, is_player_car_vec);
    write_bool_column(&mut row_group_writer, is_secondary_player_car_vec);
    write_int32_column(&mut row_group_writer, car_index);
    write_float_column(&mut row_group_writer, m_fuel_in_tank);
    write_float_column(&mut row_group_writer, m_fuel_capacity);
    write_float_column(&mut row_group_writer, m_fuel_remaining_laps);
    write_int32_column(&mut row_group_writer, m_actual_tyre_compund);
    write_int32_column(&mut row_group_writer, m_tyres_age_laps);
    write_float_column(&mut row_group_writer, m_ers_store_energy);
    write_float_column(&mut row_group_writer, m_ers_harvested_this_lap_mguk);
    write_float_column(&mut row_group_writer, m_ers_harvested_this_lap_mguh);
    write_float_column(&mut row_group_writer, m_ers_deployed_this_lap);

    row_group_writer.close().unwrap();
    1
}
