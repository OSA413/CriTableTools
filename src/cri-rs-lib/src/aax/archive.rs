use common_binary::error::CommonBinaryError;
use cri_table::{field_dto::CriFieldDto, field_type::CriFieldType, field_value::CriFieldValue, table::CriTable};

use crate::{aax::entry::CriAaxEntry, loop_flag::CriLoopFlag, sound_archive::{CriSoundArchive, CriSoundArchiveEnum}};

#[derive(Debug, Clone)]
pub struct CriAaxArchive {
    pub name: String,
    pub entries: Vec<CriAaxEntry>,
}

impl CriAaxArchive {
    pub fn read(table: &CriTable, name: &str) -> Result<CriSoundArchive, CommonBinaryError> {
        let mut entries = Vec::with_capacity(table.row_count());
        for row_index in 0..table.row_count() {
            let flag = CriLoopFlag::from(table.get_field("lpflg").unwrap().to_u8(row_index).unwrap());
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
        let mut result = CriTable::new(
            "AAX",
            &vec![], //TODO
            &[
                CriFieldDto::new("data", CriFieldType::Data, None),
                CriFieldDto::new("lpflg", CriFieldType::Byte, None),
            ],
            self.entries.len(),
        );

        if let Some(intro) = self.entries.iter().find(|entry| entry.flag == CriLoopFlag::Intro) {
            result.fields[0].rows.push(CriFieldValue::Data(intro.data.clone()));
            result.fields[1].rows.push(CriFieldValue::Byte(intro.flag.clone() as u8));
        };

        if let Some(r#loop) = self.entries.iter().find(|entry| entry.flag == CriLoopFlag::Loop) {
            result.fields[0].rows.push(CriFieldValue::Data(r#loop.data.clone()));
            result.fields[1].rows.push(CriFieldValue::Byte(r#loop.flag.clone() as u8));
        };

        result.write()
    }
}
