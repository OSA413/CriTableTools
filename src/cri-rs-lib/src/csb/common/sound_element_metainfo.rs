use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CriCsbSoundElementMetaInfo {
    pub original_string_pool: Vec<String>,
    pub name: String,
    pub fmt: u8,
    pub nch: u8,
    pub stmflg: u8,
    pub sfreq: u32,
    pub nsmpl: u32,
}
