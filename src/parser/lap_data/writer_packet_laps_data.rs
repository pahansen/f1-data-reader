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
        REQUIRED INT32 lap_index;
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
    let len_lap = message.m_lap_data.len();
    let mut is_player_car_vec: Vec<bool> = vec![false; len_lap];
    if usize::from(packet_header.m_player_car_index) < len_lap {
        is_player_car_vec[usize::from(packet_header.m_player_car_index)] = true;
    }
    let mut is_secondary_player_car_vec: Vec<bool> = vec![false; len_lap];
    if usize::from(packet_header.m_secondary_player_car_index) < len_lap {
        is_secondary_player_car_vec[usize::from(packet_header.m_secondary_player_car_index)] = true;
    }
    let lap_index: Vec<i32> = (0..22).collect();
    let mut m_last_lap_time_in_ms_vec = Vec::new();
    let mut m_current_lap_time_in_ms_vec = Vec::new();
    let mut m_sector1_time_in_ms_vec = Vec::new();
    let mut m_sector2_time_in_ms_vec = Vec::new();
    let mut m_lap_distance_vec = Vec::new();
    let mut m_current_lap_num_vec = Vec::new();

    for lap in message.m_lap_data {
        m_last_lap_time_in_ms_vec.push(i64::from(lap.m_last_lap_time_in_ms));
        m_current_lap_time_in_ms_vec.push(i64::from(lap.m_current_lap_time_in_ms));
        m_sector1_time_in_ms_vec.push(i32::from(lap.m_sector1_time_in_ms));
        m_sector2_time_in_ms_vec.push(i32::from(lap.m_sector2_time_in_ms));
        m_lap_distance_vec.push(lap.m_lap_distance);
        m_current_lap_num_vec.push(i32::from(lap.m_current_lap_num));
    }

    let mut row_group_writer = writer.next_row_group().unwrap();
    write_int32_column(
        &mut row_group_writer,
        vec![i32::from(packet_header.m_packet_format); len_lap],
    );
    write_u64_as_bytearray_column(
        &mut row_group_writer,
        vec![packet_header.m_session_uid; len_lap],
    );
    write_float_column(
        &mut row_group_writer,
        vec![packet_header.m_session_time; len_lap],
    );
    write_bool_column(&mut row_group_writer, is_player_car_vec);
    write_bool_column(&mut row_group_writer, is_secondary_player_car_vec);
    write_int32_column(&mut row_group_writer, lap_index);
    write_int64_column(&mut row_group_writer, m_last_lap_time_in_ms_vec);
    write_int64_column(&mut row_group_writer, m_current_lap_time_in_ms_vec);
    write_int32_column(&mut row_group_writer, m_sector1_time_in_ms_vec);
    write_int32_column(&mut row_group_writer, m_sector2_time_in_ms_vec);
    write_float_column(&mut row_group_writer, m_lap_distance_vec);
    write_int32_column(&mut row_group_writer, m_current_lap_num_vec);

    row_group_writer.close().unwrap();

    1
}
