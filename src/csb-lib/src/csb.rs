use sonic_audio_lib_rs::archives::cri::mw::table::field::CriTableField;

use crate::cpk::Cpk;

// TODO: make "inheritance" from the "CriTable"
// Or maybe use sonic_audio_lib_rs..... I simplified it a bit
pub struct Csb {
    fields: Vec<CriTableField>,
    stream: Option<Cpk>,
}