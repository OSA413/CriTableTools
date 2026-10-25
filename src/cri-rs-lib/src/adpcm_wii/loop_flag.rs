#[derive(PartialEq, Debug, Clone)]
pub enum CriDspLoopFlag {
    Intro = 0,
    Loop = 1,
}

impl From<u8> for CriDspLoopFlag {
    fn from(value: u8) -> Self {
        match value {
            0 => CriDspLoopFlag::Intro,
            1 => CriDspLoopFlag::Loop,
            value => panic!("Invalid value for CriDspLoopFlag: {value}"),
        }
    }
}

impl From<CriDspLoopFlag> for u8 {
    fn from(value: CriDspLoopFlag) -> Self {
        match value {
            CriDspLoopFlag::Intro => 0,
            CriDspLoopFlag::Loop => 1,
        }
    }
}

impl CriDspLoopFlag {
    pub fn to_file_name(&self) -> &'static str {
        match self {
            CriDspLoopFlag::Intro => "Intro.dsp",
            CriDspLoopFlag::Loop => "Loop.dsp",
        }
    }
}