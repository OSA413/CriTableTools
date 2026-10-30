use std::fmt::{self, Formatter};

use crate::loop_flag::CriLoopFlag;

#[derive(Clone)]
pub struct CriAdpcmWiiEntry {
    pub sfreq: u32,
    pub nsmpl: u32,
    pub nch: u8,
    pub loop_flag: CriLoopFlag,
    pub data: Vec<u8>,
    pub header: Vec<u8>,
}

impl fmt::Debug for CriAdpcmWiiEntry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "CriAdpcmWiiEntry {{ flag: {:?}, data_length: {} }}", self.loop_flag, self.data.len())
    }
}