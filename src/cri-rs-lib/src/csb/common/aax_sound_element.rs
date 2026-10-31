use crate::{aax::archive::CriAaxArchive, cpk::sound::archive::CriCpkArchive, sound_archive_mode::CriSoundArchiveFormat};

#[derive(Debug)]
pub enum CriCsbAaxSoundElementData {
    Aax(CriAaxArchive),
    Stream(CriCpkArchive),
}

pub struct CriCsbAaxSoundElement {
    pub format: CriSoundArchiveFormat,
    pub name: String,
    pub data: CriCsbAaxSoundElementData,
    pub fmt: u8,
    pub nch: u8,
    pub stmflg: u8,
    pub sfreq: u32,
    pub nsmpl: u32,
}
