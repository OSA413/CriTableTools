use common_binary::error::CommonBinaryError;
use cri_table::table::CriTable;

use crate::aax::{archive_mode::CriAaxArchiveMode, entry::CriAaxEntry, entry_flag::CriAaxEntryFlag};

pub struct CriAaxArchive {
    pub mode: CriAaxArchiveMode,
    pub entries: Vec<CriAaxEntry>,
}

impl CriAaxArchive {
    pub fn read(table: CriTable) -> Result<CriAaxArchive, CommonBinaryError> {
        let mode = match table.name.as_str() {
            "AAX" => CriAaxArchiveMode::Adx,
            "SWLPCM" => CriAaxArchiveMode::Wav,
            "ADPCM_WII" => {
                eprintln!("DSP files are not supported.");
                return Err(CommonBinaryError::SeeConsole());
            },
            value => {
                eprintln!("Unknown AAX type [{value}]");
                return Err(CommonBinaryError::SeeConsole());
            },
        };

        let mut entries = Vec::with_capacity(table.row_count());
        for row_index in 0..table.row_count() {
            let flag = CriAaxEntryFlag::from(table.get_field("lpflg").unwrap().to_u8(row_index).unwrap());
            let data = table.get_field("data").unwrap().to_data(row_index).unwrap().to_owned();
            entries.push(CriAaxEntry { flag, data });
        }

        Ok(CriAaxArchive {
            mode,
            entries,
        })
    }

    // In the AaxArchive there can be only two entries as maximum: Intro and/or Loop
    // Maybe redo simpler?
    // TODO: move to csb-rs-lib
    // pub fn add(&mut self, item: CriAaxEntry) {
    //     if self.entries.len() == 2 || self.entries.iter().find(|entry| entry.flag == item.flag).is_some() {
    //         return;
    //     }
    //     self.entries.push(item)
    // }
}
