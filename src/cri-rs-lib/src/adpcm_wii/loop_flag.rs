#[derive(PartialEq, Debug, Clone)]
pub enum CriDspLoopFlag {
    Sound = 0,
    SoundLoop = 1,
}

impl From<u8> for CriDspLoopFlag {
    fn from(value: u8) -> Self {
        match value {
            0 => CriDspLoopFlag::Sound,
            1 => CriDspLoopFlag::SoundLoop,
            value => panic!("Invalid value for CriDspLoopFlag: {value}"),
        }
    }
}

impl From<CriDspLoopFlag> for u8 {
    fn from(value: CriDspLoopFlag) -> Self {
        match value {
            CriDspLoopFlag::Sound => 0,
            CriDspLoopFlag::SoundLoop => 1,
        }
    }
}

impl CriDspLoopFlag {
    pub fn to_file_name(&self) -> &'static str {
        match self {
            CriDspLoopFlag::Sound => "Sound.dsp",
            CriDspLoopFlag::SoundLoop => "SoundLoop.dsp",
        }
    }
}