use std::fmt::{self, Formatter};

use crate::adpcm_wii::loop_flag::CriDspLoopFlag;

pub struct CriAdpcmWiiEntry {
    pub sfreq: u32,
    pub nsmpl: u32,
    pub nch: u8,
    pub loop_flag: CriDspLoopFlag,
    pub data: Vec<u8>,
    pub header: Vec<u8>,
}



impl fmt::Debug for CriAdpcmWiiEntry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "CriAdpcmWiiEntry {{ flag: {:?}, data_length: {} }}", self.loop_flag, self.data.len())
    }
}