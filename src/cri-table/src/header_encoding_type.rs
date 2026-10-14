#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CriTableHeaderEncodingType {
    // Is it actually just an ASCII?
    // In case of needed proper Shift-JIS support, include this crate:
    // https://docs.rs/encoding_rs/0.2.4/encoding_rs/constant.SHIFT_JIS.html
    ShiftJis = 0,
    Utf8 = 1,
}

impl From<CriTableHeaderEncodingType> for usize {
    fn from(value: CriTableHeaderEncodingType) -> Self {
        match value {
            CriTableHeaderEncodingType::ShiftJis => 0,
            CriTableHeaderEncodingType::Utf8 => 1,
        }
    }
}

impl From<usize> for CriTableHeaderEncodingType {
    fn from(value: usize) -> Self {
        match value {
            0 => CriTableHeaderEncodingType::ShiftJis,
            1 => CriTableHeaderEncodingType::Utf8,
            value => panic!("Unknown CriTableHeaderEncodingType {value}"),
        }
    }
}
