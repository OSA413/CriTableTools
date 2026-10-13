use common_binary::error::CommonBinaryError;

use crate::{field_flags::CriFieldFlags, field_type::CriFieldType, field_value::CriFieldValue};

pub struct CriTableField
{ 
    pub flags: CriFieldFlags,
    pub ttype: CriFieldType,
    pub name: String,
    pub source: Vec<u8>,
    pub offset: usize, //Is this really needed?
    pub rows: Vec<CriFieldValue>,
}

impl CriTableField {
    pub fn to_u8(&self, row_index: usize) -> Result<u8, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::Byte => match &self.rows[row_index] {
                CriFieldValue::Byte(value) => Ok(value.clone()),
                value => panic!("Expected value of Byte, got {value:?}"),
            },
            ttype => panic!("Expected type of Byte, got {ttype:?}"),
        }
    }

    pub fn to_u16(&self, row_index: usize) -> Result<u16, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::UInt16 => match &self.rows[row_index] {
                CriFieldValue::UInt16(value) => Ok(value.clone()),
                value => panic!("Expected value of UInt16, got {value:?}"),
            },
            ttype => panic!("Expected type of UInt16, got {ttype:?}"),
        }
    }

    pub fn to_u32(&self, row_index: usize) -> Result<u32, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::UInt32 => match &self.rows[row_index] {
                CriFieldValue::UInt32(value) => Ok(value.clone()),
                value => panic!("Expected value of UInt32, got {value:?}"),
            },
            ttype => panic!("Expected type of UInt32, got {ttype:?}"),
        }
    }

    pub fn to_u64(&self, row_index: usize) -> Result<u64, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::UInt64 => match &self.rows[row_index] {
                CriFieldValue::UInt64(value) => Ok(value.clone()),
                value => panic!("Expected value of UInt64, got {value:?}"),
            },
            ttype => panic!("Expected type of UInt64, got {ttype:?}"),
        }
    }

    pub fn to_string(&self, row_index: usize) -> Result<String, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::String => match &self.rows[row_index] {
                CriFieldValue::String(value) => Ok(value.clone()),
                value => panic!("Expected value of String, got {value:?}"),
            },
            ttype => panic!("Expected type of String, got {ttype:?}"),
        }
    }

    pub fn to_data(&self, row_index: usize) -> Result<&Vec<u8>, CommonBinaryError> {
        // To box?
        match &self.ttype {
            CriFieldType::Data => match &self.rows[row_index] {
                CriFieldValue::Data(value) => Ok(value),
                value => panic!("Expected value of Data, got {value:?}"),
            },
            ttype => panic!("Expected type of Data, got {ttype:?}"),
        }
    }
}