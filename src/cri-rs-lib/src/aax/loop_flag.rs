#[derive(PartialEq, Debug, Clone)]
pub enum CriAaxLoopFlag {
    Intro = 0,
    Loop = 1,
}

impl From<u8> for CriAaxLoopFlag {
    fn from(value: u8) -> Self {
        match value {
            0 => CriAaxLoopFlag::Intro,
            1 => CriAaxLoopFlag::Loop,
            value => panic!("Invalid value for CriAaxEntryFlag: {value}"),
        }
    }
}

impl From<CriAaxLoopFlag> for u8 {
    fn from(value: CriAaxLoopFlag) -> Self {
        match value {
            CriAaxLoopFlag::Intro => 0,
            CriAaxLoopFlag::Loop => 1,
        }
    }
}

impl CriAaxLoopFlag {
    pub fn to_file_name(&self) -> &'static str {
        match self {
            CriAaxLoopFlag::Intro => "Intro.adx",
            CriAaxLoopFlag::Loop => "Loop.adx",
        }
    }
}