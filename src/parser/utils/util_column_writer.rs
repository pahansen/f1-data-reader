use parquet::data_type::{BoolType, ByteArray, ByteArrayType, FloatType, Int32Type, Int64Type};
use std::fs::File;

pub fn write_float_column(
    row_group_writer: &mut parquet::file::writer::SerializedRowGroupWriter<File>,
    column_data_vec: Vec<f32>,
) {
    if let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
        col_writer
            .typed::<FloatType>()
            .write_batch(&column_data_vec, None, None)
            .unwrap();
        col_writer.close().unwrap()
    }
}

pub fn write_int32_column(
    row_group_writer: &mut parquet::file::writer::SerializedRowGroupWriter<File>,
    column_data_vec: Vec<i32>,
) {
    if let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
        col_writer
            .typed::<Int32Type>()
            .write_batch(&column_data_vec, None, None)
            .unwrap();
        col_writer.close().unwrap()
    }
}

pub fn write_int64_column(
    row_group_writer: &mut parquet::file::writer::SerializedRowGroupWriter<File>,
    column_data_vec: Vec<i64>,
) {
    if let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
        col_writer
            .typed::<Int64Type>()
            .write_batch(&column_data_vec, None, None)
            .unwrap();
        col_writer.close().unwrap()
    }
}

pub fn write_u64_as_bytearray_column(
    row_group_writer: &mut parquet::file::writer::SerializedRowGroupWriter<File>,
    column_data_vec: Vec<u64>,
) {
    let mut u64_byte_array_vec = Vec::new();
    for value in column_data_vec {
        let u64_string = value.to_string();
        let u64_bytes = u64_string.as_bytes();
        let u64_byte_array = ByteArray::from(u64_bytes.to_vec());
        u64_byte_array_vec.push(u64_byte_array);
    }
    if let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&u64_byte_array_vec, None, None)
            .unwrap();
        col_writer.close().unwrap()
    }
}

pub fn write_string_as_bytearray_column(
    row_group_writer: &mut parquet::file::writer::SerializedRowGroupWriter<File>,
    column_data_vec: Vec<String>,
) {
    let mut string_byte_array_vec = Vec::new();
    for value in column_data_vec {
        let string_bytes = value.as_bytes();
        let string_byte_array = ByteArray::from(string_bytes.to_vec());
        string_byte_array_vec.push(string_byte_array);
    }

    if let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
        col_writer
            .typed::<ByteArrayType>()
            .write_batch(&string_byte_array_vec, None, None)
            .unwrap();
        col_writer.close().unwrap()
    }
}

pub fn write_bool_column(
    row_group_writer: &mut parquet::file::writer::SerializedRowGroupWriter<File>,
    column_data_vec: Vec<bool>,
) {
    if let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
        col_writer
            .typed::<BoolType>()
            .write_batch(&column_data_vec, None, None)
            .unwrap();
        col_writer.close().unwrap()
    }
}
