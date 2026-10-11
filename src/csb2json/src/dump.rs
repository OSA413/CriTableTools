use common_binary::{error::CommonBinaryError};
use serde::Serialize;
use sonic_audio_lib_rs::archives::cri::{
    cpk::archive::CriCpkArchive, mw::table::{
        common::{UTF_SIGNATURE, is_table_data}, field::CriTableField, field_type::CriFieldType, table::CriTable, header_encoding_type::CriTableHeaderEncodingType,
    },
};

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
    pub table: Option<TableJson>,
}

pub fn dump(
    csb_source: &[u8],
    cpk_source: Option<&[u8]>,
) -> Result<JsonResult, CommonBinaryError> {
    let csb = dump_table(csb_source, Some(0))?;
    let cpk = match cpk_source {
        Some(source) => Some(dump_cpk(source)?),
        None => None,
    };
    Ok(JsonResult { csb, cpk })
}

fn dump_cpk(source: &[u8]) -> Result<CpkJson, CommonBinaryError> {
    let archive = CriCpkArchive::read(source, 0)?;

    let mut entries = Vec::with_capacity(archive.entries.len());
    for entry in &archive.entries {
        let mut table: Option<TableJson> = None;
        if is_table_data(&entry.value, Some(0)) {
            table = match dump_table(&entry.value, Some(0)) {
                Ok(table) => Some(table),
                Err(_) => None,
            }
        }

        entries.push(CpkEntryJson {
            id: entry.id,
            directory_name: entry.directory_name.clone(),
            name: entry.name.clone(),
            table,
        });
    }

    Ok(CpkJson {
        mode: format!("{:?}", archive.mode),
        comment: archive.comment,
        entries,
    })
}

fn dump_table(
    source: &[u8],
    pointer: Option<usize>,
) -> Result<TableJson, CommonBinaryError> {
    let reader = CriTable::read(source, pointer, UTF_SIGNATURE)?;

    let encoding = match reader.table.encoding {
        CriTableHeaderEncodingType::ShiftJis => "shift-jis",
        CriTableHeaderEncodingType::Utf8 => "utf-8",
    };

    let fields = reader
        .table
        .fields
        .iter()
        .map(|field| FieldJson {
            name: field.name.clone(),
            ttype: field_type_name(&field),
        })
        .collect();

    let mut rows = Vec::with_capacity(reader.row_count());
    for row_index in 0..reader.row_count() {
        let mut values = serde_json::Map::new();
        for field in &reader.table.fields {
            let json_value = serde_json::to_value(&field.rows[row_index]).unwrap();
            values.insert(field.name.clone(), json_value);
        }
        rows.push(values);
    }

    Ok(TableJson {
        name: reader.table.name,
        encoding,
        fields,
        rows,
    })
}

// Maybe this can be simplified?
fn field_type_name(field: &CriTableField) -> &'static str {
    match field.ttype {
        CriFieldType::Byte => "byte",
        CriFieldType::SByte => "sbyte",
        CriFieldType::UInt16 => "uint16",
        CriFieldType::Int16 => "int16",
        CriFieldType::UInt32 => "uint32",
        CriFieldType::Int32 => "int32",
        CriFieldType::UInt64 => "uint64",
        CriFieldType::Int64 => "int64",
        CriFieldType::Single => "single",
        CriFieldType::Double => "double",
        CriFieldType::String => "string",
        CriFieldType::Data => "data",
        CriFieldType::Guid => "guid",
    }
}

