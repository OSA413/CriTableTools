use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CriCsbCue {
    pub original_string_pool: Vec<String>,
    pub name: String,
    pub id: u32,
    pub synth: String,
    pub udata: String,
    pub flags: u8,
}
