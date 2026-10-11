use sonic_audio_lib_rs::archives::cri::{cpk::{entry::CriCpkEntry, mode::CriCpkMode}, mw::table::field::CriTableField};

// TODO: make "inheritance" from the "CriTable"
// Or maybe use sonic_audio_lib_rs..... I simplified it a bit
pub struct Cpk {
    pub fields: Vec<CriTableField>,
    pub mode: CriCpkMode,
    pub comment: String, 
    pub entries: Vec<CriCpkEntry>,
}