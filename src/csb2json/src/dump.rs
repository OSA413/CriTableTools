use common_binary::{error::CommonBinaryError};
use cri_rs_lib::cpk::archive::CriCpkArchive;
use cri_table::{field::CriTableField, field_type::CriFieldType, header_encoding_type::CriTableHeaderEncodingType, table::{CriTable, UTF_SIGNATURE}};
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

pub fn dump(
    csb_source: &[u8],
    cpk_source: Option<&[u8]>,
) -> Result<JsonResult, CommonBinaryError> {
    let csb = dump_table_from_source(csb_source, Some(0))?;
    let cpk = match cpk_source {
        Some(source) => Some(dump_cpk(source)?),
        None => None,
    };
    Ok(JsonResult { csb, cpk })
}

fn dump_cpk(source: &[u8]) -> Result<CpkJson, CommonBinaryError> {
    let archive = CriCpkArchive::read(source, 0)?;

    let entries = archive.entries.iter().map(|entry| CpkEntryJson {
        id: entry.id,
        directory_name: entry.directory_name.clone(),
        name: entry.name.clone(),
        table: dump_table(&entry.table).unwrap(),
    }).collect();

    Ok(CpkJson {
        mode: format!("{:?}", archive.mode),
        comment: archive.comment,
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

fn dump_table_from_source(
    source: &[u8],
    pointer: Option<usize>,
) -> Result<TableJson, CommonBinaryError> {
    dump_table(
        &CriTable::read(source, pointer, UTF_SIGNATURE)?.table
    )
}

// Maybe this can be simplified?
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

