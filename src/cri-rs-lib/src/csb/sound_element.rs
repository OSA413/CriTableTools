pub mod adpcm_wii_sound_element;
pub mod sound_element_metainfo;
pub mod aax_sound_element;

use crate::sound_archive::CriCsbSoundArchiveEnum;


pub struct CriCsbSoundElement {
    pub original_string_pool: Vec<String>,
    pub name: String,
    pub data: CriCsbSoundArchiveEnum,
    pub fmt: u8,
    pub nch: u8,
    pub stmflg: u8,
    pub sfreq: u32,
    pub nsmpl: u32,
}