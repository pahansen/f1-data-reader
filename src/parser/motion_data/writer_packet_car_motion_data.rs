use crate::parser::motion_data::struct_motion_data::PacketMotionData;
use crate::parser::utils::packet_header::PacketHeader;
use crate::parser::utils::util_column_writer::{
    write_bool_column, write_float_column, write_int32_column,
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
        REQUIRED FLOAT m_world_position_x;
        REQUIRED FLOAT m_world_position_y;
        REQUIRED FLOAT m_world_position_z;
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
    let message = PacketMotionData::read(&mut file).unwrap();
    let number_of_cars = message.m_car_motion_data.len();
    let mut is_player_car_vec: Vec<bool> = vec![false; number_of_cars];
    if usize::from(packet_header.m_player_car_index) < number_of_cars {
        is_player_car_vec[usize::from(packet_header.m_player_car_index)] = true;
    }
    let mut is_secondary_player_car_vec: Vec<bool> = vec![false; number_of_cars];
    if usize::from(packet_header.m_secondary_player_car_index) < number_of_cars {
        is_secondary_player_car_vec[usize::from(packet_header.m_secondary_player_car_index)] = true;
    }
    let car_index: Vec<i32> = (0..22).collect();
    let m_world_position_x = message
        .m_car_motion_data
        .iter()
        .map(|c| c.m_world_position_x)
        .collect::<Vec<f32>>();

    let m_world_position_y = message
        .m_car_motion_data
        .iter()
        .map(|c| c.m_world_position_y)
        .collect::<Vec<f32>>();

    let m_world_position_z = message
        .m_car_motion_data
        .iter()
        .map(|c| c.m_world_position_z)
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
    write_float_column(&mut row_group_writer, m_world_position_x);
    write_float_column(&mut row_group_writer, m_world_position_y);
    write_float_column(&mut row_group_writer, m_world_position_z);

    row_group_writer.close().unwrap();

    1
}
