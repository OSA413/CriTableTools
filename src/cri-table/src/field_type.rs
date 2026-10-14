use common_binary::{binary_reader, endianness::Endianness, error::CommonBinaryError};

use crate::{field_value::{CriFieldValue, TableOrData}, table::{CriTable, UTF_SIGNATURE}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CriFieldType {
    Byte = 0,
    SByte = 1,
    UInt16 = 2,
    Int16 = 3,
    UInt32 = 4,
    Int32 = 5,
    UInt64 = 6,
    Int64 = 7,
    Single = 8,
    Double = 9,
    String = 10,
    Data = 11,
    Guid = 12,
    // This is a "virtual" type to ease your (and my) suffering
    ProbablyTable,
}

impl CriFieldType {
    pub fn get_offset(&self) -> usize {
        match self {
            CriFieldType::Byte => 1,
            CriFieldType::SByte => 1,
            CriFieldType::Int16 => 2,
            CriFieldType::UInt16 => 2,
            CriFieldType::Int32 => 4,
            CriFieldType::UInt32 => 4,
            CriFieldType::Single => 4,
            CriFieldType::String => 4,
            CriFieldType::Int64 => 8,
            CriFieldType::UInt64 => 8,
            CriFieldType::Double => 8,
            CriFieldType::Data => 8,
            CriFieldType::ProbablyTable => 8,
            CriFieldType::Guid => 16,
        }
    }

    pub fn read_value(&self, source: &[u8], starting_pointer: usize, header_string_pool_position: usize, header_data_pool_position: usize, value_pointer: usize) -> Result<CriFieldValue, CommonBinaryError> {
        let result = match self {
            CriFieldType::Byte => CriFieldValue::Byte(binary_reader::u8::read(source, value_pointer, "")?),
            CriFieldType::SByte => CriFieldValue::SByte(binary_reader::u8::read(source, value_pointer, "")? as i8),
            CriFieldType::Int16 => CriFieldValue::Int16(binary_reader::u16::read(source, value_pointer, &Endianness::Big, "")? as i16),
            CriFieldType::UInt16 => CriFieldValue::UInt16(binary_reader::u16::read(source, value_pointer, &Endianness::Big, "")?),
            CriFieldType::Int32 => CriFieldValue::Int32(binary_reader::u32::read(source, value_pointer, &Endianness::Big, "")? as i32),
            CriFieldType::UInt32 => CriFieldValue::UInt32(binary_reader::u32::read(source, value_pointer, &Endianness::Big, "")?),
            CriFieldType::Single => CriFieldValue::Single(binary_reader::f32::read(source, value_pointer, &Endianness::Big, "")?),
            CriFieldType::Int64 => CriFieldValue::Int64(binary_reader::u64::read(source, value_pointer, &Endianness::Big, "")? as i64),
            CriFieldType::UInt64 => CriFieldValue::UInt64(binary_reader::u64::read(source, value_pointer, &Endianness::Big, "")?),
            CriFieldType::Double => CriFieldValue::Double(binary_reader::f64::read(source, value_pointer, &Endianness::Big, "")?),
            CriFieldType::String => {
                let string_pointer = binary_reader::u32::read(source, value_pointer, &Endianness::Big, "")? as usize;
                let string_pointer = starting_pointer + header_string_pool_position + string_pointer;
                let string = binary_reader::string::read(source, string_pointer, "")?.0;
                CriFieldValue::String(string)
            },
            CriFieldType::Data => {
                let position = binary_reader::u32::read(source, value_pointer, &Endianness::Big, "")? as usize;
                let length = binary_reader::u32::read(source, value_pointer + 0x04, &Endianness::Big, "")? as usize;
                let start = starting_pointer + position + header_data_pool_position;
                CriFieldValue::Data(source[start..start + length].to_vec())
            },
            CriFieldType::ProbablyTable => {
                let position = binary_reader::u32::read(source, value_pointer, &Endianness::Big, "").unwrap() as usize;
                let length = binary_reader::u32::read(source, value_pointer + 0x4, &Endianness::Big, "").unwrap() as usize;
                let start = starting_pointer + header_data_pool_position + position;

                let table_or_data = if CriTable::is_table(source, Some(start), UTF_SIGNATURE) {
                    TableOrData::Table(
                        CriTable::read(source, Some(start), UTF_SIGNATURE).unwrap()
                    )
                } else {
                    TableOrData::Data(source[start..start + length].to_vec())
                };

                CriFieldValue::ProbablyTable(table_or_data)
            },
            CriFieldType::Guid => todo!("guid"),
        };
        Ok(result)
    }

    pub fn get_null_value(&self) -> CriFieldValue {
        match self {
            CriFieldType::Byte => CriFieldValue::Byte(0),
            CriFieldType::SByte => CriFieldValue::SByte(0),
            CriFieldType::UInt16 => CriFieldValue::UInt16(0),
            CriFieldType::Int16 => CriFieldValue::Int16(0),
            CriFieldType::UInt32 => CriFieldValue::UInt32(0),
            CriFieldType::Int32 => CriFieldValue::Int32(0),
            CriFieldType::UInt64 => CriFieldValue::UInt64(0),
            CriFieldType::Int64 => CriFieldValue::Int64(0),
            CriFieldType::Single => CriFieldValue::Single(0.0),
            CriFieldType::Double => CriFieldValue::Double(0.0),
            CriFieldType::String => CriFieldValue::String(String::new()),
            CriFieldType::Data => CriFieldValue::Data(vec![]),
            CriFieldType::ProbablyTable => CriFieldValue::ProbablyTable(TableOrData::Data(vec![])),
            CriFieldType::Guid => todo!("guid"),
        }
    }
}