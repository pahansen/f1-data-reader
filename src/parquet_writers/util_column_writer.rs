use parquet::{
    data_type::{FloatType, Int32Type}
};
use std::fs::File;

pub fn write_float_column(row_group_writer: &mut parquet::file::writer::SerializedRowGroupWriter<File>, column_data_vec: Vec<f32>) {
    if let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
        col_writer
        .typed::<FloatType>()
        .write_batch(&column_data_vec, None, None)
        .unwrap();
        col_writer.close().unwrap()
    }
}

pub fn write_int32_column(row_group_writer: &mut parquet::file::writer::SerializedRowGroupWriter<File>, column_data_vec: Vec<i32>) {
    if let Some(mut col_writer) = row_group_writer.next_column().unwrap() {
        col_writer
        .typed::<Int32Type>()
        .write_batch(&column_data_vec, None, None)
        .unwrap();
        col_writer.close().unwrap()
    }
}