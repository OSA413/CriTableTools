#[derive(PartialEq, Debug, Clone)]
pub enum CriDspLoopFlag {
    Sound = 0,
    // Currentry I found no confirmation of existance of several entries,
    // but it crashed at the early stages of the development, so I need to check if that works on all CSBs
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