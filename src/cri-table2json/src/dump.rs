use std::fs;

use common_binary::{error::CommonBinaryError};
use cri_table::table::{CriTable, TableJsonWrapper, UTF_SIGNATURE};

pub fn dump_file(
    path: &str,
) -> Result<TableJsonWrapper, CommonBinaryError> {
    let source=  fs::read(path).unwrap();
    let first_signature = [source[0], source[1], source[2], source[3]];
    let pointer = if first_signature == *b"CPK " {
        0x10
    } else {
        0
    };
    let table = CriTable::read(&source, Some(pointer), UTF_SIGNATURE)?;
    Ok(TableJsonWrapper::Table(table.to_json()))
}

pub fn print_file(path: &str) {
    let dump_result = dump_file(path).unwrap();
    let json = serde_json::to_string_pretty(&dump_result).unwrap();
    println!("{json}");
}
