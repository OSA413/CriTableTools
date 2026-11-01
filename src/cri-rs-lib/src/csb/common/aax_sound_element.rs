use crate::{aax::archive::CriAaxArchive, sound_archive_mode::CriSoundArchiveFormat};

pub struct CriCsbAaxSoundElement {
    pub original_string_pool: Vec<String>,
    pub name: String,
    pub data: CriAaxArchive,
    pub fmt: u8,
    pub nch: u8,
    pub stmflg: u8,
    pub sfreq: u32,
    pub nsmpl: u32,
}
