use std::{env, fs};

use cri_table::table::{CriTable, UTF_SIGNATURE};
use similar::{ChangeTag, TextDiff};

use cri_table::{field::CriTableField, field_type::CriFieldType, field_value::{CriFieldValue, TableOrData}, header_encoding_type::CriTableHeaderEncodingType};
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

fn dump_table(table: &CriTable) -> TableJson {
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
                    TableOrData::Table(cri_table) => serde_json::to_value(TableJsonWrapper::Table(dump_table(cri_table))),
                    value => serde_json::to_value(value),
                },
                value => serde_json::to_value(value),
            }.unwrap();
            values.insert(field.name.clone(), json_value);
        }
        rows.push(values);
    }

    TableJson {
        name: table.name.clone(),
        encoding,
        fields,
        rows,
    }
}

pub fn print_table(table: &CriTable) -> String {
    let dump_result = TableJsonWrapper::Table(dump_table(&table));
    serde_json::to_string_pretty(&dump_result).unwrap()
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

fn main() {
    let mut args = env::args().skip(1);
    let file_path = args.next().unwrap();
    let file_content = fs::read(&file_path).unwrap();
    let original_table = CriTable::read(&file_content, None, UTF_SIGNATURE).unwrap();
    let recreated_table = CriTable::read(&original_table.write().unwrap(), None, UTF_SIGNATURE).unwrap();

    let diff = TextDiff::from_lines(
        print_table(&original_table),
        print_table(&recreated_table),
    );

    let mut result = Vec::with_capacity(diff.iter_all_changes().count());

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => result.push(format!("{}{}{}", "-", change.old_index().unwrap(), change)),
            ChangeTag::Insert => result.push(format!("{}{}{}", "+", change.new_index().unwrap(), change)),
            ChangeTag::Equal => (),
        };
    }

    fs::write(format!("{}.diff", file_path), result.join("")).unwrap();
}