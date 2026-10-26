use std::collections::HashMap;

use serde::Serialize;

use crate::{field_type::CriFieldType, get_sha256, table::CriTable};

#[derive(Clone, Serialize, Debug, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum CriFieldValue {
    // A helper type for the default_value writing
    Default,
    Null,
    Byte(u8),
    SByte(i8),
    UInt16(u16),
    Int16(i16),
    UInt32(u32),
    Int32(i32),
    UInt64(u64),
    Int64(i64),
    Single(f32),
    Double(f64),
    String(String),
    Data(
        #[serde(skip)]
        Vec<u8>
    ),
    Guid(u128),
    // This is a "virtual" type to ease your (and my) suffering
    ProbablyTable(TableOrData),
}

#[derive(Clone, Serialize, Debug, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum TableOrData {
    Table(
        #[serde(skip)]
        CriTable
    ),
    Data(
        #[serde(skip)]
        Vec<u8>
    ),
}

impl CriFieldValue {
    pub fn to_bytes_for_field_info(
        &self,
        ttype: &CriFieldType,
        string_pool: &(&'static str, HashMap<String, u32>),
        data_pool: &([u8;0], HashMap<String, (u32, Vec<u8>)>),
    ) -> Vec<u8> {
        match self {
            CriFieldValue::Default => vec![],
            CriFieldValue::Null => ttype.get_null_value().to_bytes_for_field_info(ttype, string_pool, data_pool),
            CriFieldValue::Byte(value) => vec![*value],
            CriFieldValue::SByte(value) => vec![*value as u8],
            CriFieldValue::UInt16(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::Int16(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::UInt32(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::Int32(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::UInt64(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::Int64(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::Single(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::Double(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::String(value) => string_pool.1.get(value).unwrap().to_be_bytes().to_vec(),
            CriFieldValue::Data(value) => {
                let hash = get_sha256(value);
                [
                    data_pool.1.get(&hash).unwrap().0.to_be_bytes().to_vec(),
                    (value.len() as u32).to_be_bytes().to_vec(),
                ].concat()
            },
            CriFieldValue::Guid(_) => panic!("guid is not implemented"),
            CriFieldValue::ProbablyTable(table_or_data) => {
                let data = match table_or_data {
                    TableOrData::Data(value) => value.clone(),
                    TableOrData::Table(table) => table.write().unwrap()
                };

                [
                    data_pool.1.get(&get_sha256(&data)).unwrap().0.to_be_bytes().to_vec(),
                    (data.len() as u32).to_be_bytes().to_vec(),
                ].concat()
            }
        }
    }

    
    pub fn to_bytes_for_data(&self, ttype: &CriFieldType) -> Vec<u8> {
        match self {
            CriFieldValue::Default => vec![],
            CriFieldValue::Null => ttype.get_null_value().to_bytes_for_data(ttype),
            CriFieldValue::Byte(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::SByte(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::Int16(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::UInt16(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::Int32(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::UInt32(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::Single(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::Int64(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::UInt64(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::Double(value) => value.to_be_bytes().to_vec(),
            CriFieldValue::String(value) => value.bytes().chain(vec![0]).collect(),
            CriFieldValue::Data(value) => value.clone(),
            CriFieldValue::ProbablyTable(table_or_data) => match table_or_data {
                TableOrData::Data(value) => value.clone(),
                TableOrData::Table(table) => table.write().unwrap(),
            },
            CriFieldValue::Guid(_) => panic!("guid is not implemented"),
        }
    }
}