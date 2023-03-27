use crate::parser::lap_data::struct_lap_data::PacketLapData;
use crate::parser::utils::packet_header::PacketHeader;
use crate::parser::utils::util_column_writer::{
    write_bool_column, write_float_column, write_int32_column, write_int64_column,
    write_u64_as_bytearray_column,
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
        REQUIRED INT64 m_last_lap_time_in_ms;
        REQUIRED INT64 m_current_lap_time_in_ms;
        REQUIRED INT32 m_sector1_time_in_ms;
        REQUIRED INT32 m_sector2_time_in_ms;
        REQUIRED FLOAT m_lap_distance;
        REQUIRED INT32 m_current_lap_num;
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
    let message = PacketLapData::read(&mut file).unwrap();
    let number_of_cars = message.m_lap_data.len();
    let mut is_player_car_vec: Vec<bool> = vec![false; number_of_cars];
    if usize::from(packet_header.m_player_car_index) < number_of_cars {
        is_player_car_vec[usize::from(packet_header.m_player_car_index)] = true;
    }
    let mut is_secondary_player_car_vec: Vec<bool> = vec![false; number_of_cars];
    if usize::from(packet_header.m_secondary_player_car_index) < number_of_cars {
        is_secondary_player_car_vec[usize::from(packet_header.m_secondary_player_car_index)] = true;
    }
    let car_index: Vec<i32> = (0..22).collect();
    let m_last_lap_time_in_ms = message
        .m_lap_data
        .iter()
        .map(|c| i64::from(c.m_last_lap_time_in_ms))
        .collect::<Vec<i64>>();

    let m_current_lap_time_in_ms = message
        .m_lap_data
        .iter()
        .map(|c| i64::from(c.m_current_lap_time_in_ms))
        .collect::<Vec<i64>>();

    let m_sector1_time_in_ms = message
        .m_lap_data
        .iter()
        .map(|c| i32::from(c.m_sector1_time_in_ms))
        .collect::<Vec<i32>>();

    let m_sector2_time_in_ms = message
        .m_lap_data
        .iter()
        .map(|c| i32::from(c.m_sector2_time_in_ms))
        .collect::<Vec<i32>>();

    let m_lap_distance = message
        .m_lap_data
        .iter()
        .map(|c| c.m_lap_distance)
        .collect::<Vec<f32>>();

    let m_current_lap_num = message
        .m_lap_data
        .iter()
        .map(|c| i32::from(c.m_current_lap_num))
        .collect::<Vec<i32>>();

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
    write_int64_column(&mut row_group_writer, m_last_lap_time_in_ms);
    write_int64_column(&mut row_group_writer, m_current_lap_time_in_ms);
    write_int32_column(&mut row_group_writer, m_sector1_time_in_ms);
    write_int32_column(&mut row_group_writer, m_sector2_time_in_ms);
    write_float_column(&mut row_group_writer, m_lap_distance);
    write_int32_column(&mut row_group_writer, m_current_lap_num);

    row_group_writer.close().unwrap();

    1
}
