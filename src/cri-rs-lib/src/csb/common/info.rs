use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CriCsbInfo { 
    pub original_string_pool: Vec<String>,
    pub data_fmt_version: u32,
    pub ext_size: u32,
}
