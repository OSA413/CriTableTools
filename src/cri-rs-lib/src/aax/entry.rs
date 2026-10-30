use std::fmt::{self, Formatter};

use crate::loop_flag::CriLoopFlag;

#[derive(Clone)]
pub struct CriAaxEntry {
    pub flag: CriLoopFlag,
    pub data: Vec<u8>
}

impl fmt::Debug for CriAaxEntry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "CriAaxEntry {{ flag: {:?}, data_length: {} }}", self.flag, self.data.len())
    }
}