use std::collections::HashMap;

use common_utils::get_sha256;
use serde::Serialize;

use crate::{field_type::CriFieldType, table::CriTable};

#[derive(Clone, Serialize, Debug, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum CriFieldValue {
    // A helper type for the default_value writing
    Default,
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
        // Maybe not skip?
        #[serde(skip)]
        CriTable,
    ),
    Data(
        #[serde(skip)]
        Vec<u8>
    ),
}

impl CriFieldValue {
    fn to_bytes_common(
        &self,
    ) -> Vec<u8> {
        match self {
            CriFieldValue::Default => vec![],
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
            CriFieldValue::Guid(_) => panic!("guid is not implemented"),
            _ => panic!("This method is not supported for String, Data and ProbablyTable"),
        }
    }

    pub fn to_bytes_for_field_info(
        &self,
        ttype: &CriFieldType,
        string_pool: &HashMap<String, u32>,
        data_pool: &HashMap<String, (u32, Vec<u8>)>,
    ) -> Vec<u8> {
        match self {
            CriFieldValue::String(value) => string_pool.get(value).unwrap().to_be_bytes().to_vec(),
            CriFieldValue::ProbablyTable(table_or_data) => {
                let mut data = match table_or_data {
                    TableOrData::Data(value) => value.clone(),
                    TableOrData::Table(table) => table.write().unwrap(),
                };

                let original_length = data.len();

                for _ in data.len()..CriTable::length_nice(data.len()) {
                    data.push(0);
                }

                [
                    data_pool.get(&get_sha256(&data)).unwrap().0.to_be_bytes().to_vec(),
                    (original_length as u32).to_be_bytes().to_vec(),
                ].concat()
            },
            CriFieldValue::Data(value) => {
                let mut data = value.clone();
                let original_length = data.len();

                for _ in data.len()..CriTable::length_nice(data.len()) {
                    data.push(0);
                }

                [
                    data_pool.get(&get_sha256(value)).unwrap().0.to_be_bytes().to_vec(),
                    (original_length as u32).to_be_bytes().to_vec(),
                ].concat()
            },
            _ => self.to_bytes_common(),
        }
    }

    
    pub fn to_bytes_for_data(&self, ttype: &CriFieldType) -> Vec<u8> {
        match self {
            CriFieldValue::String(value) => value.bytes().chain(vec![0]).collect(),
            CriFieldValue::Data(value) => {
                let mut data = value.clone();
                data
            },
            CriFieldValue::ProbablyTable(table_or_data) => {
                let mut data = match table_or_data {
                    TableOrData::Data(value) => value.clone(),
                    TableOrData::Table(table) => table.write().unwrap(),
                };

                data
            }
            _ => self.to_bytes_common(),
        }
    }
}