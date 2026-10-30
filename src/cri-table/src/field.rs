use common_binary::error::{CommonBinaryError, PointerOutOfBoundsDetails};
use serde::Serialize;

use crate::{field_flags_simple::CriFieldFlagsSimple, field_type::CriFieldType, field_value::{CriFieldValue, TableOrData}, table::CriTable};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CriTableField
{
    pub name: String,
    #[serde(rename = "type")]
    pub ttype: CriFieldType,
    #[serde(skip)]
    pub rows: Vec<CriFieldValue>,
    pub default_value: Option<CriFieldValue>,
}

// TODO properly cover all cases with matches
impl CriTableField {
    pub fn actual_flags(&self) -> CriFieldFlagsSimple {
        if self.rows.is_empty() {
            return CriFieldFlagsSimple::Null;
        }
        match &self.default_value {
            Some(_) => CriFieldFlagsSimple::Default,
            None => CriFieldFlagsSimple::RowStorage,
        }
    }

    pub fn to_bool(&self, row_index: usize) -> Result<bool, CommonBinaryError> {
        Ok(self.to_u8(row_index).unwrap() != 0)
    }

    pub fn to_u8(&self, row_index: usize) -> Result<u8, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::Byte => match &self.rows[row_index] {
                CriFieldValue::Byte(value) => Ok(*value),
                CriFieldValue::Default => match self.default_value.as_ref().unwrap() {
                    CriFieldValue::Byte(value) => Ok(*value),
                    _ => panic!("dsadsa"),
                },
                value => panic!("Expected value of Byte, got {value:?}"),
            },
            ttype => panic!("Expected type of Byte, got {ttype:?}"),
        }
    }

    pub fn to_i8(&self, row_index: usize) -> Result<i8, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::SByte => match &self.rows[row_index] {
                CriFieldValue::SByte(value) => Ok(*value),
                CriFieldValue::Default => match self.default_value.as_ref().unwrap() {
                    CriFieldValue::SByte(value) => Ok(*value),
                    _ => panic!("dsadsa"),
                },
                value => panic!("Expected value of SByte, got {value:?}"),
            },
            ttype => panic!("Expected type of SByte, got {ttype:?}"),
        }
    }

    pub fn to_u16(&self, row_index: usize) -> Result<u16, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::UInt16 => match &self.rows[row_index] {
                CriFieldValue::UInt16(value) => Ok(*value),
                CriFieldValue::Default => match self.default_value.as_ref().unwrap() {
                    CriFieldValue::UInt16(value) => Ok(*value),
                    _ => panic!("dsadsa"),
                },
                value => panic!("Expected value of UInt16, got {value:?}"),
            },
            ttype => panic!("Expected type of UInt16, got {ttype:?}"),
        }
    }

    pub fn to_i16(&self, row_index: usize) -> Result<i16, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::Int16 => match &self.rows[row_index] {
                CriFieldValue::Int16(value) => Ok(*value),
                CriFieldValue::Default => match self.default_value.as_ref().unwrap() {
                    CriFieldValue::Int16(value) => Ok(*value),
                    _ => panic!("dsadsa"),
                },
                value => panic!("Expected value of Int16, got {value:?}"),
            },
            ttype => panic!("Expected type of Int16, got {ttype:?}"),
        }
    }

    pub fn to_u32(&self, row_index: usize) -> Result<u32, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::UInt32 => match &self.rows.get(row_index) {
                Some(row) => match row {
                    CriFieldValue::UInt32(value) => Ok(*value),
                    CriFieldValue::Default => match self.default_value.as_ref().unwrap() {
                        CriFieldValue::UInt32(value) => Ok(*value),
                        _ => panic!("dsadsa"),
                    },
                    value => panic!("Expected value of UInt32, got {value:?}"),
                },
                None => Err(CommonBinaryError::PointerOutOfBounds(PointerOutOfBoundsDetails {
                    pointer: row_index,
                    source_len: self.rows.len(),
                    when: "tryint to access row of field for u32".to_string(),
                }))
            },
            ttype => panic!("Expected type of UInt32, got {ttype:?}"),
        }
    }

    pub fn to_f32(&self, row_index: usize) -> Result<f32, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::Single => match &self.rows.get(row_index) {
                Some(row) => match row {
                    CriFieldValue::Single(value) => Ok(*value),
                    value => panic!("Expected value of Single, got {value:?}"),
                },
                None => Err(CommonBinaryError::PointerOutOfBounds(PointerOutOfBoundsDetails {
                    pointer: row_index,
                    source_len: self.rows.len(),
                    when: "tryint to access row of field for f32".to_string(),
                }))
            },
            ttype => panic!("Expected type of Single, got {ttype:?}"),
        }
    }

    pub fn to_u64(&self, row_index: usize) -> Result<u64, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::UInt64 => match &self.rows[row_index] {
                CriFieldValue::UInt64(value) => Ok(*value),
                value => panic!("Expected value of UInt64, got {value:?}"),
            },
            ttype => panic!("Expected type of UInt64, got {ttype:?}"),
        }
    }

    pub fn to_string(&self, row_index: usize) -> Result<String, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::String => match &self.rows.get(row_index) {
                Some(row) => match row {
                    CriFieldValue::String(value) => Ok(value.clone()),
                    CriFieldValue::Default => match self.default_value.as_ref().unwrap() {
                        CriFieldValue::String(value) => Ok(value.clone()),
                        _ => panic!("dsadsa"),
                    },
                    value => panic!("Expected value of String, got {value:?}"),
                },
                None => Err(CommonBinaryError::PointerOutOfBounds(PointerOutOfBoundsDetails {
                    pointer: row_index,
                    source_len: self.rows.len(),
                    when: "trying to access row of field to for string".to_string(),
                }))
            },
            ttype => panic!("Expected type of String, got {ttype:?}"),
        }
    }

    pub fn to_data(&self, row_index: usize) -> Result<&Vec<u8>, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::Data => match &self.rows[row_index] {
                CriFieldValue::Data(value) => Ok(value),
                value => panic!("Expected value of Data, got {value:?}"),
            },
            CriFieldType::ProbablyTable => match &self.rows[row_index] {
                CriFieldValue::ProbablyTable(table_or_data) => match table_or_data {
                    TableOrData::Data(value) => Ok(value),
                    value => panic!("Expected value of Data, got {value:?}"),
                },
                ttype => panic!("Expected type of ProbablyTable, got {ttype:?}"),
            }
            ttype => panic!("Expected type of Data, got {ttype:?}"),
        }
    }

    pub fn to_table(&self, row_index: usize) -> Result<&CriTable, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::ProbablyTable => match &self.rows[row_index] {
                CriFieldValue::ProbablyTable(value) => match value {
                    TableOrData::Table(table) => Ok(table),
                    value_type => panic!("Expected value of Table, got {value_type:?}"),
                },
                value => panic!("Expected value of ProbablyTable, got {value:?}"),
            },
            ttype => panic!("Expected type of ProbablyTable, got {ttype:?}"),
        }
    }
}