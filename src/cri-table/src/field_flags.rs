use crate::field_raw_flags::CriFieldRawFlags;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CriFieldFlags {
    pub name: bool,
    pub default_value: bool,
    pub row_storage: bool,
}

impl CriFieldFlags {
    pub fn get_bits(&self) -> u8 {
        (match self.default_value { true => 1, false => 0 }) * CriFieldRawFlags::DefaultValue.bits()
        + (match self.row_storage { true => 1, false => 0 }) * CriFieldRawFlags::RowStorage.bits()
        + (match self.name { true => 1, false => 0 }) * CriFieldRawFlags::Name.bits()
    }
}