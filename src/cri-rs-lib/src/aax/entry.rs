use std::fmt::{self, Formatter};

use crate::aax::entry_flag::CriAaxEntryFlag;

pub struct CriAaxEntry {
    pub flag: CriAaxEntryFlag,
    pub data: Vec<u8>
}

impl fmt::Debug for CriAaxEntry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "CriAaxEntry {{ flag: {:?}, data_length: {} }}", self.flag, self.data.len())
    }
}