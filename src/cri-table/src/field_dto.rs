use crate::{field_type::CriFieldType, field_value::CriFieldValue};

pub struct CriFieldDto {
    pub ttype: CriFieldType,
    pub name: &'static str,
    pub default_value: Option<CriFieldValue>,
}

impl CriFieldDto {
    pub fn new(name: &'static str, ttype: CriFieldType, default_value: Option<CriFieldValue>) -> CriFieldDto {
        CriFieldDto {
            ttype,
            name,
            default_value,
        }
    }
}