use crate::parser::car_telemetry_data::struct_car_telemetry_data::PacketCarTelemetryData;
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
    let number_of_cars = message.m_car_telemetry_data.len();
    let mut is_player_car_vec: Vec<bool> = vec![false; number_of_cars];
    if usize::from(packet_header.m_player_car_index) < number_of_cars {
        is_player_car_vec[usize::from(packet_header.m_player_car_index)] = true;
    }
    let mut is_secondary_player_car_vec: Vec<bool> = vec![false; number_of_cars];
    if usize::from(packet_header.m_secondary_player_car_index) < number_of_cars {
        is_secondary_player_car_vec[usize::from(packet_header.m_secondary_player_car_index)] = true;
    }
    let car_index: Vec<i32> = (0..22).collect();

    let m_speed = message
        .m_car_telemetry_data
        .iter()
        .map(|c| i32::from(c.m_speed))
        .collect::<Vec<i32>>();
    let m_throttle = message
        .m_car_telemetry_data
        .iter()
        .map(|c| c.m_throttle)
        .collect::<Vec<f32>>();
    let m_steer = message
        .m_car_telemetry_data
        .iter()
        .map(|c| c.m_steer)
        .collect::<Vec<f32>>();
    let m_brake = message
        .m_car_telemetry_data
        .iter()
        .map(|c| c.m_brake)
        .collect::<Vec<f32>>();
    let m_clutch = message
        .m_car_telemetry_data
        .iter()
        .map(|c| i32::from(c.m_clutch))
        .collect::<Vec<i32>>();
    let m_gear = message
        .m_car_telemetry_data
        .iter()
        .map(|c| i32::from(c.m_gear))
        .collect::<Vec<i32>>();
    let m_engine_rpm = message
        .m_car_telemetry_data
        .iter()
        .map(|c| i32::from(c.m_engine_rpm))
        .collect::<Vec<i32>>();
    let m_drs = message
        .m_car_telemetry_data
        .iter()
        .map(|c| i32::from(c.m_drs))
        .collect::<Vec<i32>>();
    let m_rev_lights_percent = message
        .m_car_telemetry_data
        .iter()
        .map(|c| i32::from(c.m_drs))
        .collect::<Vec<i32>>();
    let m_rev_lights_bit_value = message
        .m_car_telemetry_data
        .iter()
        .map(|c| i32::from(c.m_rev_lights_bit_value))
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
    write_int32_column(&mut row_group_writer, m_speed);
    write_float_column(&mut row_group_writer, m_throttle);
    write_float_column(&mut row_group_writer, m_steer);
    write_float_column(&mut row_group_writer, m_brake);
    write_int32_column(&mut row_group_writer, m_clutch);
    write_int32_column(&mut row_group_writer, m_gear);
    write_int32_column(&mut row_group_writer, m_engine_rpm);
    write_int32_column(&mut row_group_writer, m_drs);
    write_int32_column(&mut row_group_writer, m_rev_lights_percent);
    write_int32_column(&mut row_group_writer, m_rev_lights_bit_value);

    row_group_writer.close().unwrap();
    1
}
