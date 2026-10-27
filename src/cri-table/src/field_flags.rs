use crate::field_raw_flags::CriFieldRawFlags;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CriFieldFlags {
    pub name: bool,
    pub default_value: bool,
    pub row_storage: bool,
}

impl CriFieldFlags {
    pub fn get_bits(&self) -> u8 {
        self.default_value as u8 * CriFieldRawFlags::DefaultValue.bits()
        + self.row_storage as u8 * CriFieldRawFlags::RowStorage.bits()
        + self.name as u8 * CriFieldRawFlags::Name.bits()
    }
}