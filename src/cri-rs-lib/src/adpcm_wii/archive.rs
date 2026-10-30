use common_binary::error::CommonBinaryError;
use cri_table::{field::CriTableField, field_type::CriFieldType, field_value::CriFieldValue, encoding_type::CriTableEncodingType, table::CriTable};

use crate::{adpcm_wii::entry::CriAdpcmWiiEntry, loop_flag::CriLoopFlag, sound_archive_mode::CriSoundArchiveFormat};

#[derive(Debug)]
pub struct CriAdpcmWiiArchiveWrapper {
    pub original_string_pool: Vec<String>,
    pub entries: Vec<CriAdpcmWiiArchive>,
}

impl CriAdpcmWiiArchiveWrapper {
    pub fn from_table(table: &CriTable) -> Result<CriAdpcmWiiArchiveWrapper, CommonBinaryError> {
        CriAdpcmWiiArchive::read(table)
    }
}

#[derive(Debug, Clone)]
pub struct CriAdpcmWiiArchive {
    pub format: CriSoundArchiveFormat,
    pub original_string_pool: Vec<String>,
    pub fmt: u8,
    pub name: String,
    pub nch: u8,
    pub nsmpl: u32,
    pub sfreq: u32,
    pub stmflg: u8,
    pub entries: Vec<CriAdpcmWiiEntry>,
}

impl CriAdpcmWiiArchive {
    pub fn read(table: &CriTable) -> Result<CriAdpcmWiiArchiveWrapper, CommonBinaryError> {
        let mut entries = Vec::with_capacity(table.row_count());
        for row_index in 0..table.row_count() {
            let inner_table = table.get_field("data").unwrap().to_table(row_index).unwrap().to_owned();
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

            entries.push(CriAdpcmWiiArchive {
                format: CriSoundArchiveFormat::Dsp,
                original_string_pool: inner_table.original_string_pool.clone(),
                fmt: table.get_field("fmt").unwrap().to_u8(row_index).unwrap(),
                name: table.get_field("name").unwrap().to_string(row_index).unwrap(),
                nch: table.get_field("nch").unwrap().to_u8(row_index).unwrap(),
                nsmpl: table.get_field("nsmpl").unwrap().to_u32(row_index).unwrap(),
                sfreq: table.get_field("sfreq").unwrap().to_u32(row_index).unwrap(),
                stmflg: table.get_field("stmflg").unwrap().to_u8(row_index).unwrap(),
                entries: inner_entries,
            });
        }

        Ok(CriAdpcmWiiArchiveWrapper {
            original_string_pool: table.original_string_pool.clone(),
            entries,
        })
    }

    pub fn to_table(&self) -> Result<CriTable, CommonBinaryError> {
        let mut inner_table = CriTable {
            original_string_pool: self.original_string_pool.clone(),
            name: "ADPCM_WII".to_string(),
            encoding: CriTableEncodingType::ShiftJis,
            unknown_byte: 0,
            fields: vec![
                CriTableField {
                    ttype: CriFieldType::UInt32,
                    name: "sfreq".to_string(),
                    rows: vec![],
                    default_value: None,
                },
                CriTableField {
                    ttype: CriFieldType::UInt32,
                    name: "nsmpl".to_string(),
                    rows: vec![],
                    default_value: None,
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    name: "nch".to_string(),
                    rows: vec![],
                    default_value: None,
                },
                CriTableField {
                    ttype: CriFieldType::Byte,
                    name: "lpflg".to_string(),
                    rows: vec![],
                    default_value: None,
                },
                CriTableField {
                    ttype: CriFieldType::Data,
                    name: "data".to_string(),
                    rows: vec![],
                    default_value: None,
                },
                CriTableField {
                    ttype: CriFieldType::Data,
                    name: "header".to_string(),
                    rows: vec![],
                    default_value: None,
                },
            ],
        };

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
