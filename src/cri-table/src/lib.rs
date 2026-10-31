pub mod field_type;
pub mod field_flags;
pub mod field_flags_simple;
pub mod field_raw_flags;
pub mod field_value;
pub mod field;
pub mod encoding_type;
pub mod table_write_helpers;
pub mod table;
pub mod table_generator;
pub mod field_dto;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");
