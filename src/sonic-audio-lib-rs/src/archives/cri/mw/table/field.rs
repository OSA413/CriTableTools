use crate::archives::cri::mw::table::{field_flag::CriFieldFlag, field_value_wrapper::CriTableFieldRow};

pub struct CriTableField
{
    pub flag: CriFieldFlag,
    pub name: String,
    pub source: Vec<u8>,
    pub offset: usize, //Is this really needed?
    pub rows: Vec<CriTableFieldRow>,
}
