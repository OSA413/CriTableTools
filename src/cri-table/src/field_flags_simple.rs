use crate::field_raw_flags::CriFieldRawFlags;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CriFieldFlagsSimple {
    // We assume that flag "name" is always true
    // and that flags "default_value" and "row_storage" are mutually exclusive
    RowStorage,
    Default,
}

impl CriFieldFlagsSimple {
    pub fn get_bits(&self) -> u8 {
        (match self {
            CriFieldFlagsSimple::Default => CriFieldRawFlags::DefaultValue.bits(),
            CriFieldFlagsSimple::RowStorage => CriFieldRawFlags::RowStorage.bits(),
        })
        + CriFieldRawFlags::Name.bits()
    }
}