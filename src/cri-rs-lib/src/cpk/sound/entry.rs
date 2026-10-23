use chrono::{NaiveDateTime};

use crate::{aax::archive::CriAaxArchive, sound_archive::CriSoundArchive};

#[derive(Debug)]
pub struct CriCpkEntry {
    pub update_date_time: NaiveDateTime,
    pub directory_name: String,
    pub id: u32,
    pub comment: String,
    pub aax_archive: CriSoundArchive,
}
