use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CriCsbVoiceLimitGroup{ 
    pub original_string_pool: Vec<String>,
    pub vlgname: String,
    pub vlgnvoice: u32,
}
