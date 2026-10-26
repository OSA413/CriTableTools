use sha2::{Sha256, Digest};

pub mod field_type;
pub mod field_flags;
pub mod field_raw_flags;
pub mod field_value;
pub mod field;
pub mod header_encoding_type;
pub mod table_write_helpers;
pub mod table;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");

// TODO move this to common_utils or something
pub fn get_sha256(data: impl AsRef<[u8]>) -> String {
    Sha256::digest(data).iter().map(|x| format!("{x:02x}")).collect()
}