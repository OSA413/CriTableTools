use serde::Serialize;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum CriTableEncodingType {
    // Is it actually just an ASCII?
    // In case of needed proper Shift-JIS support, include this crate:
    // https://docs.rs/encoding_rs/0.2.4/encoding_rs/constant.SHIFT_JIS.html
    ShiftJis = 0,
    Utf8 = 1,
}

impl From<CriTableEncodingType> for usize {
    fn from(value: CriTableEncodingType) -> Self {
        match value {
            CriTableEncodingType::ShiftJis => 0,
            CriTableEncodingType::Utf8 => 1,
        }
    }
}

impl From<usize> for CriTableEncodingType {
    fn from(value: usize) -> Self {
        match value {
            0 => CriTableEncodingType::ShiftJis,
            1 => CriTableEncodingType::Utf8,
            value => panic!("Unknown CriTableEncodingType {value}"),
        }
    }
}
