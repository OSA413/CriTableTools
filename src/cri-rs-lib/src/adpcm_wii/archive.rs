use common_binary::error::CommonBinaryError;
use cri_table::{field_dto::CriFieldDto, field_type::CriFieldType, field_value::CriFieldValue, table::CriTable};

use crate::{adpcm_wii::entry::CriAdpcmWiiEntry, loop_flag::CriLoopFlag};

#[derive(Debug, Clone)]
pub struct CriCsbAdpcmWiiArchive {
    pub original_string_pool: Vec<String>,
    pub name: String,
    pub fmt: u8,
    pub nch: u8,
    pub nsmpl: u32,
    pub sfreq: u32,
    pub stmflg: u8,
    pub entries: Vec<CriAdpcmWiiEntry>,
}

impl CriCsbAdpcmWiiArchive {
    pub fn read(inner_table: &CriTable) -> Result<CriAdpcmWiiEntry, CommonBinaryError> {
        assert_eq!(inner_table.row_count(), 1, "It is expected for ADPCM_WII table to have exactly one row");
        Ok(CriAdpcmWiiEntry {
            loop_flag: CriLoopFlag::from(inner_table.get_field("lpflg").unwrap().to_u8(0).unwrap()),
            sfreq: inner_table.get_field("sfreq").unwrap().to_u32(0).unwrap(),
            nsmpl: inner_table.get_field("nsmpl").unwrap().to_u32(0).unwrap(),
            nch:  inner_table.get_field("nch").unwrap().to_u8(0).unwrap(),
            data: inner_table.get_field("data").unwrap().to_data(0).unwrap().clone(),
            header: inner_table.get_field("header").unwrap().to_data(0).unwrap().clone(),
        })
    }
}
