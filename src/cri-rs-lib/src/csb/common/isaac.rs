use serde::{Deserialize, Serialize};

use crate::graph::CriGraphWrapper;


#[derive(Serialize, Deserialize, Debug)]
pub struct CriCsbIsaac {
    pub original_string_pool: Vec<String>,
    pub name: String,
    pub ptname: String,
    pub ttype: u8,
    pub grph: CriGraphWrapper,
    pub rndrng: u8,
}
