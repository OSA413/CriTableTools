use std::fs;

use common_binary::{error::CommonBinaryError};
use cri_table::{field::CriTableField, field_type::CriFieldType, field_value::{CriFieldValue, TableOrData}, header_encoding_type::CriTableHeaderEncodingType, table::{CriTable, UTF_SIGNATURE}};
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type", content = "value")]
pub enum TableJsonWrapper {
    Table(TableJson),
}

#[derive(Serialize)]
pub struct TableJson {
    pub name: String,
    pub encoding: &'static str,
    pub fields: Vec<FieldJson>,
    pub rows: Vec<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Serialize)]
pub struct FieldJson {
    pub name: String,
    #[serde(rename = "type")]
    pub ttype: &'static str,
}

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
    Ok(TableJsonWrapper::Table(dump_table(&table)?))
}

fn dump_table(table: &CriTable) -> Result<TableJson, CommonBinaryError> {
    let encoding = match table.encoding {
        CriTableHeaderEncodingType::ShiftJis => "shift-jis",
        CriTableHeaderEncodingType::Utf8 => "utf-8",
    };

    let fields = table
        .fields
        .iter()
        .map(|field| FieldJson {
            name: field.name.clone(),
            ttype: field_type_name(&field),
        })
        .collect();

    let mut rows = Vec::with_capacity(table.row_count());
    for row_index in 0..table.row_count() {
        let mut values = serde_json::Map::new();
        for field in &table.fields {
            let json_value = match &field.rows[row_index] {
                CriFieldValue::ProbablyTable(table_or_data) => match table_or_data {
                    TableOrData::Table(cri_table) => serde_json::to_value(TableJsonWrapper::Table(dump_table(cri_table).unwrap())),
                    value => serde_json::to_value(value),
                },
                value => serde_json::to_value(value),
            }.unwrap();
            values.insert(field.name.clone(), json_value);
        }
        rows.push(values);
    }

    Ok(TableJson {
        name: table.name.clone(),
        encoding,
        fields,
        rows,
    })
}

pub fn print_file(path: &str) {
    let dump_result = dump_file(path).unwrap();
    let json = serde_json::to_string_pretty(&dump_result).unwrap();
    println!("{json}");
}

fn field_type_name(field: &CriTableField) -> &'static str {
    match field.ttype {
        CriFieldType::Byte => "Byte",
        CriFieldType::SByte => "SByte",
        CriFieldType::UInt16 => "UInt16",
        CriFieldType::Int16 => "Int16",
        CriFieldType::UInt32 => "UInt32",
        CriFieldType::Int32 => "Int32",
        CriFieldType::UInt64 => "UInt64",
        CriFieldType::Int64 => "Int64",
        CriFieldType::Single => "Single",
        CriFieldType::Double => "Double",
        CriFieldType::String => "String",
        CriFieldType::Data => "Data",
        CriFieldType::Guid => "Guid",
        CriFieldType::ProbablyTable => "Data",
    }
}

