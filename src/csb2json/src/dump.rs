use std::path::Path;

use common_binary::{error::CommonBinaryError};
use cri_rs_lib::{cpk::archive::CriCpkArchive, csb::archive::CriCsbArchive};
use cri_table::{field::CriTableField, field_type::CriFieldType, header_encoding_type::CriTableHeaderEncodingType, table::CriTable};
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonResult {
    pub csb: TableJson,
    pub cpk: Option<CpkJson>,
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

#[derive(Serialize)]
pub struct CpkJson {
    pub mode: String,
    pub comment: String,
    pub entries: Vec<CpkEntryJson>,
}

#[derive(Serialize)]
pub struct CpkEntryJson {
    pub id: u32,
    pub directory_name: String,
    pub name: String,
    pub table: TableJson,
}

pub fn dump_csb(
    csb_path: &Path,
) -> Result<JsonResult, CommonBinaryError> {
    let csb = CriCsbArchive::read(csb_path)?;
    Ok(JsonResult { csb: dump_table(&csb.table)?, cpk: csb.stream.map(|cpk| dump_cpk(cpk).unwrap()) })
}

fn dump_cpk(cpk: CriCpkArchive) -> Result<CpkJson, CommonBinaryError> {
    let entries = cpk.entries.iter().map(|entry| CpkEntryJson {
        id: entry.id,
        directory_name: entry.directory_name.clone(),
        name: entry.name.clone(),
        table: dump_table(&entry.table).unwrap(),
    }).collect();

    Ok(CpkJson {
        mode: format!("{:?}", cpk.mode),
        comment: cpk.comment,
        entries,
    })
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
            let json_value = serde_json::to_value(&field.rows[row_index]).unwrap();
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

pub fn dump_file(csb_path: &str) {
    let dump_result = dump_csb(Path::new(csb_path)).unwrap();
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
    }
}

