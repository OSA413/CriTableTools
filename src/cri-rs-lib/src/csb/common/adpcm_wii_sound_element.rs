use crate::adpcm_wii::archive::CriAdpcmWiiArchiveWrapper;

#[derive(Debug)]
pub enum CriCsbAdpcmWiiSoundElementData {
    AdpcmWii(CriAdpcmWiiArchiveWrapper),
}

pub struct CriCsbAdpcmWiiSoundElement {
    pub original_string_pool: Vec<String>,
    pub name: String,
    pub data: CriCsbAdpcmWiiSoundElementData,
    pub fmt: u8,
    pub nch: u8,
    pub stmflg: u8,
    pub sfreq: u32,
    pub nsmpl: u32,
}