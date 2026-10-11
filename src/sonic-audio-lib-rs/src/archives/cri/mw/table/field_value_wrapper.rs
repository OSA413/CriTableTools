use common_binary::error::CommonBinaryError;
use serde::Serialize;

use crate::archives::cri::mw::table::{field_type::CriFieldType, field_value::CriFieldValue};

// Please note that a field defines the type of the value, not the value itself (?)
// This is just a convinient wrapper
// A field should contain values of the same type in a row
#[derive(Serialize)]
pub struct CriTableFieldRow {
    #[serde(skip)]
    pub ttype: CriFieldType,
    pub value: CriFieldValue,
}

impl CriTableFieldRow {
    pub fn to_u8(&self) -> Result<u8, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::Byte => match &self.value {
                CriFieldValue::Byte(value) => Ok(value.clone()),
                value => panic!("Expected value of Byte, got {value:?}"),
            },
            ttype => panic!("Expected type of Byte, got {ttype:?}"),
        }
    }

    pub fn to_u16(&self) -> Result<u16, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::UInt16 => match &self.value {
                CriFieldValue::UInt16(value) => Ok(value.clone()),
                value => panic!("Expected value of UInt16, got {value:?}"),
            },
            ttype => panic!("Expected type of Byte, got {ttype:?}"),
        }
    }

    pub fn to_u32(&self) -> Result<u32, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::UInt32 => match &self.value {
                CriFieldValue::UInt32(value) => Ok(value.clone()),
                value => panic!("Expected value of UInt32, got {value:?}"),
            },
            ttype => panic!("Expected type of Byte, got {ttype:?}"),
        }
    }

    pub fn to_u64(&self) -> Result<u64, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::UInt64 => match &self.value {
                CriFieldValue::UInt64(value) => Ok(value.clone()),
                value => panic!("Expected value of UInt64, got {value:?}"),
            },
            ttype => panic!("Expected type of Byte, got {ttype:?}"),
        }
    }

    pub fn to_string(&self) -> Result<String, CommonBinaryError> {
        match &self.ttype {
            CriFieldType::String => match &self.value {
                CriFieldValue::String(value) => Ok(value.clone()),
                value => panic!("Expected value of String, got {value:?}"),
            },
            ttype => panic!("Expected type of Byte, got {ttype:?}"),
        }
    }
}