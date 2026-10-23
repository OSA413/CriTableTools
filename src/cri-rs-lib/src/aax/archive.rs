use common_binary::error::CommonBinaryError;
use cri_table::{header_encoding_type::CriTableHeaderEncodingType, table::CriTable};

use crate::{aax::{entry::CriAaxEntry, loop_flag::CriAaxLoopFlag}, sound_archive::{CriSoundArchive, CriSoundArchiveEnum}};

#[derive(Debug, Clone)]
pub struct CriAaxArchive {
    pub name: String,
    pub entries: Vec<CriAaxEntry>,
}

impl CriAaxArchive {
    pub fn read(table: &CriTable, name: &str) -> Result<CriSoundArchive, CommonBinaryError> {
        let mut entries = Vec::with_capacity(table.row_count());
        for row_index in 0..table.row_count() {
            let flag = CriAaxLoopFlag::from(table.get_field("lpflg").unwrap().to_u8(row_index).unwrap());
            let data = table.get_field("data").unwrap().to_data(row_index).unwrap().to_owned();
            entries.push(CriAaxEntry { flag, data });
        }

        Ok(CriSoundArchive {
            name: name.to_string(),
            data: CriSoundArchiveEnum::Aax(
                CriAaxArchive {
                    name: name.to_string(),
                    entries,
                }
            ),
        })
    }

    pub fn write(&self) -> Result<Vec<u8>, CommonBinaryError> {
        let result = CriTable {
            name: "AAX".to_string(),
            encoding: CriTableHeaderEncodingType::ShiftJis,
            fields: vec![],
            unknown_byte: 0,
        };

        

        result.write()
    }
}
