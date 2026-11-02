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
    pub fn read(table: &CriTable) -> Result<CriCsbAdpcmWiiArchive, CommonBinaryError> {
        let inner_table = table.get_field("data").unwrap().to_table(0).unwrap().to_owned();
        let mut inner_entries = Vec::new();
        for inner_row_index in 0..inner_table.row_count() {
            inner_entries.push(CriAdpcmWiiEntry {
                loop_flag: CriLoopFlag::from(inner_table.get_field("lpflg").unwrap().to_u8(inner_row_index).unwrap()),
                sfreq: inner_table.get_field("sfreq").unwrap().to_u32(inner_row_index).unwrap(),
                nsmpl: inner_table.get_field("nsmpl").unwrap().to_u32(inner_row_index).unwrap(),
                nch:  inner_table.get_field("nch").unwrap().to_u8(inner_row_index).unwrap(),
                data: inner_table.get_field("data").unwrap().to_data(inner_row_index).unwrap().clone(),
                header: inner_table.get_field("header").unwrap().to_data(inner_row_index).unwrap().clone(),
            });
        }

        assert_eq!(inner_table.row_count(), 1, "It is expected for ADPCM_WII table to have exactly one row");

        Ok(CriCsbAdpcmWiiArchive {
            original_string_pool: inner_table.original_string_pool.clone(),
            fmt: table.get_field("fmt").unwrap().to_u8(0).unwrap(),
            name: table.get_field("name").unwrap().to_string(0).unwrap(),
            nch: table.get_field("nch").unwrap().to_u8(0).unwrap(),
            nsmpl: table.get_field("nsmpl").unwrap().to_u32(0).unwrap(),
            sfreq: table.get_field("sfreq").unwrap().to_u32(0).unwrap(),
            stmflg: table.get_field("stmflg").unwrap().to_u8(0).unwrap(),
            entries: inner_entries,
        })
    }

    pub fn to_table(&self) -> Result<CriTable, CommonBinaryError> {
        let mut inner_table = CriTable::new(
            "ADPCM_WII",
            &self.original_string_pool,
            &[
                CriFieldDto::new("sfreq", CriFieldType::UInt32, None),
                CriFieldDto::new("nsmpl", CriFieldType::UInt32, None),
                CriFieldDto::new("nch", CriFieldType::Byte, None),
                CriFieldDto::new("lpflg", CriFieldType::Byte, None),
                CriFieldDto::new("data", CriFieldType::Data, None),
                CriFieldDto::new("header", CriFieldType::Data, None),
            ],
            self.entries.len(),
        );

        for inner_entry in &self.entries {
            inner_table.fields[0].rows.push(CriFieldValue::UInt32(inner_entry.sfreq));
            inner_table.fields[1].rows.push(CriFieldValue::UInt32(inner_entry.nsmpl));
            inner_table.fields[2].rows.push(CriFieldValue::Byte(inner_entry.nch));
            inner_table.fields[3].rows.push(CriFieldValue::Byte(inner_entry.loop_flag.clone() as u8));
            inner_table.fields[4].rows.push(CriFieldValue::Data(inner_entry.data.clone()));
            inner_table.fields[5].rows.push(CriFieldValue::Data(inner_entry.header.clone()));
        }

        Ok(inner_table)
    }
}
