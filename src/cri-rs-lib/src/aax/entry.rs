use crate::aax::entry_flag::CriAaxEntryFlag;

pub struct CriAaxEntry {
    pub flag: CriAaxEntryFlag,
    pub data: Vec<u8>
}