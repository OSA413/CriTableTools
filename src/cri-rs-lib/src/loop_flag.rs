use std::path::{Path, PathBuf};

#[derive(PartialEq, Debug, Clone)]
pub enum CriLoopFlag {
    Intro = 0,
    Loop = 1,
}

impl From<u8> for CriLoopFlag {
    fn from(value: u8) -> Self {
        match value {
            0 => CriLoopFlag::Intro,
            1 => CriLoopFlag::Loop,
            value => panic!("Invalid value for CrioopFlag: {value}"),
        }
    }
}

impl From<CriLoopFlag> for u8 {
    fn from(value: CriLoopFlag) -> Self {
        match value {
            CriLoopFlag::Intro => 0,
            CriLoopFlag::Loop => 1,
        }
    }
}

impl CriLoopFlag {
    pub fn to_file_name(&self) -> PathBuf {
        match self {
            CriLoopFlag::Intro => Path::new("Intro").to_path_buf(),
            CriLoopFlag::Loop => Path::new("Loop").to_path_buf(),
        }
    }
}