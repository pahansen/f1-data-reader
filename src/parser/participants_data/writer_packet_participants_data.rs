use crate::parser::utils::util_column_writer::{
    write_bool_column, write_float_column, write_int32_column, write_u64_as_bytearray_column, write_string_as_bytearray_column
};
use crate::parser::utils::packet_header::PacketHeader;
use crate::parser::participants_data::struct_participants_data::PacketParticipantsData;
use binrw::BinRead;
use parquet::{
    file::{properties::WriterProperties, writer::SerializedFileWriter},
    schema::parser::parse_message_type,
};
use std::{
    fs::{self, File},
    path::Path,
    str,
    sync::Arc,
};

pub fn new(file_path: &Path) -> SerializedFileWriter<File> {
    let schema = "message schema {
        REQUIRED INT32 m_packet_format;
        REQUIRED BYTE_ARRAY m_session_uid;
        REQUIRED FLOAT m_session_time;
        REQUIRED BOOLEAN is_player_car;
        REQUIRED BOOLEAN is_secondary_player_car;
        REQUIRED INT32 participant_index;
        REQUIRED BYTE_ARRAY m_name;
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
    let message = PacketParticipantsData::read(&mut file).unwrap();
    let len_participants = message.m_participants.len();
    let mut is_player_car_vec: Vec<bool> = vec![false; len_participants];
    if usize::from(packet_header.m_player_car_index) < len_participants {
        is_player_car_vec[usize::from(packet_header.m_player_car_index)] = true;
    }
    let mut is_secondary_player_car_vec: Vec<bool> = vec![false; len_participants];
    if usize::from(packet_header.m_secondary_player_car_index) < len_participants {
        is_secondary_player_car_vec[usize::from(packet_header.m_player_car_index)] = true;
    }
    let participant_index: Vec<i32> = (0..22).collect();
    let mut m_name_vec: Vec<String> = Vec::new();

    for participant in message.m_participants {
        let buf = participant.m_name;
        let m_name = str::from_utf8(&buf).unwrap();
        m_name_vec.push(m_name.to_string());
    }

    let mut row_group_writer = writer.next_row_group().unwrap();
    write_int32_column(
        &mut row_group_writer,
        vec![i32::from(packet_header.m_packet_format); len_participants],
        None,
        None,
    );
    write_u64_as_bytearray_column(
        &mut row_group_writer,
        vec![packet_header.m_session_uid; len_participants],
        None,
        None,
    );
    write_float_column(
        &mut row_group_writer,
        vec![packet_header.m_session_time; len_participants],
    );
    write_bool_column(&mut row_group_writer, is_player_car_vec, None, None);
    write_bool_column(
        &mut row_group_writer,
        is_secondary_player_car_vec,
        None,
        None,
    );
    write_int32_column(&mut row_group_writer, participant_index, None, None);
    write_string_as_bytearray_column(&mut row_group_writer, m_name_vec, None, None);

    row_group_writer.close().unwrap();

    1
}
