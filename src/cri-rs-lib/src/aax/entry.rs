use std::fmt::{self, Formatter};

use crate::aax::loop_flag::CriAaxLoopFlag;

#[derive(Clone)]
pub struct CriAaxEntry {
    pub flag: CriAaxLoopFlag,
    pub data: Vec<u8>
}

impl fmt::Debug for CriAaxEntry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "CriAaxEntry {{ flag: {:?}, data_length: {} }}", self.flag, self.data.len())
    }
}